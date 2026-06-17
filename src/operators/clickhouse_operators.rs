use chrono::Utc;
use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};

use crate::errors::CLIError;

use super::migrations_operators::MigrationOnDisk;

#[derive(Row, Serialize, Deserialize, Debug, Clone)]
pub struct MigrationRow {
    pub version: String,
    pub ran_at: u32,
}

pub async fn create_migrations_table(client: clickhouse::Client) -> Result<(), CLIError> {
    client
        .query(
            "
            CREATE TABLE IF NOT EXISTS ch_migrations (
                version String,
                ran_at DateTime,
                ) ENGINE = MergeTree()
            ORDER BY(ran_at, version)
            ",
        )
        .execute()
        .await
        .map_err(|e| e.into())
}

pub async fn check_if_migrations_table_exists(
    client: clickhouse::Client,
) -> Result<bool, CLIError> {
    let table_exists = client
        .query(
            "
            SELECT name FROM system.tables WHERE name = 'ch_migrations'
            ",
        )
        .fetch_all::<String>()
        .await?;

    Ok(!table_exists.is_empty())
}

pub async fn get_clickhouse_client_and_ping() -> Result<Client, CLIError> {
    let url = std::env::var("CLICKHOUSE_URL")
        .map_err(|_| CLIError::BadArgs("Missing CLICKHOUSE_URL env var".to_string()))?;
    let database = std::env::var("CLICKHOUSE_DB")
        .map_err(|_| CLIError::BadArgs("Missing CLICKHOUSE_DB env var".to_string()))?;

    let mut client = Client::default().with_url(url).with_database(database);

    if let Ok(user) = std::env::var("CLICKHOUSE_USER") {
        client = client.with_user(user);
    }

    if let Ok(password) = std::env::var("CLICKHOUSE_PASSWORD") {
        client = client.with_password(password);
    }

    client.query("SELECT 1").execute().await?;

    Ok(client)
}

pub async fn get_migrations_from_clickhouse(
    client: clickhouse::Client,
) -> Result<Vec<MigrationRow>, CLIError> {
    let migrations = client
        .query(
            "
        SELECT ?fields FROM ch_migrations ORDER BY ran_at
        ",
        )
        .fetch_all::<MigrationRow>()
        .await?;
    Ok(migrations)
}

pub async fn apply_migrations(
    client: clickhouse::Client,
    migrations: Vec<MigrationOnDisk>,
) -> Result<(), CLIError> {
    for migration in &migrations {
        let mut insert = client.insert::<MigrationRow>("ch_migrations")?;

        insert
            .write(&MigrationRow {
                ran_at: Utc::now().timestamp() as u32,
                version: migration.version.clone(),
            })
            .await?;

        let up_query = migration.get_up_query().await?;
        let queries = up_query
            .split(';')
            .filter(|s| {
                !s.is_empty()
                    && !s.contains("--")
                    && !s.chars().all(|c| c.is_whitespace() || c == '\n')
            })
            .collect::<Vec<&str>>();

        println!("Running migration {}", migration.name);

        for query in queries {
            client.query(query).execute().await?;
        }

        insert.end().await?;
    }
    Ok(())
}

pub async fn undo_migration(
    client: clickhouse::Client,
    migration: MigrationOnDisk,
) -> Result<(), CLIError> {
    let down_query = migration.get_down_query().await?;
    let queries = down_query
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    println!("Reverting migration {}", migration.name);

    for query in queries {
        client.query(query).execute().await?;
    }

    client
        .query(
            format!(
                "DELETE FROM ch_migrations WHERE version = '{}'",
                migration.version
            )
            .as_str(),
        )
        .execute()
        .await?;

    Ok(())
}

pub async fn get_last_migration_from_clickhouse(
    client: clickhouse::Client,
) -> Result<Option<MigrationRow>, CLIError> {
    let mut rows = client
        .query("SELECT ?fields FROM ch_migrations ORDER BY ran_at DESC, version DESC LIMIT 1")
        .fetch_all::<MigrationRow>()
        .await?;

    Ok(rows.pop())
}

use chrono::Utc;
use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};

use crate::{errors::CLIError, tools::migrations::SetupArgs};

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

pub async fn get_clickhouse_client_and_ping(args: SetupArgs) -> Result<Client, CLIError> {
    let mut client = Client::default().with_url(
        args.url
            .ok_or(CLIError::BadArgs("Missing URL".to_string()))?,
    );

    if let Some(user) = args.user {
        client = client.with_user(user);
    }

    if let Some(password) = args.password {
        client = client.with_password(password);
    }

    if let Some(database) = args.database {
        client = client.with_database(database);
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

        println!("Running query {:?}", queries);

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
    let mut queries = down_query
        .split(';')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    queries.truncate(queries.len().saturating_sub(1));

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

pub async fn ensure_migrations_sync(
    local_migrations: Vec<MigrationOnDisk>,
    applied_migrations: Vec<MigrationRow>,
) -> Result<(), CLIError> {
    let db_migrations_not_in_local: Vec<MigrationRow> = applied_migrations
        .iter()
        .filter_map(|applied_migration| {
            if local_migrations
                .iter()
                .any(|lm| lm.version == applied_migration.version)
            {
                return None;
            }
            Some(applied_migration.clone())
        })
        .collect();

    if !db_migrations_not_in_local.is_empty() {
        return Err(CLIError::BadArgs(
            "Your local migrations and the database migrations are out of sync!".to_string(),
        ));
    }

    Ok(())
}

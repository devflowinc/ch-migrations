use chrono::Utc;
use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};

use crate::{commands::setup::RequiredSetupArgs, errors::CLIError};

use super::migrations_operators::MigrationOnDisk;

#[derive(Row, Serialize, Deserialize, Debug, Clone)]
pub struct MigrationRow {
    pub version: String,
    pub ran_at: i64,
}

pub async fn create_migrations_table_if_exists(client: clickhouse::Client) -> Result<(), CLIError> {
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

pub async fn drop_migrations_table_if_exists(client: clickhouse::Client) -> Result<(), CLIError> {
    client
        .query("DROP TABLE IF EXISTS ch_migrations")
        .execute()
        .await
        .map_err(|e| e.into())
}

pub async fn get_clickhouse_client_and_ping(args: RequiredSetupArgs) -> Result<Client, CLIError> {
    let client = Client::default()
        .with_url(args.url)
        .with_user(args.user)
        .with_password(args.password)
        .with_database(args.database)
        .with_option("async_insert", "1")
        .with_option("wait_for_async_insert", "0");

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

pub async fn apply_migrations_from_local(
    client: clickhouse::Client,
    migrations: Vec<MigrationOnDisk>,
) -> Result<(), CLIError> {
    let mut insert = client.insert::<MigrationRow>("ch_migrations")?;
    for migration in &migrations {
        insert
            .write(&MigrationRow {
                ran_at: Utc::now().timestamp(),
                version: migration.timestamp.format("%Y-%m-%d-%H%M%S").to_string(),
            })
            .await?;
        client.query(&migration.up_query).execute().await?;
    }
    Ok(())
}

use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};

use crate::{commands::setup::RequiredSetupArgs, errors::CLIError};

#[derive(Row, Serialize, Deserialize, Debug)]
pub struct MigrationRow {
    version: String,
    ran_at: Option<u32>,
}

pub async fn create_migrations_table_if_exists(client: clickhouse::Client) -> Result<(), CLIError> {
    client
        .query(
            "
            CREATE TABLE IF NOT EXISTS ch_migrations (
                version String,
                ran_at Nullable(DateTime),
                PRIMARY KEY(version),
                ) ENGINE = MergeTree()
            ",
        )
        .execute()
        .await
        .map_err(|e| e.into())
}

pub async fn drop_migrations_table_if_exists(client: clickhouse::Client) -> Result<(), CLIError> {
    client
        .query(
            "DROP TABLE IF EXISTS ch_migrations"
        )
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
        SELECT ?fields FROM ch_migrations ORDER BY ran_at NULLS LAST
        ",
        )
        .fetch_all::<MigrationRow>()
        .await?;
    Ok(migrations)
}

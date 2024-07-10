use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{commands::setup::RequiredSetupArgs, errors::CLIError};

#[derive(Row, Serialize, Deserialize)]
pub struct MigrationRow {
    version: String,
    #[serde(with = "clickhouse::serde::time::datetime64::secs")]
    ran_at: OffsetDateTime,
}

pub async fn create_migrations_table_if_exists(client: clickhouse::Client) -> Result<(), CLIError> {
    client
        .query(
            "
            CREATE TABLE IF NOT EXISTS ch_migrations (
                version String,
                ran_at Datetime
                ) ENGINE = MergeTree()
            ",
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

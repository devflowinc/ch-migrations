use crate::{
    errors::CLIError,
    operators::{clickhouse_operators::{
        create_migrations_table_if_exists, drop_migrations_table_if_exists,
        get_clickhouse_client_and_ping, get_migrations_from_clickhouse,
    }, migrations_operators::get_migrations_from_dir},
};

use super::setup::RequiredSetupArgs;

pub async fn run_command() -> Result<(), CLIError> {
    let config = RequiredSetupArgs::from_toml_file().await?;
    let client = get_clickhouse_client_and_ping(config).await?;

    create_migrations_table_if_exists(client.clone()).await?;

    let local_migrations = get_migrations_from_dir().await?;
    let db_migrations = get_migrations_from_clickhouse(client.clone()).await?;

    Ok(())
}

use crate::{
    errors::CLIError,
    operators::clickhouse_operators::{get_clickhouse_client_and_ping, run_pending_migrations},
    SetupArgs,
};

pub async fn run_command() -> Result<(), CLIError> {
    let config = SetupArgs::from_toml_file().await?;
    let client = get_clickhouse_client_and_ping(config).await?;

    run_pending_migrations(client).await?;

    Ok(())
}

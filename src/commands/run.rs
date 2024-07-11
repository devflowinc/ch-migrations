use crate::{
    commands::SetupArgs, errors::CLIError,
    operators::clickhouse_operators::get_clickhouse_client_and_ping,
    tools::migrations::run_pending_migrations,
};

pub async fn run_command() -> Result<(), CLIError> {
    let config = SetupArgs::from_toml_file().await?;
    let client = get_clickhouse_client_and_ping(config).await?;

    run_pending_migrations(client).await?;

    Ok(())
}

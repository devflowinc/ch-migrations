use crate::{
    errors::CLIError,
    tools::migrations::{run_pending_migrations, SetupArgs},
};

pub async fn run_command() -> Result<(), CLIError> {
    let config = SetupArgs::from_toml_file().await?;

    run_pending_migrations(config).await?;

    Ok(())
}

use crate::{errors::CLIError, tools::migrations::run_pending_migrations};
use std::path::Path;

pub async fn run_command(source: &Path) -> Result<(), CLIError> {
    run_pending_migrations(source).await?;

    Ok(())
}

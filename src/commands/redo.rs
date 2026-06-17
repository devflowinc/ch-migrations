use crate::errors::CLIError;

use super::{revert::revert_commmand, run::run_command};
use std::path::Path;

pub async fn redo_command(source: &Path) -> Result<(), CLIError> {
    revert_commmand(source).await?;
    run_command(source).await?;

    Ok(())
}

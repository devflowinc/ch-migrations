use crate::errors::CLIError;

use super::{revert::revert_commmand, run::run_command};

pub async fn redo_command() -> Result<(), CLIError> {
    revert_commmand().await?;
    run_command().await?;

    Ok(())
}

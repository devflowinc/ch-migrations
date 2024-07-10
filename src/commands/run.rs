use crate::{
    errors::CLIError,
    operators::clickhouse_operators::{client_factory, migrations_table_exists},
};

use super::setup::RequiredSetupArgs;

pub async fn run_command() -> Result<(), CLIError> {
    let config = RequiredSetupArgs::from_toml_file().await?;
    let client = client_factory(config).await;
    let exists = migrations_table_exists(client).await?;
    println!("exists: {:?}", exists);
    Ok(())
}

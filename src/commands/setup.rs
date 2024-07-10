use crate::{errors::CLIError, operators::clickhouse_operators::get_clickhouse_client_and_ping, SetupArgs};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequiredSetupArgs {
    pub url: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl RequiredSetupArgs {
    pub fn from_setup_args(args: SetupArgs) -> Result<Self, CLIError> {
        if args.user.is_none()
            || args.password.is_none()
            || args.database.is_none()
            || args.url.is_none()
        {
            return Err(CLIError::BadArgs("All arguments not provided".to_string()));
        }

        Ok(Self {
            url: args.url.unwrap(),
            user: args.user.unwrap(),
            password: args.password.unwrap(),
            database: args.database.unwrap(),
        })
    }

    pub async fn from_toml_file() -> Result<Self, CLIError> {
        let toml_file_path = std::env::current_dir()?.join("chm.toml");
        let toml_data = tokio::fs::read_to_string(toml_file_path).await?;

        toml::from_str::<RequiredSetupArgs>(toml_data.as_str())
            .map_err(|_| CLIError::InternalError("Failed to get args from toml file".to_string()))
    }
}

pub async fn setup_command(args: SetupArgs) -> Result<(), CLIError> {
    let migrations_dir = std::env::current_dir()?.join("migrations");

    if migrations_dir.is_dir() {
        return Err(CLIError::BadArgs("migrations already exists".to_string()));
    }

    // if any cli arg is given, use it instead
    let args = if args.user.is_some()
        || args.database.is_some()
        || args.url.is_some()
        || args.password.is_some()
    {
        RequiredSetupArgs::from_setup_args(args)
    } else {
        RequiredSetupArgs::from_setup_args(SetupArgs::from_envs())
    }?;

    get_clickhouse_client_and_ping(args.clone()).await?;

    let toml_config_file = std::env::current_dir()?.join("chm.toml");

    if toml_config_file.is_dir() {
        return Err(CLIError::BadArgs(
            "directory migrations already exists".to_string(),
        ));
    }

    let mut toml_file = tokio::fs::File::create(toml_config_file).await?;

    let toml_data = toml::to_string(&args)
        .map_err(|_| CLIError::InternalError("Failed to write to toml file".to_string()))?;

    toml_file.write(toml_data.as_bytes()).await?;

    tokio::fs::create_dir(migrations_dir).await?;

    Ok(())
}

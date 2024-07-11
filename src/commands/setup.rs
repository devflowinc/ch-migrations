use crate::{
    commands::SetupArgs, errors::CLIError,
    operators::clickhouse_operators::get_clickhouse_client_and_ping,
};
use tokio::io::AsyncWriteExt;

impl SetupArgs {
    pub async fn from_toml_file() -> Result<Self, CLIError> {
        let toml_file_path = std::env::current_dir()?.join("ch_migrations/chm.toml");
        let toml_data = tokio::fs::read_to_string(toml_file_path).await?;

        toml::from_str::<SetupArgs>(toml_data.as_str())
            .map_err(|_| CLIError::InternalError("Failed to get args from toml file".to_string()))
    }

    pub async fn save_to_toml_file(&self) -> Result<(), CLIError> {
        let toml_config_file = std::env::current_dir()?.join("ch_migrations/chm.toml");

        if toml_config_file.is_dir() {
            return Err(CLIError::BadArgs(
                "directory migrations already exists".to_string(),
            ));
        }

        let mut toml_file = tokio::fs::File::create(toml_config_file).await?;

        let toml_data = toml::to_string(self)
            .map_err(|_| CLIError::InternalError("Failed to write to toml file".to_string()))?;

        toml_file.write(toml_data.as_bytes()).await?;

        Ok(())
    }
}

pub async fn setup_command(args: SetupArgs) -> Result<(), CLIError> {
    get_clickhouse_client_and_ping(args.clone()).await?;

    let migrations_dir = std::env::current_dir()?.join("ch_migrations");

    if migrations_dir.is_dir() {
        return Err(CLIError::BadArgs("migrations already exists".to_string()));
    }

    tokio::fs::create_dir(migrations_dir).await?;

    args.save_to_toml_file().await?;

    Ok(())
}

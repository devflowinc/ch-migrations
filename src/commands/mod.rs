use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

pub mod generate;
pub mod redo;
pub mod revert;
pub mod run;
pub mod setup;

#[derive(Parser)]
#[command(author, version)]
#[command(name = "chm", about = "Clickhouse migration tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a folder to contain migrations and a .toml file with connection details, will error
    /// if migrations folder already exists.
    Setup(SetupArgs),
    /// Commands to mutate migrations
    #[command(subcommand)]
    Migration(MigrationCommands),
}

#[derive(Subcommand)]
pub enum MigrationCommands {
    Generate(GenerateArgs),
    /// Command to first identify pending migrations and run the new up migrations
    Run,
    /// Command to revert and then apply the latest migration
    Redo,
    /// Command to revert last migration
    Revert,
}

#[derive(Args, Clone, Deserialize, Serialize)]
pub struct SetupArgs {
    /// Clickhouse URL
    #[arg(env = "CLICKHOUSE_URL", default_value = None)]
    pub url: Option<String>,
    /// Clickhouse User
    #[arg(env = "CLICKHOUSE_USER", default_value = None)]
    pub user: Option<String>,
    /// Clickhouse Password
    #[arg(env = "CLICKHOUSE_PASSWORD", default_value = None)]
    pub password: Option<String>,
    /// Clickhouse Database
    #[arg(env = "CLICKHOUSE_DB", default_value = None)]
    pub database: Option<String>,
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    /// Name of the migration to be generated
    pub name: String,
}

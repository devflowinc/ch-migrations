use clap::{Args, Parser, Subcommand};
use commands::{
    generate::generate_command, redo::redo_command, revert::revert_commmand, run::run_command,
    setup::setup_command,
};
use errors::CLIError;
use serde::{Deserialize, Serialize};
mod commands;
mod errors;
mod operators;

#[derive(Parser)]
#[command(author, version)]
#[command(name = "chm", about = "Clickhouse migration tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
struct SetupArgs {
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

#[tokio::main]
async fn main() {
    let err = dotenvy::dotenv();

    if let Err(e) = err {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    let args = Cli::parse();

    let res: Result<(), CLIError> = match args.command {
        Commands::Setup(args) => setup_command(args).await,
        Commands::Migration(commands) => match commands {
            MigrationCommands::Generate(args) => generate_command(args).await,
            MigrationCommands::Run => run_command().await,
            MigrationCommands::Redo => redo_command().await,
            MigrationCommands::Revert => revert_commmand().await,
        },
    };

    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

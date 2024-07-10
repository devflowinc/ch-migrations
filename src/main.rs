use clap::{Args, Parser, Subcommand};
use commands::setup::setup_command;
use errors::CLIError;
mod commands;
mod errors;

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
    /// Command to generate a sub folder with up and down migrations
    #[command(subcommand)]
    Generate,
    /// Command to first identify pending migrations and run the new up migrations
    #[command(subcommand)]
    Run,
    /// Command to revert and then apply the latest migration
    #[command(subcommand)]
    Redo,
    /// Command to revert last migration
    #[command(subcommand)]
    Revert,
}

#[derive(Args, Clone)]
struct SetupArgs {
    pub url: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
}

impl SetupArgs {
    pub fn from_envs() -> Self {
        let url = std::env::var("CLICKHOUSE_URL").ok();
        let user = std::env::var("CLICKHOUSE_USER").ok();
        let database = std::env::var("CLICKHOUSE_DB").ok();
        let password = std::env::var("CLICKHOUSE_PASSWORD").ok();
        Self {
            url,
            user,
            database,
            password,
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let res: Result<(), CLIError> = match args.command {
        Commands::Setup(args) => setup_command(args).await,
    };

    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

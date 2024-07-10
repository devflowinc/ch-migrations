use clap::{Args, Parser, Subcommand};
use commands::{generate::generate_command, run::run_command, setup::setup_command};
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
    Generate(GenerateArgs),
    /// Command to first identify pending migrations and run the new up migrations
    Run,
    /// Command to revert and then apply the latest migration
    Redo,
    /// Command to revert last migration
    Revert,
}

#[derive(Args, Clone)]
struct SetupArgs {
    /// Clickhouse URL
    pub url: Option<String>,
    /// Clickhouse User
    pub user: Option<String>,
    /// Clickhouse Password
    pub password: Option<String>,
    /// Clickhouse Database
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

#[derive(Args, Debug)]
struct GenerateArgs {
    /// Name of the migration to be generated
    pub name: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let res: Result<(), CLIError> = match args.command {
        Commands::Setup(args) => setup_command(args).await,
        Commands::Run => run_command().await,
        Commands::Generate(args) => generate_command(args).await,
        _ => Err(CLIError::NotImplemented),
    };

    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

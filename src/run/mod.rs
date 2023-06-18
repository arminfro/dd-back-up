pub mod backup;
mod config;
pub mod utils;

use clap::{Parser, Subcommand};

use self::{
    backup::{run as backup_run, BackupArgs},
    config::Config,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform the backups
    Run(BackupArgs),
}

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(backup_args) => {
            let config = Config::new(&backup_args.config_file_path)
                .map_err(|e| format!("Failed to create Config struct object: {}", e))?;
            backup_run(backup_args, &config).map_err(|e| format!("Failed to run backups: {}", e))
        }
    }
}

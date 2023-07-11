pub mod backup;
mod config;
pub mod utils;

use clap::{Parser, Subcommand};

use self::backup::{run as backup_run, BackupArgs};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Perform the backups
    Run(BackupArgs),
}

/// Runs the backup process.
///
/// This function is responsible for parsing the command line arguments and executing the backup process.
///
/// # Errors
///
/// Returns an error if the backup process fails to run.
pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    trace!("CLI command matching {:?}", &cli.command);
    match &cli.command {
        Commands::Run(backup_args) => {
            backup_run(backup_args).map_err(|e| format!("Failed to run backups: {}", e))
        }
    }
}

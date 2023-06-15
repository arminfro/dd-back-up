pub mod back_up;
mod config;
pub mod utils;

use clap::{Parser, Subcommand};

use self::{
    back_up::{run as back_up_run, BackUpArgs},
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
    BackUp(BackUpArgs),
}

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();
    let config =
        Config::new().map_err(|e| format!("Failed to create Config struct object: {}", e))?;

    match &cli.command {
        Commands::BackUp(args) => {
            back_up_run(args, config).map_err(|e| format!("Failed to run backups: {}", e))
        }
    }
}

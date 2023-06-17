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
    /// Perform the backups
    Run(BackUpArgs),
}

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(back_up_args) => {
            let config = Config::new(&back_up_args.config_file_path)
                .map_err(|e| format!("Failed to create Config struct object: {}", e))?;
            back_up_run(back_up_args, &config).map_err(|e| format!("Failed to run backups: {}", e))
        }
    }
}

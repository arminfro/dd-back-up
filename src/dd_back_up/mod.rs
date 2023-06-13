pub mod back_up;
mod config;

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
    let config = Config::new().expect("Failed to create Config struct object");

    match &cli.command {
        Commands::BackUp(args) => back_up_run(args, config),
    }
}

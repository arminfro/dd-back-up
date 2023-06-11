mod back_up;

use clap::{Parser, Subcommand};

use self::back_up::{run as back_up_run, BackUpArgs};

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

    match &cli.command {
        Commands::BackUp(args) => back_up_run(args),
    }
}

pub mod back_up_config;

use clap::Args;

use super::config::Config;

#[derive(Args, Debug)]
pub struct BackUpArgs {}

pub fn run(back_up_args: &BackUpArgs, config: Config) -> Result<(), String> {
    eprintln!("DEBUGPRINT[1]: mod.rs:10: back_up_args={:#?}", back_up_args);
    eprintln!("DEBUGPRINT[2]: mod.rs:10: config={:#?}", config);
    Ok(())
}

mod back_up;
mod back_ups;
mod command_output;
mod device;
mod filesystem;
mod lsblk;

use clap::Args;

use super::back_up::back_ups::BackUps;
use super::back_up::lsblk::Lsblk;
use super::config::Config;

#[derive(Args, Debug)]
pub struct RunArgs {
    /// performs a dry run, no dd operation, just to see the output
    #[clap(short, long, default_value = "false")]
    dry: bool,
    /// pass in the path of the config file
    #[clap(short, long)]
    pub config_file_path: Option<String>,
}

pub fn run(back_up_args: &RunArgs, config: &Config) -> Result<(), String> {
    let lsblk = Lsblk::new()?;
    // eprintln!("DEBUGPRINT[2]: mod.rs:17: lsblk={:#?}", lsblk);

    for back_up_config in &config.backups {
        if let Some(back_ups) = BackUps::new(back_up_config, &lsblk, back_up_args, config)? {
            back_ups.run()?;
        }
    }

    Ok(())
}

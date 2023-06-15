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
pub struct RunArgs {}

pub fn run(back_up_args: &RunArgs, config: Config) -> Result<(), String> {
    eprintln!("DEBUGPRINT[1]: mod.rs:15: back_up_args={:#?}", back_up_args);
    let lsblk = Lsblk::new()?;
    // eprintln!("DEBUGPRINT[2]: mod.rs:17: lsblk={:#?}", lsblk);

    for (dst_filesystem, back_up_config) in &config.dst_filesystems {
        if let Some(back_ups) = BackUps::new(dst_filesystem, back_up_config, &lsblk)? {
            back_ups.run()?;
        }
    }

    Ok(())
}

mod back_up;
mod device;
mod filesystem;
mod lsblk;

use clap::Args;

use super::back_up::back_up::BackUp;
use super::back_up::lsblk::Lsblk;
use super::config::Config;

#[derive(Args, Debug)]
pub struct BackUpArgs {}

pub fn run(back_up_args: &BackUpArgs, config: Config) -> Result<(), String> {
    eprintln!("DEBUGPRINT[1]: mod.rs:15: back_up_args={:#?}", back_up_args);
    let lsblk = Lsblk::new()?;
    eprintln!("DEBUGPRINT[2]: mod.rs:17: lsblk={:#?}", lsblk);

    for (dst_filesystem, back_up_config) in &config.dst_filesystems {
        let backup = BackUp::new(dst_filesystem, back_up_config);
        eprintln!("DEBUGPRINT[2]: mod.rs:17: backup={:#?}", backup);
    }
    Ok(())
}

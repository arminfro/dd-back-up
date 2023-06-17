mod backup;
mod backups;
mod command_output;
mod device;
mod filesystem;
mod lsblk;

use clap::Args;

use super::backup::backups::Backups;
use super::backup::lsblk::Lsblk;
use super::config::Config;

#[derive(Args, Debug)]
pub struct BackupArgs {
    /// performs a dry run, no dd operation, just to see the output
    #[clap(short, long, default_value = "false")]
    dry: bool,
    /// pass in the path of the config file
    #[clap(short, long)]
    pub config_file_path: Option<String>,
}

pub fn run(backup_args: &BackupArgs, config: &Config) -> Result<(), String> {
    let lsblk = Lsblk::new()?;
    // eprintln!("DEBUGPRINT[2]: mod.rs:17: lsblk={:#?}", lsblk);

    for backup_config in &config.backups {
        if let Some(backups) = Backups::new(backup_config, &lsblk, backup_args, config)? {
            backups.run()?;
        }
    }

    Ok(())
}

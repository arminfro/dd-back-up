mod backup;
mod backups;
mod command_output;
mod device;
mod filesystem;
mod lsblk;

use super::backup::backups::Backups;
use super::backup::lsblk::Lsblk;
use super::config::{BackupDevice, Config};
use crate::run::config::BackupConfig;

use clap::Args;

#[derive(Args, Debug)]
pub struct BackupArgs {
    #[clap(short = 'n', long, default_value = "false")]
    /// Performs a dry run, simulating backup operations without making any changes.
    pub dry_run: bool,

    #[clap(flatten)]
    /// Command-line arguments for file-based configuration.
    pub file_config_args: Option<FileConfigArgs>,

    #[clap(flatten)]
    /// Command-line arguments for single backup run mode.
    pub single_backup_args: Option<SingleBackupArgs>,

    #[clap(short, long)]
    /// The mount path of the destination filesystem, overwrites config value.
    pub mountpath: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct FileConfigArgs {
    #[clap(short, long, group = "file-config-args")]
    /// The path to the configuration file.
    pub config_file_path: Option<String>,
}

#[derive(Args, Debug)]
pub struct SingleBackupArgs {
    #[clap(long, conflicts_with = "file-config-args")]
    /// The UUID of the destination backup filesystem or partition, single-back-up-only.
    pub destination_uuid: Option<String>,

    #[clap(long)]
    /// The serial number of the source device to be backed up, single-back-up-only.
    pub source_serial: Option<String>,

    #[clap(long, default_value = "./")]
    /// The destination path where the backup will be stored, single-back-up-only.
    pub destination_path: Option<String>,

    #[clap(long, default_value = None)]
    /// The number of backup copies to maintain, single-back-up-only.
    pub copies: Option<usize>,

    #[clap(long)]
    /// The name of the backup, single-back-up-only.
    pub name: Option<String>,

    #[clap(long, default_value = "fsck -n")]
    /// Alternative command to perform filesystem check (`fsck -n`), single-back-up-only.
    pub fsck_command: String,

    #[clap(long)]
    /// Flag to skip filesystem check (`fsck`), single-back-up-only.
    pub skip_fsck: bool,

    #[clap(long)]
    /// Flag to skip mounting, single-back-up-only.
    pub skip_mount: bool,
}

/// Runs the backup process based on the provided command-line arguments.
///
/// This function takes the parsed `BackupArgs` and orchestrates the backup process based on the provided
/// configuration. It iterates through each backup configuration, creates `Backups` objects, and executes
/// the backup operation. The `lsblk` object is used to gather information about the available block devices.
///
/// # Arguments
///
/// * `backup_args` - A reference to the `BackupArgs` struct containing the parsed command-line arguments.
///
/// # Returns
///
/// An `Ok` variant if the backup process completes successfully, or an `Err` variant with an error message as `String`
/// if an error occurs during the backup process.
pub fn run(backup_args: &BackupArgs) -> Result<(), String> {
    let config = backup_args_to_config(backup_args)?;
    let lsblk = Lsblk::new()?;

    for backup_config in &config.backups {
        if let Some(backups) = Backups::new(backup_config, &lsblk, backup_args, &config)? {
            backups.run()?;
        }
    }

    Ok(())
}

/// Converts `BackupArgs` into a `Config` object.
///
/// This function takes the `BackupArgs` struct, which contains the parsed command-line arguments,
/// and converts it into a `Config` object used by the backup application. It creates a new `Config`
/// object, populates its fields based on the provided arguments, and returns the resulting `Config`.
/// If any error occurs during the conversion process, an `Err` variant with an error message is returned.
///
/// # Arguments
///
/// * `backup_args` - A reference to the `BackupArgs` struct containing the parsed command-line arguments.
///
/// # Returns
///
/// A `Result` containing the resulting `Config` object if the conversion is successful, or an error message as `String`
/// if an error occurs during the conversion.
fn backup_args_to_config(backup_args: &BackupArgs) -> Result<Config, String> {
    let mut config: Config = if let Some(file_config_args) = &backup_args.file_config_args {
        Config::new(&file_config_args.config_file_path)
    } else {
        Config::new(&None)
    }
    .map_err(|e| format!("Failed to create Config struct object: {}", e))?;

    config.mountpath = Some(
        backup_args
            .mountpath
            .clone()
            .unwrap_or(config.mountpath.clone().unwrap_or("/mnt".to_string())),
    );

    config.backups = if let Some(single_backup_args) = &backup_args.single_backup_args {
        let source_serial = single_backup_args.source_serial.clone().ok_or(
            "Source serial needs to be provided in single backup mode, like: `--source-serial x...x`",
        )?;
        let destination_uuid = single_backup_args.destination_uuid.clone().ok_or(
            "Destination UUID needs to be provided in single backup mode, like: `--destination-uuid x...x`",
        )?;

        vec![BackupConfig {
            backup_devices: vec![BackupDevice {
                serial: source_serial,
                name: single_backup_args.name.clone(),
                copies: single_backup_args.copies.clone(),
            }],
            uuid: destination_uuid,
            destination_path: single_backup_args.destination_path.clone(),
            fsck_command: Some(single_backup_args.fsck_command.clone()),
            skip_fsck: Some(single_backup_args.skip_fsck || single_backup_args.skip_mount),
            skip_mount: Some(single_backup_args.skip_mount.clone()),
        }]
    } else {
        config.backups
    };

    Ok(config)
}

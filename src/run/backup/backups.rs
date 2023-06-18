use crate::run::backup::backup::Backup;
use crate::run::config::{BackupConfig, Config};

use super::device::Device;
use super::filesystem::Filesystem;
use super::lsblk::Lsblk;
use super::BackupArgs;

#[derive(Debug)]
pub struct Backups<'a> {
    /// The destination filesystem for the backup.
    pub dst_filesystem: Filesystem,
    /// The list of backup devices.
    pub backup_devices: Vec<Device>,
    /// The command line arguments for the backup operation.
    pub backup_args: &'a BackupArgs,
}

impl<'a> Backups<'a> {
    /// Creates a new `BackUp` instance based on the provided parameters.
    /// It returns `Some(BackUp)` if the destination filesystem is found, otherwise `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `backup_config` - The backup configuration.
    /// * `lsblk` - The `Lsblk` instance containing available filesystems and devices.
    /// * `backup_args` - The command-line arguments for the backup operation.
    /// * `config` - The global configuration.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(BackUps))`: If the destination filesystem is found and the backup is configured.
    /// - `Ok(None)`: If the destination filesystem is not found or not configured for backup.
    /// - `Err(String)`: If there is an error during the process.
    pub fn new(
        backup_config: &BackupConfig,
        lsblk: &Lsblk,
        backup_args: &'a BackupArgs,
        config: &'a Config,
    ) -> Result<Option<Backups<'a>>, String> {
        let dst_filesystem = Filesystem::new(
            &backup_config.uuid,
            &lsblk.available_filesystems,
            config.mountpath.clone(),
        )?;

        if let Some(dst_filesystem) = dst_filesystem {
            let backup_devices_result: Result<Vec<_>, _> = backup_config
                .backup_devices
                .iter()
                .map(|backup_device| {
                    Device::new(
                        &backup_device,
                        &lsblk.available_devices,
                        backup_config.destination_path.clone(),
                    )
                })
                .collect();

            // Unwrap the `Result<Vec<Device>, String>` and filter out any `None` values using `filter_map`
            let backup_devices: Vec<Device> = backup_devices_result
                .map_err(|e| format!("Failed to create Device object: {}", e))?
                .into_iter()
                .filter_map(|x| x)
                .collect();

            let backups = Backups {
                dst_filesystem,
                backup_devices,
                backup_args,
            };
            debug!("{:?}", backups);
            Ok(Some(backups))
        } else {
            Ok(None)
        }
    }

    /// Executes the backup process.
    /// Mount filesystems if needed, do backups pairs matching the conditions, unmount
    /// Returns `Ok(())` if the backup process is successful, otherwise returns an error message.
    pub fn run(mut self) -> Result<(), String> {
        if !self.dst_filesystem.is_mounted() {
            self.dst_filesystem.mount()?;
        }

        for backup_device in &self.backup_devices {
            if let Err(err) =
                Backup::new(&self.dst_filesystem, &backup_device, self.backup_args).run()
            {
                error!("Error performing backup: {}", err);
            }
        }

        self.dst_filesystem.unmount()?;
        Ok(())
    }
}

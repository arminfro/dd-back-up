use crate::dd_back_up::back_up::back_up::BackUp;
use crate::dd_back_up::config::{BackUpConfig, Config};

use super::device::Device;
use super::filesystem::Filesystem;
use super::lsblk::Lsblk;
use super::RunArgs;

#[derive(Debug)]
pub struct BackUps<'a> {
    /// The destination filesystem for the backup.
    pub dst_filesystem: Filesystem,
    /// The list of backup devices.
    pub back_up_devices: Vec<Device>,
    /// The command line arguments for the backup operation.
    pub back_up_args: &'a RunArgs,
}

impl<'a> BackUps<'a> {
    /// Creates a new `BackUp` instance based on the provided parameters.
    /// It returns `Some(BackUp)` if the destination filesystem is found, otherwise `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `back_up_config` - The backup configuration.
    /// * `lsblk` - The `Lsblk` instance containing available filesystems and devices.
    /// * `back_up_args` - The command-line arguments for the backup operation.
    /// * `config` - The global configuration.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(BackUps))`: If the destination filesystem is found and the backup is configured.
    /// - `Ok(None)`: If the destination filesystem is not found or not configured for backup.
    /// - `Err(String)`: If there is an error during the process.
    pub fn new(
        back_up_config: &BackUpConfig,
        lsblk: &Lsblk,
        back_up_args: &'a RunArgs,
        config: &'a Config,
    ) -> Result<Option<BackUps<'a>>, String> {
        let dst_filesystem = Filesystem::new(
            &back_up_config.uuid,
            &lsblk.available_filesystems,
            config.mountpath.clone(),
        )?;

        if let Some(dst_filesystem) = dst_filesystem {
            let back_up_devices_result: Result<Vec<_>, _> = back_up_config
                .back_up_devices
                .iter()
                .map(|back_up_device| {
                    Device::new(
                        &back_up_device.serial,
                        &back_up_device.name,
                        &lsblk.available_devices,
                        back_up_config.destination_path.clone(),
                    )
                })
                .collect();

            // Unwrap the `Result<Vec<Device>, String>` and filter out any `None` values using `filter_map`
            let back_up_devices: Vec<Device> = back_up_devices_result
                .map_err(|e| format!("Failed to create Device object: {}", e))?
                .into_iter()
                .filter_map(|x| x)
                .collect();

            Ok(Some(BackUps {
                dst_filesystem,
                back_up_devices,
                back_up_args,
            }))
        } else {
            Ok(None)
        }
    }

    /// Executes the backup process.
    /// Returns `Ok(())` if the backup process is successful, otherwise returns an error message.
    pub fn run(mut self) -> Result<(), String> {
        if !self.dst_filesystem.is_mounted() {
            self.dst_filesystem.mount()?;
        }

        for back_up_device in &self.back_up_devices {
            if let Err(err) = self.do_backup(back_up_device) {
                eprintln!("Error performing backup: {}", err);
            }
        }

        self.dst_filesystem.unmount()?;
        Ok(())
    }

    /// Performs the backup for a specific device.
    ///
    /// # Arguments
    ///
    /// * `back_up_device` - The device to perform the backup for.
    fn do_backup(&self, back_up_device: &Device) -> Result<(), String> {
        let back_up = BackUp::new(&self.dst_filesystem, &back_up_device, self.back_up_args);
        back_up.run()
    }
}

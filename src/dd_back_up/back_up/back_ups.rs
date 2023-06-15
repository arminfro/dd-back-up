use crate::dd_back_up::back_up::back_up::BackUp;
use crate::dd_back_up::config::BackUpConfig;

use super::device::Device;
use super::filesystem::Filesystem;
use super::lsblk::Lsblk;

#[derive(Debug)]
pub struct BackUps {
    dst_filesystem: Filesystem,
    back_up_devices: Vec<Device>,
}

impl BackUps {
    /// Creates a new `BackUp` instance based on the provided parameters.
    /// It returns `Some(BackUp)` if the destination filesystem is found, otherwise `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `dst_filesystem_uuid` - The UUID of the destination filesystem.
    /// * `back_up_config` - The backup configuration.
    /// * `lsblk` - The `Lsblk` instance containing available filesystems and devices.
    pub fn new(
        dst_filesystem_uuid: &String,
        back_up_config: &BackUpConfig,
        lsblk: &Lsblk,
    ) -> Result<Option<BackUps>, String> {
        let dst_filesystem = Filesystem::new(dst_filesystem_uuid, &lsblk.available_filesystems)?;

        if let Some(dst_filesystem) = dst_filesystem {
            let back_up_devices_result: Result<Vec<_>, _> = back_up_config
                .back_up_devices
                .iter()
                .map(|back_up_device| Device::new(back_up_device, &lsblk.available_devices))
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
        let back_up = BackUp::new(&self.dst_filesystem, &back_up_device);
        back_up.run()
    }
}

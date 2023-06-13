use crate::dd_back_up::config::BackUpConfig;

use super::device::Device;
use super::filesystem::Filesystem;
use super::lsblk::Lsblk;

#[derive(Debug)]
pub struct BackUp {
    dst_filesystem: Filesystem,
    back_up_devices: Vec<Device>,
}

impl BackUp {
    pub fn new(
        dst_filesystem_uuid: &String,
        back_up_config: &BackUpConfig,
        lsblk: &Lsblk,
    ) -> Result<Option<BackUp>, String> {
        let dst_filesystem = Filesystem::new(dst_filesystem_uuid, &lsblk.available_filesystems)?;

        if dst_filesystem.is_some() {
            let dst_filesystem = dst_filesystem.unwrap();

            let back_up_devices_result: Result<Vec<_>, _> = back_up_config
                .back_up_devices
                .iter()
                .map(|back_up_device| Device::new(back_up_device, &lsblk.available_devices))
                .collect();

            let back_up_devices = back_up_devices_result
                .map_err(|e| format!("Failed to create Device object: {}", e))?
                .into_iter()
                .filter_map(|x| x)
                .collect();

            let backup = BackUp {
                dst_filesystem,
                back_up_devices,
            };
            eprintln!("DEBUGPRINT[1]: back_up.rs:40: backup={:#?}", backup);
            backup.run();
            Ok(Some(backup))
        } else {
            Ok(None)
        }
    }

    pub fn run(&self) {
        // continue developing...
    }
}

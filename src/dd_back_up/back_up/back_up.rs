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
        let dst_filesystem = Filesystem::new(dst_filesystem_uuid, lsblk)?;
        eprintln!(
            "DEBUGPRINT[1]: back_up.rs:19: dst_filesystem={:#?}",
            dst_filesystem
        );

        if dst_filesystem.is_some() {
            let dst_filesystem = dst_filesystem.unwrap();
            let back_up_devices = back_up_config
                .back_up_devices
                .iter()
                .map(|back_up_device| Device::new(back_up_device))
                .collect();
            let backup = BackUp {
                dst_filesystem,
                back_up_devices,
            };
            Ok(Some(backup))
        } else {
            Ok(None)
        }
    }
}

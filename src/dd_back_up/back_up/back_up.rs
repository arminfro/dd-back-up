use crate::dd_back_up::config::BackUpConfig;

use super::device::Device;
use super::filesystem::Filesystem;

#[derive(Debug)]
pub struct BackUp {
    dst_filesystem: Filesystem,
    back_up_devices: Vec<Device>,
}

impl BackUp {
    pub fn new(dst_filesystem_uuid: &String, back_up_config: &BackUpConfig) -> BackUp {
        let dst_filesystem = Filesystem::new(dst_filesystem_uuid);
        let back_up_devices = back_up_config
            .back_up_devices
            .iter()
            .map(|back_up_device| Device::new(back_up_device))
            .collect();

        BackUp {
            dst_filesystem,
            back_up_devices,
        }
    }
}

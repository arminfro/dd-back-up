use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::run::{config::BackupDevice, utils::convert_to_byte_size};

use super::lsblk::BlockDevice;

/// Represents a device identified by its serial number.
#[derive(Debug)]
pub struct Device {
    /// The underlying block device information.
    pub blockdevice: BlockDevice,
    /// The path to the device.
    pub device_path: String,
    /// The name of the device.
    pub name: Option<String>,
    /// The destination path for the device.
    pub destination_path: String,
    /// The number of copies to be kept for this device.
    pub copies: Option<usize>,
}

impl Device {
    /// Creates a new `Device` instance with the specified serial number and optional name.
    ///
    /// It validates the uniqueness of the serial number among the available devices
    /// and returns `Some(Device)` if a unique match is found, or `None` otherwise.
    /// Additionally, it checks if the device is currently mounted and filters out mounted devices.
    ///
    /// # Arguments
    ///
    /// * `serial` - The serial number of the device.
    /// * `name` - The optional name of the device.
    /// * `available_devices` - The list of available block devices.
    /// * `destination_path` - The optional destination path for the device from the configuration.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(Device))`: If a unique device is found matching the serial number and if it isn't mounted.
    /// - `Ok(None)`: If no device is found matching the serial number or all matching devices are mounted.
    /// - `Err(String)`: If the serial number is not unique among the available devices.
    pub fn new(
        backup_device: &BackupDevice,
        available_devices: &[BlockDevice],
        destination_path: String,
    ) -> Result<Option<Device>, String> {
        match Self::validate_serial(&backup_device.serial, available_devices) {
            Ok(blockdevice) => {
                if !Self::is_device_mounted(&format!("/dev/{}", &blockdevice.name))? {
                    Ok(Some(Device {
                        blockdevice: blockdevice.clone(),
                        device_path: format!("/dev/{}", &blockdevice.name),
                        name: backup_device.name.clone(),
                        copies: backup_device.copies,
                        destination_path,
                    }))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                warn!("{}, skipping it", e);
                Ok(None)
            }
        }
    }

    /// Filters the available devices to those with the specified serial number,
    /// ensuring uniqueness and presence of device
    fn validate_serial<'a>(
        serial: &str,
        available_devices: &'a [BlockDevice],
    ) -> Result<&'a BlockDevice, String> {
        let serial_filtered_lsblk: Vec<&BlockDevice> = available_devices
            .iter()
            .filter(|blockdevice| blockdevice.serial.clone().unwrap() == serial)
            .collect();

        let is_device_serial_uniq = serial_filtered_lsblk.len() <= 1;
        let is_device_available = serial_filtered_lsblk.len() != 0;

        if is_device_available {
            if is_device_serial_uniq {
                return Ok(serial_filtered_lsblk[0]);
            } else {
                return Err(format!("Device has not a unique serial: {}", serial));
            }
        }
        Err(format!("Device not found: {}", serial))
    }

    /// Checks if the specified device is currently mounted by querying `/proc/mounts`.
    ///
    /// Returns `Ok(true)` if the device is mounted, `Ok(false)` if it is not mounted,
    /// or `Err(String)` if an error occurred while checking.
    fn is_device_mounted(device_path: &str) -> Result<bool, String> {
        let file = File::open("/proc/mounts")
            .map_err(|e| format!("Failed to open /proc/mounts: {}", e.to_string()))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(entry) = line {
                let fields: Vec<&str> = entry.split(' ').collect();
                if fields.len() >= 2 && fields[0].contains(device_path) {
                    error!("Device {} is mounted, skipping it", device_path);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Returns the total size of the block device, converted to bytes, or None if the size is unavailable.
    /// This value is static in one run
    pub fn total_size(&self) -> Result<Option<u64>, String> {
        convert_to_byte_size(&self.blockdevice.size)
    }
}

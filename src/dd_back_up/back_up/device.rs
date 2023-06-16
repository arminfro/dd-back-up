use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::dd_back_up::{config::BackUpDevice, utils::convert_to_byte_size};

use super::lsblk::BlockDevice;

/// Represents a device identified by its serial number.
#[derive(Debug)]
pub struct Device {
    /// The underlying block device information.
    pub blockdevice: BlockDevice,
    /// The path to the device.
    pub device_path: String,
    /// The name of the device.
    pub name: String,
    /// The destination path for the device.
    pub destination_path: String,
    /// The number of copies to be kept for this device.
    pub copies: usize,
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
    /// - `Ok(Some(Device))`: If a unique device is found matching the serial number and is not mounted.
    /// - `Ok(None)`: If no device is found matching the serial number or all matching devices are mounted.
    /// - `Err(String)`: If the serial number is not unique among the available devices.
    pub fn new(
        back_up_device: &BackUpDevice,
        available_devices: &[BlockDevice],
        destination_path: Option<String>,
    ) -> Result<Option<Device>, String> {
        let serial_filtered_lsblk =
            Self::validate_serial_uniq(&back_up_device.serial, available_devices)?;

        let device = Self::validate_present_serial(serial_filtered_lsblk)
            .filter(|blockdevice| {
                !(Self::is_device_mounted(&format!("/dev/{}", &blockdevice.name))
                    .ok()
                    .unwrap_or(false))
            })
            .map(|blockdevice| Device {
                blockdevice: blockdevice.clone(),
                device_path: format!("/dev/{}", &blockdevice.name),
                name: back_up_device
                    .name
                    .clone()
                    .unwrap_or("".to_string())
                    .replace(" ", "-"),
                destination_path: destination_path.unwrap_or("./".to_string()),
                copies: back_up_device.copies.unwrap_or(1),
            });

        Ok(device)
    }

    /// Validates the presence of a unique device with the specified serial number.
    fn validate_present_serial(serial_filtered_lsblk: Vec<&BlockDevice>) -> Option<&BlockDevice> {
        if serial_filtered_lsblk.len() == 1 {
            Some(serial_filtered_lsblk[0])
        } else {
            None
        }
    }

    /// Filters the available devices to those with the specified serial number,
    /// ensuring uniqueness.
    fn validate_serial_uniq<'a>(
        serial: &str,
        available_devices: &'a [BlockDevice],
    ) -> Result<Vec<&'a BlockDevice>, String> {
        let serial_filtered_lsblk: Vec<&BlockDevice> = available_devices
            .iter()
            .filter(|blockdevice| blockdevice.serial.as_deref() == Some(serial))
            .collect();

        if serial_filtered_lsblk.len() <= 1 {
            Ok(serial_filtered_lsblk)
        } else {
            Err(format!("Not a unique serial: {}", serial))
        }
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
                    eprintln!("Device {} is mounted, skipping.", device_path);

                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Returns the total size of the block device, converted to bytes, or None if the size is unavailable.
    pub fn total_size(&self) -> Result<Option<u64>, String> {
        convert_to_byte_size(&self.blockdevice.size)
    }
}

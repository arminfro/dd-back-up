use super::lsblk::BlockDevice;

/// Represents a device identified by its serial number.
#[derive(Debug)]
pub struct Device {
    /// The underlying block device information.
    pub blockdevice: BlockDevice,
    pub device_path: String,
    pub name: String,
    pub destination_path: String,
}

impl Device {
    /// Creates a new `Device` instance with the specified serial number and optional name.
    ///
    /// It validates the uniqueness of the serial number among the available devices
    /// and returns `Some(Device)` if a unique match is found, or `None` otherwise.
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
    /// - `Ok(Some(Device))`: If a unique device is found matching the serial number.
    /// - `Ok(None)`: If no device is found matching the serial number.
    /// - `Err(String)`: If the serial number is not unique among the available devices.
    pub fn new(
        serial: &str,
        name: &Option<String>,
        available_devices: &[BlockDevice],
        destination_path: Option<String>,
    ) -> Result<Option<Device>, String> {
        let serial_filtered_lsblk = Self::validate_serial_uniq(serial, available_devices)?;

        let device =
            Self::validate_present_serial(serial_filtered_lsblk).map(|blockdevice| Device {
                blockdevice: blockdevice.clone(),
                device_path: format!("/dev/{}", &blockdevice.name),
                name: name.clone().unwrap_or("".to_string()).replace(" ", "-"),
                destination_path: destination_path.unwrap_or("./".to_string()),
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
}

use super::lsblk::BlockDevice;

/// Represents a device identified by its serial number.
#[derive(Debug)]
pub struct Device {
    /// The underlying block device information.
    pub blockdevice: BlockDevice,
    pub device_path: String,
}

impl Device {
    /// Creates a new `Device` instance with the specified serial number.
    ///
    /// It validates the uniqueness of the serial number among the available devices
    /// and returns `Some(Device)` if a unique match is found, or `None` otherwise.
    pub fn new(serial: &str, available_devices: &[BlockDevice]) -> Result<Option<Device>, String> {
        let serial_filtered_lsblk = Self::validate_serial_uniq(serial, available_devices)?;

        let device =
            Self::validate_present_serial(serial_filtered_lsblk).map(|blockdevice| Device {
                blockdevice: blockdevice.clone(),
                device_path: format!("/dev/{}", &blockdevice.name),
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

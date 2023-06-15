use super::{command_output::command_output, lsblk::BlockDevice};

/// Represents a filesystem associated with a block device.
#[derive(Debug)]
pub struct Filesystem {
    /// The underlying block device information.
    pub blockdevice: BlockDevice,
    /// The path to the device.
    pub device_path: String,
    /// The mount path for the filesystem.
    pub mountpath: Option<String>,
}

impl Filesystem {
    /// Creates a new `Filesystem` instance for the specified UUID, using the provided `Lsblk` instance.
    ///
    /// It returns `Ok(Some(Filesystem))` if the UUID is unique and associated with a block device,
    /// `Ok(None)` if the UUID is not found in the available filesystems,
    /// or an error message if the UUID is not unique.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID of the filesystem.
    /// * `available_filesystems` - The list of available block devices to search for a matching UUID.
    /// * `mountpath` - The optional mount path of the filesystem.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(Filesystem))`: If a unique match is found based on the UUID.
    /// - `Ok(None)`: If no match is found based on the UUID.
    /// - `Err(String)`: If the UUID is not unique among the available filesystems.
    pub fn new(
        uuid: &str,
        available_filesystems: &Vec<BlockDevice>,
        mountpath: Option<String>,
    ) -> Result<Option<Filesystem>, String> {
        let uuid_filtered_lsblk = Self::validate_uuid_uniq(uuid, available_filesystems)?;

        match Self::validate_present_uuid(uuid_filtered_lsblk) {
            Some(blockdevice) => Ok(Some(Filesystem {
                blockdevice: blockdevice.clone(),
                device_path: format!("/dev/{}", &blockdevice.name),
                mountpath,
            })),
            None => Ok(None),
        }
    }

    /// Validates if the UUID is associated with a unique block device.
    /// Returns `Some(&BlockDevice)` if the UUID is unique and associated with a block device,
    /// or `None` if it's not unique.
    fn validate_present_uuid(uuid_filtered_lsblk: Vec<&BlockDevice>) -> Option<&BlockDevice> {
        if uuid_filtered_lsblk.len() == 1 {
            Some(uuid_filtered_lsblk[0])
        } else {
            None
        }
    }

    /// Validates if the UUID is unique among the available filesystems.
    /// Returns a filtered list of block devices with the specified UUID, or an error if the UUID is not unique.
    fn validate_uuid_uniq<'b>(
        uuid: &str,
        available_filesystems: &'b Vec<BlockDevice>,
    ) -> Result<Vec<&'b BlockDevice>, String> {
        let uuid_filtered_lsblk: Vec<&BlockDevice> = available_filesystems
            .iter()
            .filter(|filesystem| filesystem.uuid.as_deref() == Some(uuid))
            .collect::<Vec<&BlockDevice>>();

        if uuid_filtered_lsblk.len() <= 1 {
            Ok(uuid_filtered_lsblk)
        } else {
            Err(format!("Not a unique UUID: {}", uuid))
        }
    }

    /// Checks if the device is mounted.
    /// Returns `true` if the device is mounted, otherwise `false`.
    pub fn is_mounted(&self) -> bool {
        self.blockdevice.mountpoint.is_some()
    }

    /// Mounts the device.
    /// Returns `Ok(())` if the device is mounted successfully, otherwise returns an error message.
    pub fn mount(&mut self) -> Result<(), String> {
        let mount_path = self.mount_path();
        let output = command_output(
            vec!["mount", &self.device_path, &mount_path],
            &format!("mount filesystem {} at {}", self.device_path, mount_path),
            Some(true),
        )?;

        if output.status.success() {
            self.blockdevice.mountpoint = Some(mount_path);
            println!("Filesystem mounted successfully");
            Ok(())
        } else {
            Err(format!("Error mounting filesystem {}", self.device_path))
        }
    }

    /// Unmounts the device.
    /// Returns `Ok(())` if the device is unmounted successfully, otherwise returns an error message.
    pub fn unmount(&mut self) -> Result<(), String> {
        let mountpoint = self
            .blockdevice
            .mountpoint
            .clone()
            .ok_or_else(|| self.mount_path())?;

        let output = command_output(
            vec!["umount", &mountpoint],
            &format!("unmount filesystem {} at {}", self.device_path, &mountpoint),
            Some(true),
        )?;

        if output.status.success() {
            self.blockdevice.mountpoint = None;
            println!("Filesystem unmounted successfully");
            Ok(())
        } else {
            Err(format!(
                "Error unmounting filesystem {} at {}: {}",
                self.device_path,
                &mountpoint,
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }

    fn mount_path(&self) -> String {
        self.mountpath.clone().unwrap_or("/mnt".to_string())
    }
}

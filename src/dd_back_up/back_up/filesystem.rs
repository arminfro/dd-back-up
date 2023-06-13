use super::{command_output::command_output, lsblk::BlockDevice};

const MOUNT_PATH: &str = "/mnt";

/// Represents a filesystem associated with a block device.
#[derive(Debug)]
pub struct Filesystem {
    pub blockdevice: BlockDevice,
    pub device_path: String,
}

impl Filesystem {
    /// Creates a new `Filesystem` instance for the specified UUID, using the provided `Lsblk` instance.
    /// It returns `Ok(Some(Filesystem))` if the UUID is unique and associated with a block device,
    /// `Ok(None)` if the UUID is not found in the available filesystems, or an error message if the UUID is not unique.
    pub fn new(
        uuid: &str,
        available_filesystems: &Vec<BlockDevice>,
    ) -> Result<Option<Filesystem>, String> {
        let uuid_filtered_lsblk = Self::validate_uuid_uniq(uuid, available_filesystems)?;

        match Self::validate_present_uuid(uuid_filtered_lsblk) {
            Some(blockdevice) => Ok(Some(Filesystem {
                blockdevice: blockdevice.clone(),
                device_path: format!("/dev/{}", &blockdevice.name),
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
    fn validate_uuid_uniq<'a>(
        uuid: &str,
        available_filesystems: &'a Vec<BlockDevice>,
    ) -> Result<Vec<&'a BlockDevice>, String> {
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
        let output = command_output(
            vec!["mount", &self.device_path, MOUNT_PATH],
            &format!("mount filesystem {} at {}", self.device_path, MOUNT_PATH),
            Some(true),
        )?;

        if output.status.success() {
            self.blockdevice.mountpoint = Some(MOUNT_PATH.to_string());
            println!("Filesystem mounted successfully");
            Ok(())
        } else {
            Err(format!("Error mounting filesystem {}", self.device_path))
        }
    }

    /// Unmounts the device.
    /// Returns `Ok(())` if the device is unmounted successfully, otherwise returns an error message.
    pub fn unmount(&mut self) -> Result<(), String> {
        let output = command_output(
            vec!["umount", MOUNT_PATH],
            &format!("unmount filesystem {} at {}", self.device_path, MOUNT_PATH),
            Some(true),
        )?;

        if output.status.success() {
            self.blockdevice.mountpoint = None;
            println!("Filesystem unmounted successfully");
            Ok(())
        } else {
            Err(format!("Error unmounting filesystem {}", self.device_path))
        }
    }
}

use super::lsblk::BlockDevice;

/// Represents a filesystem associated with a block device.
#[derive(Debug)]
pub struct Filesystem {
    pub blockdevice: BlockDevice,
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
}

use std::{fs, path::Path};

use super::{command_output::command_output, lsblk::BlockDevice};

/// Represents a filesystem associated with a block device.
#[derive(Debug)]
pub struct Filesystem {
    /// The underlying block device information.
    pub blockdevice: BlockDevice,
    /// The path to the device.
    pub device_path: String,
    /// The mount path for the filesystem.
    pub mountpath: String,
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
                mountpath: mountpath.unwrap_or("/mnt".to_string()),
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
        let output = command_output(
            vec!["mount", &self.device_path, &self.mountpath],
            &format!(
                "mount filesystem {} at {}",
                self.device_path, self.mountpath
            ),
            Some(true),
        )?;

        if output.status.success() {
            self.blockdevice.mountpoint = Some(self.mountpath.clone());
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
            .ok_or(self.mountpath.clone())?;

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

    /// Checks if the number of existing backups exceeds the specified number of copies.
    pub fn present_number_of_copies(
        &self,
        stable_postfix_file_name: &str,
        back_up_dst_dir: &str,
    ) -> usize {
        let backup_files = match fs::read_dir(back_up_dst_dir) {
            Ok(files) => files
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        e.file_name()
                            .to_str()
                            .map(|s| s.to_string())
                            .filter(|s| s.contains(stable_postfix_file_name))
                    })
                })
                .collect::<Vec<String>>(),
            Err(_) => Vec::new(),
        };

        backup_files.len() // >= self.back_up_device.copies as usize
    }

    /// Deletes the oldest backup file.
    pub fn delete_oldest_backup(
        &self,
        stable_postfix_file_name: &str,
        back_up_dst_path: &str,
    ) -> Result<(), String> {
        let backup_files = fs::read_dir(back_up_dst_path)
            .map_err(|e| format!("Failed to read backup directory: {}", e))?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.file_name()
                        .to_str()
                        .map(|s| s.to_string())
                        .filter(|s| s.contains(stable_postfix_file_name))
                })
            })
            .collect::<Vec<String>>();

        if let Some(oldest_file) = backup_files.iter().min_by_key(|&file_name| {
            let file_path = Path::new(back_up_dst_path).join(file_name);
            if let Ok(metadata) = fs::metadata(&file_path) {
                if let Ok(created) = metadata.created() {
                    return created;
                }
            }
            // fallback value to ensure consistent ordering in case of None
            std::time::UNIX_EPOCH
        }) {
            let file_path = format!("{}/{}", back_up_dst_path, oldest_file);
            println!("Removing old back up file: {}", file_path);
            fs::remove_file(&file_path)
                .map_err(|e| format!("Failed to delete oldest backup file '{}': {}", file_path, e))
        } else {
            Ok(())
        }
    }
}

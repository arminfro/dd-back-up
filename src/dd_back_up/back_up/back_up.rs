use std::path::Path;

use relative_path::RelativePath;

use crate::dd_back_up::utils::current_date;

use super::{command_output::command_output, device::Device, filesystem::Filesystem, RunArgs};

pub struct BackUp<'a> {
    /// The destination filesystem for the backup.
    pub dst_filesystem: &'a Filesystem,
    /// The backup device.
    pub back_up_device: &'a Device,
    /// The command line arguments for the backup operation.
    pub back_up_args: &'a RunArgs,
}

impl<'a> BackUp<'a> {
    /// Creates a new `BackUp` instance.
    ///
    /// # Arguments
    ///
    /// * `dst_filesystem` - The destination filesystem for the backup.
    /// * `back_up_device` - The device to be backed up.
    pub fn new(
        dst_filesystem: &'a Filesystem,
        back_up_device: &'a Device,
        back_up_args: &'a RunArgs,
    ) -> BackUp<'a> {
        BackUp {
            dst_filesystem,
            back_up_device,
            back_up_args,
        }
    }

    /// Runs the backup process using the `dd` command.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the backup process is successful.
    /// * `Err` with an error message if the backup process encounters an error.
    pub fn run(&self) -> Result<(), String> {
        self.validate_state()?;

        let input_file_arg = format!("if={}", self.input_file_path());
        let output_file_arg = format!("of={}", self.back_up_file_path());
        let command_parts = vec!["dd", &input_file_arg, &output_file_arg, "status=progress"];
        let description = format!("run dd command: {:?}", &command_parts.join(" "));
        match self.back_up_args.dry {
            true => {
                println!(
                    "[Dry-Run] backup would run with command: {}",
                    &command_parts.join(" "),
                );
                Ok(())
            }
            false => {
                let output =
                    command_output(command_parts.clone(), description.as_str(), Some(true))?;

                if output.status.success() {
                    println!(
                        "Success running backup with dd command {}: {}",
                        &command_parts.join(" "),
                        String::from_utf8_lossy(&output.stdout).to_string()
                    );

                    self.chown()
                } else {
                    Err(format!(
                        "Error running dd command {}: {}",
                        &command_parts.join(" "),
                        String::from_utf8_lossy(&output.stderr).to_string()
                    ))
                }
            }
        }
    }

    /// Sets the owner of the backup file to the current user ID and group ID.
    ///
    /// This function changes the owner of the backup file specified by `output_file_path`
    /// to the current user and group. It uses the `chown` command to perform the operation.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(String)`: If an error occurs during the operation.
    fn chown(&self) -> Result<(), String> {
        let output_file_path = self.back_up_file_path();

        // Retrieve the current user and group IDs
        let user_id = unsafe { libc::getuid() };
        let group_id = unsafe { libc::getgid() };

        let user_group_id_arg = format!("{}:{}", user_id, group_id);
        let command_parts = vec!["chown", &user_group_id_arg, &output_file_path];
        command_output(
            command_parts,
            "change owner of backup file to $UID",
            Some(true),
        )
        .map(|_| ())
    }

    /// Returns the input file path for the backup.
    fn input_file_path(&self) -> String {
        self.back_up_device.device_path.clone()
    }

    fn back_up_dir_path(&self) -> String {
        let relative_path =
            RelativePath::new(&self.dst_filesystem.blockdevice.mountpoint.clone().unwrap())
                .join_normalized(self.back_up_device.destination_path.clone())
                .to_string();

        format!("/{}", relative_path)
    }

    /// Returns the output file path for the backup.
    fn back_up_file_path(&self) -> String {
        let relative_path = RelativePath::new(&self.back_up_dir_path())
            .join_normalized(self.file_name())
            .to_string();

        format!("/{}", relative_path)
    }

    /// Generates the file name for the backup image.
    fn file_name(&self) -> String {
        format!(
            "{}_{}_{}",
            current_date(),
            self.back_up_device.name,
            self.stable_postfix_file_name().replace(" ", "-")
        )
    }

    /// Generates the stable postfix file name for the backup image.
    ///
    /// The stable postfix file name is generated by combining the model and serial
    /// number of the block device associated with the backup. Any spaces in the
    /// names are replaced with hyphens.
    ///
    /// # Returns
    ///
    /// The stable postfix file name as a string.
    fn stable_postfix_file_name(&self) -> String {
        format!(
            "{}.img",
            vec![
                self.back_up_device.blockdevice.model.clone(),
                self.back_up_device.blockdevice.serial.clone(),
            ]
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<String>>()
            .join("_")
            .replace(" ", "-")
        )
    }

    /// Checks if the number of existing backups exceeds the specified number of copies.
    fn needs_deletion(&self) -> bool {
        let present_number_of_copies = self
            .dst_filesystem
            .present_number_of_copies(&self.stable_postfix_file_name(), &self.back_up_dir_path());
        present_number_of_copies >= self.back_up_device.copies as usize
    }

    /// Validates the state of the backup process by performing the following checks:
    /// 1. Checks if the target file is already present. If it is, an error is returned.
    /// 2. Checks if the oldest backup needs to be deleted based on the configured number of copies.
    ///    If a deletion is required, the oldest backup is deleted.
    /// 3. If no deletion is needed, checks if the target filesystem has enough space to accommodate
    ///    the new backup. If there is insufficient space, an error is returned.
    /// If all checks pass, `Ok(())` is returned indicating that the state is valid and the backup
    /// process can proceed.
    fn validate_state(&self) -> Result<(), String> {
        self.check_if_target_file_is_present()?;
        let needed_deletion = self.delete_oldest_backup_if_needed()?;
        if !needed_deletion {
            self.check_if_target_filesystem_has_enough_space()?;
        }
        Ok(())
    }

    /// Deletes the oldest backup file if the number of existing backups exceeds the specified number of copies.
    fn delete_oldest_backup_if_needed(&self) -> Result<bool, String> {
        let needs_deletion = self.needs_deletion();
        if needs_deletion && !self.back_up_args.dry {
            self.dst_filesystem
                .delete_oldest_backup(&self.stable_postfix_file_name(), &self.back_up_dir_path())?;
        }
        Ok(needs_deletion)
    }

    /// Checks if the target filesystem has enough space to accommodate the backup of the device.
    /// It compares the available space on the filesystem with the total size of the device to be backed up.
    /// If there is sufficient space, `Ok(())` is returned, indicating that the backup can proceed.
    /// If there is not enough space, an error is returned with a descriptive message.
    /// If either available_space or needed_space is None then proceed with an Ok as well.
    fn check_if_target_filesystem_has_enough_space(&self) -> Result<(), String> {
        let available_space = self.dst_filesystem.available_space();
        let needed_space = self.back_up_device.total_size()?;

        if let Some(available_space) = available_space {
            if let Some(needed_space) = needed_space {
                let remaining_space: i64 = available_space as i64 - needed_space as i64;
                if remaining_space > 0 {
                    return Ok(());
                } else {
                    return Err(format!(
                        "Not enough space on destination filesystem {}, to backup device {}",
                        self.dst_filesystem.device_path, self.back_up_device.device_path
                    ));
                }
            }
        }
        println!("Could not check if sufficient space is available");
        Ok(())
    }

    /// Checks if the target backup file is already present.
    ///
    /// If the backup file already exists at the specified output file path,
    /// this function returns an error indicating that the backup should be skipped.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the backup file does not exist and can proceed.
    /// - `Err(String)`: If the backup file is already present.
    fn check_if_target_file_is_present(&self) -> Result<(), String> {
        let file_path = self.back_up_file_path();
        let path = Path::new(&file_path);

        if path.exists() && path.is_file() {
            Err(format!(
                "Backup file is already present {}. Skipping. If you want to do more than 1 backup per day, rename file manually",
                file_path
            ))
        } else {
            Ok(())
        }
    }
}

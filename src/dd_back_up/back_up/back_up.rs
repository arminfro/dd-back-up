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
        let input_file_arg = format!("if={}", self.input_file_path());
        let output_file_arg = format!("of={}", self.output_file_path());
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

                    Ok(())
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

    /// Returns the input file path for the backup.
    fn input_file_path(&self) -> String {
        self.back_up_device.device_path.clone()
    }

    /// Returns the output file path for the backup.
    fn output_file_path(&self) -> String {
        let relative_path =
            RelativePath::new(&self.dst_filesystem.blockdevice.mountpoint.clone().unwrap())
                .join_normalized(self.back_up_device.destination_path.clone())
                .join_normalized(self.file_name())
                .to_string();

        format!("/{}", relative_path)
    }

    /// Generates the file name for the backup image.
    fn file_name(&self) -> String {
        format!(
            "{}_{}_{}.img",
            current_date(),
            self.back_up_device.name,
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
}

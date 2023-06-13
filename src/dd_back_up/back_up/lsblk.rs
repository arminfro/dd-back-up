use serde::{Deserialize, Serialize};
use serde_json;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockDevice {
    pub name: String,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub uuid: Option<String>,
    pub wwn: Option<String>,
    pub size: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LsblkOutput {
    pub blockdevices: Vec<BlockDevice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lsblk {
    available_devices: Vec<BlockDevice>,
    available_filesystems: Vec<BlockDevice>,
}

impl Lsblk {
    /// Creates a new instance of `Lsblk`.
    ///
    /// It captures the output of the `lsblk` command, filters and stores the available devices
    /// and available filesystems.
    ///
    /// Returns:
    /// - `Ok(Lsblk)`: If the `lsblk` command was successful and the output was parsed correctly.
    /// - `Err(String)`: If there was an error executing or parsing the `lsblk` command.
    pub fn new() -> Result<Lsblk, String> {
        let lsblk_output =
            Self::capture_lsblk().map_err(|e| format!("Failed to read JSON from lsblk: {}", e))?;

        let available_devices = Self::available_devices(&lsblk_output);
        let available_filesystems = Self::available_filesystems(&lsblk_output);

        Ok(Lsblk {
            available_devices,
            available_filesystems,
        })
    }

    /// Filters and returns the available devices from the lsblk output.
    fn available_devices(lsblk_output: &LsblkOutput) -> Vec<BlockDevice> {
        lsblk_output
            .blockdevices
            .iter()
            .filter(|a| a.serial.is_some())
            .cloned()
            .collect()
    }

    /// Filters and returns the available filesystems from the lsblk output.
    fn available_filesystems(lsblk_output: &LsblkOutput) -> Vec<BlockDevice> {
        lsblk_output
            .blockdevices
            .iter()
            .filter(|a| a.uuid.is_some())
            .cloned()
            .collect()
    }

    /// Executes the lsblk command and captures the output as a JSON string.
    ///
    /// Returns:
    /// - `Ok(LsblkOutput)`: If the lsblk command was successful and the JSON output was parsed correctly.
    /// - `Err(String)`: If there was an error executing or parsing the lsblk command.
    fn capture_lsblk() -> Result<LsblkOutput, String> {
        let output = Command::new("lsblk")
            .args(&["-lJ", "-o", "NAME,MODEL,SERIAL,SIZE,UUID,WWN"])
            .stdout(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to execute lsblk: {}", e))?;

        if output.status.success() {
            let stdout = output.stdout;
            let stdout_str = String::from_utf8_lossy(&stdout).to_string();

            let lsblk_output: LsblkOutput = serde_json::from_str(&stdout_str)
                .map_err(|e| format!("Failed to deserialize JSON: {}", e))?;

            Ok(lsblk_output)
        } else {
            Err("Execution of lsblk failed".to_string())
        }
    }
}

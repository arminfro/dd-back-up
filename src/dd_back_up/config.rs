use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::PathBuf,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BackUpDevice {
    /// The serial number of the device.
    pub serial: String,
    /// An optional name for the device.
    pub name: Option<String>,
}

/// Represents the configuration for a single backup.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BackUpConfig {
    /// The list of devices to be backed up.
    ///
    /// Strings are identifiers of whole devices.
    /// The identifier can be the serial number or the wwn (world wide name).
    /// Since some devices may not have a serial number or even have duplicated serial numbers,
    /// the identifier serves as a unique identifier for the device.
    pub back_up_devices: Vec<BackUpDevice>,
    /// The UUID of the destination backup filesystem or partition.
    pub uuid: String,
    /// The destination path where the backup will be stored.
    /// If not provided, the default path will be used.
    pub destination_path: Option<String>,
}

/// Represents the configuration containing multiple backup configurations.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// The list of backup configurations.
    /// Each configuration specifies the destination backup filesystem or partition
    /// and the devices to be backed up on that filesystem.
    pub backups: Vec<BackUpConfig>,
    /// The path on which the destination filesystem will be mounted.
    /// If not provided, the default mount path will be used.
    pub mountpath: Option<String>,
}

impl Config {
    /// Creates a new `Config` instance by reading the configuration file.
    ///
    /// # Returns
    ///
    /// - `Ok(Config)`: If the configuration file is successfully read and parsed.
    /// - `Err(String)`: If there is an error reading or parsing the configuration file.
    pub fn new() -> Result<Config, String> {
        Self::read_config_file()
    }

    /// Reads the configuration file and returns a `HashMap` of destination devices to `BackUpConfig`.
    ///
    /// # Returns
    ///
    /// - `Ok(HashMap<String, BackUpConfig>)`: If the configuration file is successfully read and parsed.
    /// - `Err(String)`: If there is an error reading or parsing the configuration file.
    pub fn read_config_file() -> Result<Config, String> {
        match File::open(Self::config_file_path()?) {
            Ok(config_file) => {
                let parsed_config: Result<Config, _> = serde_json::from_reader(config_file);

                parsed_config.map_err(|e| format!("Cannot parse config file -> {}", e.to_string()))
            }
            Err(e) => Err(e.to_string()),
        }
    }

    /// Returns the path to the configuration file.
    ///
    /// # Returns
    ///
    /// - `Ok(PathBuf)`: The path to the configuration file if it exists.
    /// - `Err(String)`: If there is an error getting the configuration file path or the path doesn't exist.
    pub fn config_file_path() -> Result<PathBuf, String> {
        Ok(Self::config_home_path()
            .map_err(|e| format!("Failed reading or creating data directory -> {}", e))?
            .join("config.json"))
    }

    /// Returns the path to the home directory where the configuration file is located.
    /// Side effect: May create `~/.dd-back-up/` directory if it doesn't exist.
    ///
    /// # Returns
    ///
    /// - `Ok(PathBuf)`: The path to the home directory.
    /// - `Err(String)`: If there is an error getting the home directory path or creating the data directory.
    pub fn config_home_path() -> Result<PathBuf, String> {
        let data_dir = dirs::home_dir()
            .ok_or("Failed to find Home dir")?
            .join(".dd-back-up");

        if !data_dir.exists() {
            Self::create_data_directory(&data_dir)?;
        }

        Ok(data_dir)
    }

    /// Creates the data directory if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `data_dir` - The path to the data directory.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the data directory is successfully created or already exists.
    /// - `Err(String)`: If there is an error creating the data directory.
    fn create_data_directory(data_dir: &PathBuf) -> Result<(), String> {
        fs::create_dir(data_dir).map_err(|e| {
            format!(
                "Failed to create data directory at {}, Error -> {}",
                data_dir.to_string_lossy(),
                e
            )
        })
    }
}

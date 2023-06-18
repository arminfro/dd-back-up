use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{self, File},
    path::PathBuf,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupDevice {
    /// The serial number of the device.
    pub serial: String,
    /// An optional name for the device.
    pub name: Option<String>,
    /// The number of copies to be kept for this device.
    ///
    /// If set to `None`, only one copy will be kept.
    /// If set to a positive integer, the oldest copies will be deleted when the limit is reached.
    /// If set to 0, Config::validate_config will return Err(String).
    pub copies: Option<usize>,
}

/// Represents the configuration for a single backup.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupConfig {
    /// The list of devices to be backed up.
    ///
    /// Strings are identifiers of whole devices.
    /// The identifier can be the serial number or the wwn (world wide name).
    /// Since some devices may not have a serial number or even have duplicated serial numbers,
    /// the identifier serves as a unique identifier for the device.
    pub backup_devices: Vec<BackupDevice>,
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
    pub backups: Vec<BackupConfig>,
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
    pub fn new(config_file_path: &Option<String>) -> Result<Config, String> {
        trace!("some trace log");
        debug!("some debug log");
        info!("some information log");
        warn!("some warning log");
        error!("some error log");
        Self::validate_config(Self::read_config_file(config_file_path))
    }

    /// Reads the configuration file and returns a `HashMap` of destination devices to `BackUpConfig`.
    ///
    /// # Returns
    ///
    /// - `Ok(HashMap<String, BackUpConfig>)`: If the configuration file is successfully read and parsed.
    /// - `Err(String)`: If there is an error reading or parsing the configuration file.
    fn read_config_file(config_file_path: &Option<String>) -> Result<Config, String> {
        let config_file_path = match config_file_path {
            Some(path_string) => Ok(PathBuf::from(path_string)),
            None => Self::default_config_file_path(),
        }?;

        match File::open(&config_file_path) {
            Ok(config_file) => {
                let parsed_config: Result<Config, _> = serde_json::from_reader(config_file);

                parsed_config.map_err(|e| format!("Cannot parse config file -> {}", e.to_string()))
            }
            Err(e) => Err(format!(
                "{}: {}",
                e.to_string(),
                config_file_path.as_path().to_str().unwrap(),
            )),
        }
    }

    /// Validates the configuration to ensure unique UUIDs and serial numbers.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to validate.
    ///
    /// # Returns
    ///
    /// - `Ok(Config)`: If the configuration is valid.
    /// - `Err(String)`: If the configuration is not valid, with a descriptive error message.
    fn validate_config(config: Result<Config, String>) -> Result<Config, String> {
        let config = config?;

        // Check for unique UUIDs
        let uuids: HashSet<&String> = config.backups.iter().map(|backup| &backup.uuid).collect();
        if uuids.len() != config.backups.len() {
            return Err("Duplicate UUID found in backups".to_string());
        }

        for backup in &config.backups {
            // Check for unique serial numbers within each backup
            let serials: HashSet<&String> = backup
                .backup_devices
                .iter()
                .map(|device| &device.serial)
                .collect();
            if serials.len() != backup.backup_devices.len() {
                return Err(format!(
                    "Duplicate serial number found in backup with UUID '{}'",
                    backup.uuid
                ));
            }

            // Check if the number of copies is specified and greater than 0
            for device in &backup.backup_devices {
                if let Some(copies) = device.copies {
                    if copies <= 0 {
                        return Err(format!(
                        "Invalid number of copies for device with serial '{}'. Must be greater than 0.",
                        device.serial
                    ));
                    }
                }
            }
        }

        Ok(config)
    }

    /// Returns the default path to the configuration file.
    ///
    /// # Returns
    ///
    /// - `Ok(PathBuf)`: The path to the configuration file if it exists.
    /// - `Err(String)`: If there is an error getting the configuration file path or the path doesn't exist.
    pub fn default_config_file_path() -> Result<PathBuf, String> {
        Ok(Self::config_home_path()
            .map_err(|e| format!("Failed reading or creating data directory -> {}", e))?
            .join("config.json"))
    }

    /// Returns the path to the home directory where the configuration file is located.
    /// Side effect: May create `~/.config/dd_backup/` directory if it doesn't exist.
    ///
    /// # Returns
    ///
    /// - `Ok(PathBuf)`: The path to the home directory.
    /// - `Err(String)`: If there is an error getting the home directory path or creating the data directory.
    pub fn config_home_path() -> Result<PathBuf, String> {
        let data_dir = dirs::home_dir()
            .ok_or("Failed to find Home dir")?
            .join(".config")
            .join("dd_backup");

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

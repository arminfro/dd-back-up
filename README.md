# `dd_backup`

A command-line tool written in Rust that performs block device backups using the `dd` command.
It allows you to back up specific devices to a designated filesystem or partition.

## Features

- Creates automated backups of devices identified by serial numbers.
  - Supports multiple target filesystems, each capable of backing up multiple devices.
  - Configurable relative destination paths for backups on each target filesystem.
- Each device can have an optional `copies` field to maintain a fixed number of stored backups.
  - Ensures a consistent size of stored backups.
  - Automatically creates or deletes backup image files based on the configuration.
- Provides the ability to define another backup filesystem for the device on which your others backups are located.
  - Allows you to have a backup of your backup device.
- Safety features:
  - Dry run mode to simulate backup operations without making actual changes.
  - Checks for available space before starting the next backup.
  - Verifies uniqueness of UUIDs and serial numbers to avoid confusion.
  - Executes `sync` to flush data to disk before unmounting.
  - Performs filesystem check before writing any data on the target filesystem.
    - can be disabled, or overwritten with custom command
- Logging:
  - Supports different log levels (trace, debug, info, warn, error).
  - Color-coded log output for improved readability.
- Can be used on a USB stick with a Linux live system to back up any operating system.

## Installation

```shell
git clone https://github.com/arminfro/dd_backup.git && cd dd_backup && cargo install --path .
```

## Usage:

To use dd_backup, you need to configure the backup settings in a JSON configuration file.

### Configuration

The configuration file (`~/.config/dd_backup/config.json`) is used to specify the backup configurations. It has the following structure:

```json
{
  "mountpath": "/mnt",
  "backups": [
    {
      "uuid": "dst-back-up-fs-uuid-1",
      "destination_path": "./",
      "fsck_command": "fsck -n",
      "skip_fsck": false,
      "backup_devices": [
        {
          "serial": "device-serial-1",
          "name": "desktop"
          "copies": 2
        },
        {
          "serial": "device-serial-2",
          "name": "laptop"
        }
      ]
    },
    {
      ...
    }
  ]
}
```

- `mountpath`: The path on which the destination filesystem will be mounted. This path is used as the base directory for specifying the destination path of each backup.

  - Optional, defaults to "/mnt"

- `backups`: An array of backup configurations. Each configuration specifies a destination backup filesystem or partition and the devices to be backed up on that filesystem.

  - `uuid`: The UUID of the destination backup filesystem or partition.

    - obtain the uuid with tools like `lsblk -n -o NAME,UUID`

  - `destination_path`: The destination path where the backup will be stored. This path is relative to the mountpath. If not provided, the backup will be stored in the root of the mountpath.

    - Optional, defaults to "./"

    - _Note_: If you want to use subdirs, create them manually

  - `fsck_command`: Specifies an alternative command to execute for filesystem checks. The command should exit with a status code of 0 if all tests have passed. Only filesystems supported by the specified `fsck` command are supported without it.

    - Optional field. Defaults to `fsck -n` (read-only mode).

    - The command will have the target filesystem path appended, for example: `/dev/sda3`.

  - `skip_fsck`: Configures whether to skip the filesystem check (fsck) altogether.

    - Optional field. Defaults to `false`. If set to `true`, the filesystem check will be skipped.

  - `backup_devices`: An array of devices to be backed up on the destination filesystem. Each device is specified by its serial number and an optional name.

    - obtain the serial with tools like `lsblk -n -o NAME,SERIAL`

    - `copies`: The number of copies to be kept for this device. If specified, the oldest backup will be deleted when creating a new backup if the number of backups exceeds the specified count. If not specified, nothing will be deleted.

      - Optional, defaults to `None`.

      - _Note_: If you decrease the value of copies after a while, you may need to manually delete backup files until you have the desired number of copies. Otherwise, the program will continue to delete only one backup per run, which may result in the same count as before decreasing.

      - _Note_: To obtain the number of present copies the program will consider the values name, model and serial as common suffix for counting. If you want to keep a copy which will not be managed by the application append some value to the filename.

The program allows you to configure backups for all your backup devices, whether they are currently connected or not.
It checks for the presence of the filesystem and the device.
If either of them is not found, the corresponding pair will be skipped during the backup process.

### Running the Backup:

Once you have configured the backup settings, you can run the backup process by executing `dd_backup run`.

#### CLI Interface

```shell
Usage: dd_backup run [OPTIONS]

Options:
  -d, --dry
          performs a dry run, no dd operation, just to see the output
  -c, --config-file-path <CONFIG_FILE_PATH>
          pass in the path of the config file
```

The `run` command will mount the backup filesystem if necessary, perform the backup for each specified device, and finally unmount the filesystem.
The file will have a name like `2023-06-15_desktop_Micro-Line_10170080910002B1.img`, containing the date, the backup device name, the model and the serial.

Make sure to exercise caution when specifying the backup devices and the target filesystem/partition.
Use the `--dry` flag to see what devices would be backed up before running it.

#### Logging

To adjust the amount of log output, you can set the `RUST_LOG` environment variable to different levels such as `trace` or `debug` for more detailed output, or `warn` or `error` for less verbose output.

Here's an example command that runs the application with increased log output, saves the logs to a file, and also displays them on the command line:

```shell
RUST_LOG=debug dd_backup run 2>&1 | tee -a backup.log
```

License:
This project is licensed under the MIT license.

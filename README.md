# `dd_backup`

A command-line tool that performs block device backups using the `dd` command.
It allows you to back up specific devices to a designated filesystem.

## Features

- Creates automated backups of devices identified by serial numbers.
  - Supports multiple target filesystems, each capable of backing up multiple devices.
  - Configurable relative destination paths for backups on each target filesystem.
- Each device can have an optional `copies` field to maintain a fixed number of stored backups.
  - Ensures a consistent size of stored backups.
  - Automatically deletes oldest backup image file, if count exceeds.
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

## Why block device backups?

`dd_backup` operates at the block device level, offering a distinct approach to backups compared to many other backup tools that work at the file-system level.

When performing backups at the block device level, you will get a complete backup of the device, capturing all data, configurations, and file systems as they are, without any abstraction or modification.
This comes at the cost of more time and more storage space on creating, since all sectors are captured including empty ones.

The main reason why you may like to have a block device backup is backing up the operating system with all partition and storage configurations in the same way as your other data.
In the event of a sudden hard drive failure, you can quickly restore the entire block device, enabling a faster recovery process of your OS device.

It's even possible to combine this approach with backup tools operating on the file system layer, since you can use a loopback device to read and write the image file, created by `dd`.
You can run `dd_backup` once to create the initial block device backup and then use the resulting image file for restoration or for subsequent file-system level backups, capturing specific changes and modifications.

In my personal use case, I run `dd_backup` before going to sleep, as the creation time is not a concern for me.
I employ it on a bootable live Linux system that is configured to automatically run `dd_backup` upon boot.
This setup enables me to back up my home server, which lacks a display or keyboard, without the need to physically move the server and its external storage devices to my desk.
Additionally, I have two physical hard drives in my main computer, one for my Linux system and the other for my Windows system, both sharing a boot partition.
In the worst-case scenario of one hard drive failure two operating systems are not bootable.

## Installation

```shell
cargo install dd_backup
```

## Usage:

To use dd_backup, you can configure the backup settings in a JSON configuration file or use options for single back up only.

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
      "skip_mount": false,
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
      "uuid": "dst-back-up-fs-uuid-2",
      "backup_devices": [{ "serial": "device-serial-3" }]
    },
    {
      ...
    }
  ]
}
```

- `mountpath`: The path on which the destination filesystem will be mounted. This path is used as the base directory for specifying the destination path of each backup.

  - Optional, defaults to "/mnt"

- `backups`: An array of backup configurations. Each configuration specifies a destination backup filesystem and the devices to be backed up on that filesystem.

  - `uuid`: The UUID of the destination backup filesystem.

    - obtain the uuid with tools like `lsblk -n -o NAME,UUID`

  - `destination_path`: The destination path where the backup will be stored. This path is relative to the mountpath. If not provided, the backup will be stored in the root of the mountpath.

    - Optional, defaults to "./"

    - _Note_: If you want to use subdirs, create them manually

  - `fsck_command`: Specifies an alternative command to execute for filesystem checks. The command should exit with a status code of 0 if all tests have passed. Only filesystems supported by the specified `fsck` command are supported without it.

    - Optional field. Defaults to `fsck -n` (read-only mode).

    - The command will have the target filesystem path appended, for example: `/dev/sda3`.

  - `skip_fsck`: Configures whether to skip the filesystem check (fsck) altogether.

    - Optional field. Defaults to `false`. If set to `true`, the filesystem check will be skipped.

  - `skip_mount`: Configures whether to mount the filesystem or not.

    - Optional field. Defaults to `false`. If set to `true`, the filesystem won't be mounted. Use it if your filesystem is already mounted and should remain mounted after the backup process. Sets `skip_fsck` to `true`.

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

Make sure to **exercise caution** when specifying the backup devices and the target filesystem/partition.
Use the `--dry-run` flag to see what devices would be backed up before running it.

#### CLI Interface

Options with no short flag are only for single backup execution.

```shell
Usage: dd_backup run [OPTIONS]

Options:
  -n, --dry-run
          Performs a dry run, simulating backup operations without making any changes [default: "false"]
  -c, --config-file-path <CONFIG_FILE_PATH> [default: "~/.config/dd_backup.json"]
          The path to the configuration file
      --destination-uuid <DESTINATION_UUID>
          The UUID of the destination backup filesystem, single-back-up-only
      --source-serial <SOURCE_SERIAL>
          The serial number of the source device to be backed up, single-back-up-only
      --destination-path <DESTINATION_PATH>
          The destination path where the backup will be stored, single-back-up-only [default: ./]
      --copies <COPIES>
          The number of backup copies to maintain, single-back-up-only
      --name <NAME>
          The name of the backup, single-back-up-only
      --fsck-command <FSCK_COMMAND>
          Alternative command to perform filesystem check (`fsck -n`), single-back-up-only [default: "fsck -n"]
      --skip-fsck
          Flag to skip filesystem check (`fsck`), single-back-up-only [default: "false"]
      --skip-mount
          Flag to skip mounting, single-back-up-only [default: "false"]
  -m, --mountpath <MOUNTPATH>
          The mount path of the destination filesystem, overwrites config value [default: "/mnt"]
  -h, --help
          Print help
  -V, --version
          Print version
```

The `run` command will mount the backup filesystem if necessary, perform the backups for each specified device, and finally unmount the filesystem (if not configured otherwise).

The file will have a name like `2023-06-15_desktop_Micro-Line_10170080910002B1.img`, containing the date, the backup device name, the model and the serial.

##### Performing Single Backup

There are also options available for performing a single backup. These options are useful if you want to trigger a specific backup process with cron jobs, or if you have a card reader and want to back up different SD cards with individual names.

When using the single-backup options, it is necessary to specify the source serial number and destination UUID for the specific backup operation.
You can also provide any other configurable option for a backup device defined in the `backup_devices` array.

These options are not allowed in conjunction with the config file option (`-c, --config-file-path`), as they are intended for one-time backup scenarios. Also the default config file is not picked up when using it.

#### Logging

To adjust the amount of log output, you can set the `RUST_LOG` environment variable to different levels such as `trace` or `debug` for more detailed output, or `warn` or `error` for less verbose output.

Here's an example command that runs the application with increased log output, saves the logs to a file, and also displays them on the command line:

```shell
RUST_LOG=debug dd_backup run 2>&1 | tee -a backup.log
```

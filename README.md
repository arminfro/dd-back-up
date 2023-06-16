`dd-back-up` is a command-line tool written in Rust that performs block device backups using the `dd` command.
It allows you to back up specific devices to a designated filesystem or partition on a Linux system.

## Installation

```shell
git clone https://github.com/arminfro/dd-back-up.git && cd dd-back-up && cargo install --path .
```

## Usage:

To use dd-back-up, you need to configure the backup settings in a JSON configuration file.

### Configuration

The configuration file (`~/.config/dd-back-up/config.json`) is used to specify the backup configurations. It has the following structure:

```json
{
  "mountpath": "/mnt",
  "backups": [
    {
      "uuid": "dst-back-up-fs-uuid-1",
      "destination_path": "./",
      "back_up_devices": [
        {
          "serial": "device-serial-1",
          "name": "desktop"
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

  - `back_up_devices`: An array of devices to be backed up on the destination filesystem. Each device is specified by its serial number and an optional name.

    - obtain the serial with tools like `lsblk -n -o NAME,SERIAL`

The program allows you to configure backup for all devices, whether they are currently connected or not. It checks for the presence of the filesystem and the device. If either of them is not found, the corresponding pair will be skipped during the backup process.

### Running the Backup:

Once you have configured the backup settings, you can run the backup process by executing `dd-back-up run`.

```shell
Usage: dd-back-up run [OPTIONS]

Options:
  -d, --dry
          performs a dry run, no dd operation, just to see the output
  -c, --config-file-path <CONFIG_FILE_PATH>
          pass in the path of the config file
```

The "run" command will mount the backup filesystem if necessary, perform the backup for each specified device, and finally unmount the filesystem.
The file will have a name like `2023-06-15_desktop_Micro-Line_10170080910002B1.img`, containing the date, the backup device name, the model and the serial.

Make sure to exercise caution when specifying the backup devices and the target filesystem/partition.
Use the `--dry` flag to see what devices would be backed up before running it.

License:
This project is licensed under the MIT license.

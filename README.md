`dd-back-up` is a command-line tool written in Rust that performs block device backups using the `dd` command.
It allows you to back up specific devices to a designated filesystem or partition.

## Usage:

To use dd-back-up, you need to configure the backup settings in a JSON configuration file located at `~/.dd-back-up/config.json`.
The configuration file should have the following structure:

```json
{
  "83d1b338-d7c8-48cd-b081-c4eb10948414": {
    "back_up_devices": ["10270080900002B1", "NM13P0Y3"]
  },
  ...
  "some-uuid": { "back_up_devices": ["some-serial"] },
}
```

- "83d1b338-d7c8-48cd-b081-c4eb10948414" represents the UUID (Universally Unique Identifier) of the partition or filesystem where the backup should be saved.
  - You can obtain the UUID using utilities such as `lsblk -n -o NAME,UUID`.
- "back_up_devices" is an array of identifiers representing the devices to be backed up. Each identifier corresponds to a specific device that you want to backup. In the example above, "10270080900002B1" and "NM13P0Y3" are used as identifiers.
  - You can find the serial number of a device on the device itself or use tools like `lsblk -n -o NAME,SERIAL` to list devices with their serial numbers.

You can configure multiple backup devices for a single backup filesystem by adding additional entries under different UUIDs in the configuration file.

The program allows you to configure backup for all devices, whether they are currently connected or not. It checks for the presence of the filesystem and the device. If either of them is not found, the corresponding pair will be skipped during the backup process.

### Running the Backup:

Once you have configured the backup settings, you can run the backup process by executing the following command:

```shell
dd-back-up run [--dry]
```

The "back-up" command will read the configuration file, mount the backup filesystem if necessary, perform the backup for each specified device, and finally unmount the filesystem.
The backup will be saved in the root directory of the mounted filesystem. The file will have a name like `Micro_Line-10170080900002B1-2023-06-15.img`, containing model, serial and date.

Please note that the dd command is used under the hood to perform the actual backup. Make sure to exercise caution when specifying the backup devices and the target filesystem/partition and use the `--dry` flag to see what devices would be backed up before running it.

License:

This project is licensed under the MIT license.

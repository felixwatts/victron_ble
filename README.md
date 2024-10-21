# Victron BLE

Read data from Victron devices over Bluetooth Low Energy.

Some Victron devices use the BLE advertising data protocol to broadcast some aspects of their state on a regular basis. This crate enables you to easily decrypt and parse the broadcast data into usable form.

Currently only Solar Charger type devices are supported but support can be added for other device types if requested.

# Usage

 If you already have the manufacturer data from your Victron device you can use
 the basic function `parse_manufacturer_data` to decrypt and parse it.
 
 ```rust
let device_encryption_key = hex::decode("Victron device encryption key").unwrap();
// Sourced from the manufacturer data part of a BLE advertisement event
// The Victron manufacturer ID is 737
let device_manufacturer_data = [0x10, 0, 0, 0, 0 ]; 

let device_state_result = victron_ble::parse_manufacturer_data(&device_manufacturer_data, &device_encryption_key);

println!("{device_state_result:?}");
```

If you want the crate to handle the bluetooth side, including discovering the 
device and receiving the manufacturer data then enable the `bluetooth` feature and
use the `open_stream` function which currently supports MacOS and Linux.
 
 ```rust
let device_name = "Victon Bluetooth device name";
let device_encryption_key = hex::decode("Victron device encryption key").unwrap();

let mut device_state_stream = victron_ble::open_stream(
    device_name, 
    device_encryption_key
).await;

while let Some(result) = device_state_stream.next().await {
    println!("{result:?}");
}
 ```
 
 # Encryption Key
 
 The device status messages published by the Victron device are encrypted. In order
 to decrypt them the device encyption key is needed. This can be found for a given
 device using the Victron Connect app on iOS or Android.
 
 Using the app, connect to the device, then go to Settings -> Product Info -> Encryption data.
 
 # Serialization
 
 If you add the `serde` feature then the `DeviceState` enum will be (de)serializable.
 
 # Ackowledgements
 
 Various aspects of this crate are either inspired by or copied from these
 projects:
 
 - <https://github.com/keshavdv/victron-ble>
 - <https://github.com/PeterGrace/vedirect_rs>

## Sources

https://communityarchive.victronenergy.com/questions/187303/victron-bluetooth-advertising-protocol.html

And see the file is this repo: `extra-manufacturer-data-2022-12-14.pdf`




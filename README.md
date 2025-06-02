# Victron BLE

Read data from Victron devices over Bluetooth Low Energy.

Some Victron devices broadcast some aspects of their state over Bluetooth on a regular basis. This crate makes it easy to access that data.

## Basic Usage

Use the `open_stream` function to get a stream of state updates for a given
Victron device:

```rust
let device_name = "Victron Bluetooth device name";
let device_encryption_key = hex::decode("Victron device encryption key").unwrap();

let mut device_state_stream = victron_ble::open_stream(
    device_name, 
    device_encryption_key
).unwrap();

while let Some(result) = device_state_stream.next().await {
    println!("{result:?}");
}
```

## Encryption Key

The device status messages published by the Victron device are encrypted. In order
to decrypt them the device encryption key is needed. This can be found for a given
device using the Victron Connect app on iOS or Android.

Using the app, connect to the device, then go to Settings -> Product Info -> Encryption data.

## Supported Device Types

Currently the following device types are supported.

> Support can be added for other device types if requested.

- Solar Charger
- Battery Monitor
- Inverter

## Serialization

If you add the `serde` feature then the `DeviceState` enum will be (de)serializable.

## Example

An example application is provided which prints the state of a given device to to the terminal.

```
cargo run --example example <Victron device name> <Victron device encryption key>
```

## Acknowledgements

Various aspects of this crate are either inspired by or copied from these
projects:

- <https://github.com/keshavdv/victron-ble>
- <https://github.com/PeterGrace/vedirect_rs>




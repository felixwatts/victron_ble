# Victron BLE

Read data from Victron devices over Bluetooth Low Energy.

Some Victron devices broadcast some aspects of their state over Bluetooth on a regular basis. This crate makes it easy to access that data.

## Basic Usage

Use the `open_stream` function to get a stream of state updates for a given
Victron device:

```rust
use std::{println, time::Duration};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let device_name = "Victron Bluetooth device name".into();
    let device_encryption_key = hex::decode("00"/* Victron device encryption key. See below. */).unwrap();

    let mut device_state_stream = victron_ble::open_stream(
        device_name,
        device_encryption_key
    ).unwrap();

    while let Some(result) = device_state_stream.next().await {
        println!("{result:?}");
    }
}
```

## Device Setup

In order to turn on the Victron device's BLE state broadcasts you must enable the "Instant Readout"
setting. This can be done via the Victron Connect App on iOS or Android.

## Encryption Key

The device status messages published by the Victron device are encrypted. In order
to decrypt them the device encryption key is needed. This can be found for a given
device using the Victron Connect app on iOS or Android.

Using the app, connect to the device, then go to Settings -> Product Info -> Encryption data.

## Supported Device Types

Currently the following device types are supported. **Support can be added for other device types if requested.**

- Solar Charger
- Battery Monitor
- Inverter
- VE Bus
- AC Charger

## Features

### `bluetooth`

Adds the `open_stream` function which handles all of the bluetooth discovery and receiving but is only supported for the `macos` and `linux` targets. With the `bluetooth` feature off you still get the `parse_manufacturer_data` function but you must source your own manufacturer data packet. `bluetooth` is a default feature.

### `serde`

Makes the `DeviceState` enum (de)serializable.

## no_std

If you turn the `bluetooth` feature off then the crate can be compiled in a `no_std` context.

## Example

An example application is provided which prints the state of a given device to the terminal.

```bash
cargo run --example bluetooth <Victron device name> <Victron device encryption key>
```

## Acknowledgements

Various aspects of this crate are either inspired by or copied from these
projects:

- <https://github.com/keshavdv/victron-ble>
- <https://github.com/PeterGrace/vedirect_rs>




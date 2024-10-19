//! Read data from Victron devices over Bluetooth Low Energy
//!
//! Some Victron devices publish status data continuously over Bluetooth using
//! BLE advertising data protocol. This crate makes it easy to access that data.
//! 
//! If you already have the manufacturer data from your Victron device you can use
//! the basic function `parse_manufacturer_data` to decrypt and parse it.
//! 
//! ```rust
//! # use std::{println, time::Duration};
//! #
//! # #[tokio::main]
//! # async fn main() {
//!     let device_encryption_key = hex::decode("Victron device encryption key").unwrap();
//!     // Sourced from the manufacturer data part of a BLE advertisement event
//!     let device_manufacturer_data = [0x10, 0, 0, 0, 0 ]; 
//! 
//!     let device_state_result = victron_ble::parse_manufacturer_data(&device_manufacturer_data, &device_encryption_key);
//! 
//!     println!("{device_state_result:?}");
//! # }
//! ```
//! 
//! If you want the crate to handle the bluetooth side, including discovering the 
//! device and receiving the manufacturer data then use the `open_stream` function
//! which currently supports MacOS and Linux.
//! 
//! ```rust
//! # use std::{println, time::Duration};
//! #
//! # #[tokio::main]
//! # async fn main() {
//!     let device_name = "Victon Bluetooth device name";
//!     let device_encryption_key = hex::decode("Victron device encryption key").unwrap();
//! 
//!     let mut device_state_stream = victron_ble::open_stream(
//!         device_name, 
//!         &device_encryption_key
//!     ).await;
//! 
//!     while let Some(result) = device_state_stream.recv().await {
//!         println!("{result:?}");
//!     }
//! # }
//! ```
//! 
//! # Encryption Key
//! 
//! The device status messages published by the Victron device are encrypted. In order
//! to decrypt them the device encyption key is needed. This can be found for a given
//! device using the Victron Connect app on iOS or Android.
//! 
//! Using the app, connect to the device, then go to Settings -> Product Info -> Encryption data.
//! 
//! # Serialization
//! 
//! If you add the `serde` feature then the `DeviceState` enum will be (de)serializable.
//! 
//! # Ackowledgements
//! 
//! Various aspects of this crate are either inspired by or copied from these
//! projects:
//! 
//! - <https://github.com/keshavdv/victron-ble>
//! - <https://github.com/PeterGrace/vedirect_rs>

mod bit_reader;
mod record;
mod err;
mod model;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

pub use model::*;
pub use crate::err::*;

#[cfg(target_os = "linux")]
pub use linux::open_stream;
#[cfg(target_os = "macos")]
pub use macos::open_stream;

use record::Record;

/// Decrypyt and parse the content of the manufacturer data published by a Victron device.
pub fn parse_manufacturer_data(manufacturer_data: &[u8], device_encryption_key: &[u8]) -> Result<DeviceState> {
    let record = Record::new(manufacturer_data, device_encryption_key)?;
    DeviceState::parse(&record)
}

//! # Victron BLE
//!
//! Read data from Victron devices over Bluetooth Low Energy.
//!
//! Some Victron devices broadcast some aspects of their state over Bluetooth on a regular basis. This crate makes it easy to access that data.
//!
//! Currently only Solar Charger type devices are supported but support can be added for other device types if requested.
//!
//! ## Basic Usage
//!
//! Use the `open_stream` function to get a stream of state updates for a given
//! Victron device:
//!
//! ```rust
//! # use std::{println, time::Duration};
//! # use tokio_stream::StreamExt;
//! #
//! # #[tokio::main]
//! # async fn main() {
//!     let device_name = "Victon Bluetooth device name";
//!     let device_encryption_key = hex::decode("Victron device encryption key").unwrap();
//!
//!     let mut device_state_stream = victron_ble::open_stream(
//!         device_name,
//!         device_encryption_key
//!     ).unwrap();
//!
//!     while let Some(result) = device_state_stream.next().await {
//!         println!("{result:?}");
//!     }
//! # }
//! ```
//!
//! ## Encryption Key
//!
//! The device status messages published by the Victron device are encrypted. In order
//! to decrypt them the device encyption key is needed. This can be found for a given
//! device using the Victron Connect app on iOS or Android.
//!
//! Using the app, connect to the device, then go to Settings -> Product Info -> Encryption data.
//!
//! ## Serialization
//!
//! If you add the `serde` feature then the `DeviceState` enum will be (de)serializable.
//!
//! ## Example
//!
//! An example application is provided which prints the state of a given device to to the terminal.
//!
//! ```
//! cargo run --example example <Victron device name> <Victron device encryption key>
//! ```
//!
//! ## Ackowledgements
//!
//! Various aspects of this crate are either inspired by or copied from these
//! projects:
//!
//! - <https://github.com/keshavdv/victron-ble>
//! - <https://github.com/PeterGrace/vedirect_rs>

mod bit_reader;
mod err;
mod linux;
mod macos;
mod model;
mod record;

pub use crate::err::*;
pub use model::*;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::{wrappers::UnboundedReceiverStream, Stream};

#[cfg(target_os = "linux")]
use linux::open_stream as _open_stream;
#[cfg(target_os = "macos")]
use macos::open_stream as _open_stream;

use record::Record;

/// Decrypyt and parse the content of the manufacturer data published by a Victron device.
pub fn parse_manufacturer_data(
    manufacturer_data: &[u8],
    device_encryption_key: &[u8],
) -> Result<DeviceState> {
    let record = Record::new(manufacturer_data, device_encryption_key)?;
    DeviceState::parse(&record)
}

/// Continuously monitor device state.
///
/// Will attempt to discover the named device, then continuously listen for device state
/// bluetooth broadcasts which will each be decrypted, parsed and sent to the user
/// via a stream.
///
/// # Example
///
///  ```rust
/// # use std::{println, time::Duration};
/// # use tokio_stream::StreamExt;
/// #
/// # #[tokio::main]
/// # async fn main() {
///     let device_name = "Victon Bluetooth device name";
///     let device_encryption_key = hex::decode("Victron device encryption key").unwrap();
///
///     let mut device_state_stream = victron_ble::open_stream(
///         device_name,
///         device_encryption_key
///     ).unwrap();
///
///     while let Some(result) = device_state_stream.next().await {
///         println!("{result:?}");
///     }
/// # }
/// ```
pub fn open_stream(
    device_name: String,
    device_encryption_key: Vec<u8>,
) -> Result<impl Stream<Item = Result<DeviceState>>> {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        let _ = _open_stream(device_name, device_encryption_key, sender.clone()).await;
    });

    Ok(UnboundedReceiverStream::new(receiver))
}

fn handle_manufacturer_data(
    manufacturer_data: &[u8],
    device_encryption_key: &[u8],
    sender: &mut UnboundedSender<Result<DeviceState>>,
) -> Result<()> {
    let device_state_result = parse_manufacturer_data(manufacturer_data, device_encryption_key);

    match device_state_result {
        Err(Error::WrongAdvertisement) => Ok(()), // Message irrelevant to user, wait for next advertisement
        Err(e) => {
            // Report error to user
            let _ = sender.send(Err(e.clone()));
            // Fatal error, stop
            Err(e)
        }
        Ok(device_state) => {
            let _ = sender.send(Ok(device_state));
            // If consumer has dropped the channel then stop
            Err(Error::ClientClosedChannel)
        }
    }
}

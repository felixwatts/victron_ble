#![cfg_attr(not(feature = "bluetooth"), no_std)]
#![doc = include_str!("../README.md")]

mod bit_reader;
mod bluetooth;
mod err;
mod model;
mod record;

use record::Record;

pub use err::*;
pub use model::*;

#[cfg(feature = "bluetooth")]
pub use bluetooth::open_stream;

/// Decrypt and parse the content of the manufacturer data published by a Victron device.
pub fn parse_manufacturer_data(
    manufacturer_data: &[u8],
    device_encryption_key: &[u8],
) -> Result<DeviceState> {
    let record = Record::new(manufacturer_data, device_encryption_key)?;
    DeviceState::parse(&record)
}

#![cfg_attr(not(feature = "bluetooth"), no_std)]
#![doc = include_str!("../README.md")]

mod bit_reader;
mod bluetooth;
mod err;
mod model;
mod record;

pub use crate::err::*;
#[cfg(feature = "bluetooth")]
pub use bluetooth::open_stream;
pub use model::*;
use record::Record;

/// Decrypt and parse the content of the manufacturer data published by a Victron device.
pub fn parse_manufacturer_data(
    manufacturer_data: &[u8],
    device_encryption_key: &[u8],
) -> Result<DeviceState> {
    let record = Record::new(manufacturer_data, device_encryption_key)?;
    DeviceState::parse(&record)
}

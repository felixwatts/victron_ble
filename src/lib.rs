mod bit_reader;
mod record;
mod err;
mod model;

pub use model::DeviceState;
pub use crate::err::*;

use record::Record;

pub fn parse_manufacturer_data(manufacturer_data: &[u8], device_encryption_key: &[u8]) -> Result<DeviceState> {
    let record = Record::new(manufacturer_data, device_encryption_key)?;
    DeviceState::parse(&record)
}

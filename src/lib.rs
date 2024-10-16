mod bit_reader;
mod record;
mod err;
mod model;

use std::collections::HashMap;
use model::DeviceState;
use record::Record;
use crate::err::*;

pub fn parse_manufacturer_data(manufacturer_data: &HashMap<u16, Vec<u8>>, device_encryption_key: &[u8]) -> Result<DeviceState> {
    let record = Record::new(manufacturer_data, device_encryption_key)?;
    DeviceState::parse(&record)
}

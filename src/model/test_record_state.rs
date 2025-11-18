use crate::bit_reader::BitReader;
use crate::err::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct TestRecordState {
    pub uptime_s: Option<f32>,
    pub temperature_c: Option<f32>,
}

impl TestRecordState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);
        let uptime_s = reader.read_unsigned_field(30, 0x3FFFFFFF, 1.0, 0.0)?;
        let temperature_c = reader.read_unsigned_field(7, 0xFF, 1.0, -40.0)?;

        Ok(Self {
            uptime_s,
            temperature_c,
        })
    }
}

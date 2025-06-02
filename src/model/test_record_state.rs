use crate::bit_reader::BitReader;
use crate::err::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TestRecordState {
    pub uptime_s: u64,
    pub temperature_c: i64,
}

impl TestRecordState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);
        let uptime_s = reader.read_unsigned_int(30)?;
        let temperature_c = (reader.read_unsigned_int(7)? as i64) - 40;

        Ok(Self {
            uptime_s,
            temperature_c,
        })
    }
}

use super::error_state::ErrorState;
use super::Mode;
use crate::bit_reader::BitReader;
use crate::err::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AcChargerState {
    pub device_error: Mode,
    pub charger_error: ErrorState,
    pub battery_voltage_1: f32,
    pub battery_current_1: f32,
    pub battery_voltage_2: f32,
    pub battery_current_2: f32,
    pub battery_voltage_3: f32,
    pub battery_current_3: f32,
}

impl AcChargerState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let device_error = Mode::try_from(reader.read_unsigned_int(8)?)?;
        let charger_error = ErrorState::try_from(reader.read_unsigned_int(8)?)?;
        let battery_voltage_1 = reader.read_signed_int(13)? as f32 / 100.0;
        let battery_current_1 = reader.read_signed_int(11)? as f32 / 10.0;
        let battery_voltage_2 = reader.read_signed_int(13)? as f32 / 100.0;
        let battery_current_2 = reader.read_signed_int(11)? as f32 / 10.0;
        let battery_voltage_3 = reader.read_signed_int(13)? as f32 / 100.0;
        let battery_current_3 = reader.read_signed_int(11)? as f32 / 10.0;

        Ok(Self {
            device_error,
            charger_error,
            battery_voltage_1,
            battery_current_1,
            battery_voltage_2,
            battery_current_2,
            battery_voltage_3,
            battery_current_3,
        })
    }
}

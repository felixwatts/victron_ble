use crate::bit_reader::BitReader;
use crate::err::*;

use super::error_state::ErrorState;
use super::mode::Mode;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SolarChargerState {
    pub mode: Mode,
    pub error_state: ErrorState,
    pub battery_voltage_v: Option<f32>,
    pub battery_current_a: Option<f32>,
    pub yield_today_kwh: Option<f32>,
    pub pv_power_w: Option<f32>,
    pub load_current_a: Option<f32>,
}

impl SolarChargerState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let mode = Mode::try_from(reader.read_unsigned_int(8)?)?;
        let error_state = ErrorState::try_from(reader.read_unsigned_int(8)?)?;
        let battery_voltage_v = reader.read_signed_field(16, 0x7FFF, 0.01)?;
        let battery_current_a = reader.read_signed_field(16, 0x7FFF, 0.1)?;
        let yield_today_kwh = reader.read_unsigned_field(16, 0xFFFF, 0.01, 0.0)?;
        let pv_power_w = reader.read_unsigned_field(16, 0xFFFF, 1.0, 0.0)?;
        let load_current_a = reader.read_unsigned_field(9, 0x1FF, 0.1, 0.0)?;

        Ok(Self {
            mode,
            error_state,
            battery_voltage_v,
            battery_current_a,
            yield_today_kwh,
            pv_power_w,
            load_current_a,
        })
    }
}

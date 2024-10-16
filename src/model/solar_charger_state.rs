use crate::bit_reader::BitReader;
use crate::err::*;

use super::error_state::ErrorState;
use super::mode::Mode;

#[derive(Debug)]
pub struct SolarChargerState{
    pub mode: Mode,
    pub error_state: ErrorState,
    pub battery_voltage_v: f32,
    pub battery_current_a: f32,
    pub yield_today_kwh: f32,
    pub pv_power_w: f32,
    pub load_current_a: f32
}

impl SolarChargerState{
    pub (crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let mode = Mode::try_from(reader.read_unsigned_int(8)?)?;
        let error_state = ErrorState::try_from(reader.read_unsigned_int(8)?)?;
        let battery_voltage_v = (reader.read_signed_int(16)? as f32) / 100.0;
        let battery_current_a = (reader.read_signed_int(16)? as f32) / 10.0;
        let yield_today_kwh = (reader.read_unsigned_int(16)? as f32) / 100.0;
        let pv_power_w = reader.read_unsigned_int(16)? as f32;
        let load_current_a = (reader.read_unsigned_int(9)? as f32) / 10.0;

        Ok(
            Self{
                mode,
                error_state,
                battery_voltage_v,
                battery_current_a,
                yield_today_kwh,
                pv_power_w,
                load_current_a
            }
        )
    }
}
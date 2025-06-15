use super::alarm_reason::AlarmReason;
use super::Mode;
use crate::bit_reader::BitReader;
use crate::err::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InverterState {
    pub mode: Mode,
    pub alarm_reason: AlarmReason,
    pub battery_voltage_v: f32,
    pub ac_apparent_power_va: f32,
    pub ac_voltage_v: f32,
    pub ac_current_a: f32,
}

impl InverterState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let mode = Mode::try_from(reader.read_unsigned_int(8)?)?;
        let alarm_reason = AlarmReason::from_bits(reader.read_signed_int(16)?)
            .ok_or(Error::InvalidAlarmReason)?;
        let battery_voltage_v = reader.read_signed_int(16)? as f32 / 100.0;
        let ac_apparent_power_va = reader.read_unsigned_int(16)? as f32;
        let ac_voltage_v = reader.read_unsigned_int(15)? as f32 / 100.0;
        let ac_current_a = reader.read_unsigned_int(11)? as f32 / 10.0;

        Ok(Self {
            mode,
            alarm_reason,
            battery_voltage_v,
            ac_apparent_power_va,
            ac_voltage_v,
            ac_current_a,
        })
    }
}

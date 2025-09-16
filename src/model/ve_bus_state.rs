use super::error_state::ErrorState;
use super::mode::Mode;
use crate::bit_reader::BitReader;
use crate::err::*;
use num_enum::TryFromPrimitive;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct VeBusState {
    pub mode: Mode,
    pub error: ErrorState,
    pub battery_voltage_v: f32,
    pub battery_current_a: f32,
    pub ac_in_state: AcInState,
    pub ac_in_power_w: f32,
    pub ac_out_power_w: f32,
    pub alarm: AlarmNotification,
    pub battery_temperature_c: f32,
    pub soc_percent: f32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum AcInState {
    AcIn1 = 0,
    AcIn2 = 1,
    NotConnected = 2,
    Unknown = 3,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum AlarmNotification {
    NoAlarm = 0,
    Warning = 1,
    Alarm = 2,
    NotApplicable = 3,
}

impl VeBusState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let mode = Mode::try_from(reader.read_unsigned_int(8)?)?;
        let error = ErrorState::try_from(reader.read_unsigned_int(8)?)?;
        let battery_current_a = (reader.read_signed_int(16)? as f32) / 10.0;
        let battery_voltage_v = (reader.read_unsigned_int(14)? as f32) / 100.0;
        let ac_in_state = AcInState::try_from(reader.read_unsigned_int(2)? as u8)
            .ok()
            .ok_or(Error::InvalidAcInState)?;
        let ac_in_power_w = reader.read_signed_int(19)? as f32;
        let ac_out_power_w = reader.read_signed_int(19)? as f32;
        let alarm = AlarmNotification::try_from(reader.read_unsigned_int(2)? as u8)
            .ok()
            .ok_or(Error::InvalidAlarmNotification)?;
        let battery_temperature_c = reader.read_unsigned_int(7)? as f32 - 40.0;
        let soc_percent = reader.read_unsigned_int(7)? as f32;

        Ok(Self {
            mode,
            error,
            battery_voltage_v,
            battery_current_a,
            ac_in_state,
            ac_in_power_w,
            ac_out_power_w,
            alarm,
            battery_temperature_c,
            soc_percent,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Raw: [05, 00, 16, 00, 46, 05, 2f, 00, 00, 00, 00, c2, ff, c1, 16, 11]
    // Expected values (from example app):
    // - Mode: Float (5)
    // - Error: No Error
    // - Battery: 13.49V, 2.2A
    // - AC Output: 2W
    // - Temperature: 26°C
    #[test]
    fn test_ve_bus_parse() {
        let test_data = [
            0x05, 0x00, 0x16, 0x00, 0x46, 0x05, 0x2f, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xff, 0xc1,
            0x16, 0x11,
        ];

        let result = VeBusState::parse(&test_data).unwrap();

        assert_eq!(result.mode, Mode::Float);
        assert_eq!(result.error, ErrorState::NoError);

        assert!((result.battery_voltage_v - 13.50).abs() < 0.1);

        assert!((result.battery_current_a - 2.2).abs() < 0.1);

        assert_eq!(result.ac_in_state, AcInState::AcIn1);

        assert!((result.battery_temperature_c - 26.0).abs() < 2.0);

        assert_eq!(result.alarm, AlarmNotification::NoAlarm);
    }
}

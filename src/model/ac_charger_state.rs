use crate::bit_reader::BitReader;
use crate::err::*;

use super::error_state::ErrorState;
use super::mode::Mode;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AcChargerState {
    pub mode: Mode,
    pub error_state: ErrorState,
    pub battery_voltage1_v: f32,
    pub battery_current1_a: f32,
    pub battery_voltage2_v: f32,
    pub battery_current2_a: f32,
    pub battery_voltage3_v: f32,
    pub battery_current3_a: f32,
    pub temperature_c: f32,
    pub ac_current_a: f32,
}

impl AcChargerState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let mode = Mode::try_from(reader.read_unsigned_int(8)?)?;
        let error_state = ErrorState::try_from(reader.read_unsigned_int(8)?)?;
        let battery_voltage1_v = (reader.read_signed_int(13)? as f32) / 100.0;
        let battery_current1_a = (reader.read_signed_int(11)? as f32) / 10.0;
        let battery_voltage2_v = (reader.read_signed_int(13)? as f32) / 100.0;
        let battery_current2_a = (reader.read_signed_int(11)? as f32) / 10.0;
        let battery_voltage3_v = (reader.read_signed_int(13)? as f32) / 100.0;
        let battery_current3_a = (reader.read_signed_int(11)? as f32) / 10.0;
        let temperature_c = (reader.read_signed_int(7)? as f32) + 40.0;
        let ac_current_a = (reader.read_unsigned_int(9)? as f32) / 10.0;

        Ok(Self {
            mode,
            error_state,
            battery_voltage1_v,
            battery_current1_a,
            battery_voltage2_v,
            battery_current2_a,
            battery_voltage3_v,
            battery_current3_a,
            temperature_c,
            ac_current_a,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Raw: [0x04, 0x00, 0xA0, 0x05, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1B, 0xB9, 0x2F]
    // As recorded and checked on Blue Smart IP22 Charger
    #[test]
    fn test_ac_charger_state_parse_1() {
        let test_data = [
            0x04, 0x00, 0xA0, 0x05, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1B,
            0xB9, 0x2F,
        ];

        let result = AcChargerState::parse(&test_data).unwrap();

        assert_eq!(result.mode, Mode::Absorption);
        assert_eq!(result.error_state, ErrorState::NoError);

        assert!((result.battery_voltage1_v - 14.40).abs() < f32::EPSILON);
        assert!((result.battery_current1_a - 0.0).abs() < f32::EPSILON);
        assert!((result.battery_voltage2_v - -0.01).abs() < f32::EPSILON);
        assert!((result.battery_current2_a - -0.1).abs() < f32::EPSILON);
        assert!((result.battery_voltage3_v - -0.01).abs() < f32::EPSILON);
        assert!((result.battery_current3_a - -0.1).abs() < f32::EPSILON);
    }

    // Raw: [0x04, 0x00, 0xA0, 0x25, 0x03, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x74, 0xA5, 0x82]
    // As recorded and checked on Blue Smart IP22 Charger
    #[test]
    fn test_ac_charger_state_parse_2() {
        let test_data = [
            0x04, 0x00, 0xA0, 0x25, 0x03, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x74,
            0xA5, 0x82,
        ];

        let result = AcChargerState::parse(&test_data).unwrap();

        assert_eq!(result.mode, Mode::Absorption);
        assert_eq!(result.error_state, ErrorState::NoError);

        assert!((result.battery_voltage1_v - 14.40).abs() < f32::EPSILON);
        assert!((result.battery_current1_a - 2.5).abs() < f32::EPSILON);
        assert!((result.battery_voltage2_v - -0.01).abs() < f32::EPSILON);
        assert!((result.battery_current2_a - -0.1).abs() < f32::EPSILON);
        assert!((result.battery_voltage3_v - -0.01).abs() < f32::EPSILON);
        assert!((result.battery_current3_a - -0.1).abs() < f32::EPSILON);
    }
}

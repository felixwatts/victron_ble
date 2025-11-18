use crate::bit_reader::BitReader;
use crate::err::*;

use super::error_state::ErrorState;
use super::mode::Mode;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AcChargerState {
    pub mode: Mode,
    pub error_state: ErrorState,
    pub battery_voltage1_v: Option<f32>,
    pub battery_current1_a: Option<f32>,
    pub battery_voltage2_v: Option<f32>,
    pub battery_current2_a: Option<f32>,
    pub battery_voltage3_v: Option<f32>,
    pub battery_current3_a: Option<f32>,
    pub temperature_c: Option<f32>,
    pub ac_current_a: Option<f32>,
}

impl AcChargerState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let mode = Mode::try_from(reader.read_unsigned_int(8)?)?;
        let error_state = ErrorState::try_from(reader.read_unsigned_int(8)?)?;
        let battery_voltage1_v = reader.read_unsigned_field(13, 0x1FFF, 0.01, 0.0)?;
        let battery_current1_a = reader.read_unsigned_field(11, 0x7FF, 0.1, 0.0)?;
        let battery_voltage2_v = reader.read_unsigned_field(13, 0x1FFF, 0.01, 0.0)?;
        let battery_current2_a = reader.read_unsigned_field(11, 0x7FF, 0.1, 0.0)?;
        let battery_voltage3_v = reader.read_unsigned_field(13, 0x1FFF, 0.01, 0.0)?;
        let battery_current3_a = reader.read_unsigned_field(11, 0x7FF, 0.1, 0.0)?;
        let temperature_c = reader.read_unsigned_field(7, 0x7F, 1.0, -40.0)?;
        let ac_current_a = reader.read_unsigned_field(9, 0x1FF, 0.1, 0.0)?;

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
        assert!((result.battery_voltage1_v.unwrap() - 14.40).abs() < f32::EPSILON);
        assert!((result.battery_current1_a.unwrap() - 0.0).abs() < f32::EPSILON);
        assert!(result.battery_voltage2_v.is_none());
        assert!(result.battery_current2_a.is_none());
        assert!(result.battery_voltage3_v.is_none());
        assert!(result.battery_current3_a.is_none());
        assert!(result.temperature_c.is_none());
        assert!(result.ac_current_a.is_none());
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
        assert!((result.battery_voltage1_v.unwrap() - 14.40).abs() < f32::EPSILON);
        assert!((result.battery_current1_a.unwrap() - 2.5).abs() < f32::EPSILON);
        assert!(result.battery_voltage2_v.is_none());
        assert!(result.battery_current2_a.is_none());
        assert!(result.battery_voltage3_v.is_none());
        assert!(result.battery_current3_a.is_none());
        assert!(result.temperature_c.is_none());
        assert!(result.ac_current_a.is_none());
    }
}

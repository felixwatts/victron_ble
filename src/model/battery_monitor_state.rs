use super::alarm_reason::AlarmReason;
use crate::bit_reader::BitReader;
use crate::err::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatteryMonitorState {
    pub time_to_go_mins: Option<f32>,
    pub battery_voltage_v: Option<f32>,
    pub alarm_reason: AlarmReason,
    pub aux_input: AuxInput,
    pub battery_current_a: Option<f32>,
    pub consumed_amp_hours_ah: Option<f32>,
    pub state_of_charge_pct: Option<f32>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AuxInput {
    VoltageV(f32),
    MidVoltageV(f32),
    TemperatureK(f32),
    None,
}

impl BatteryMonitorState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let time_to_go_mins = reader.read_unsigned_field(16, 0xFFFF, 1.0, 0.0)?;
        let battery_voltage_v = reader.read_signed_field(16, 0x7FFF, 0.01)?;
        let alarm_reason =
            AlarmReason::from_bits(reader.read_signed_int(16)?).ok_or(Error::InvalidAlarmReason)?;

        // we need to read the next two fields out of order because
        // the format of the former depends on the latter.
        let mut aux_input_reader = reader.clone();
        reader.skip(16)?;
        let aux_input_type = reader.read_unsigned_int(2)?;
        let aux_input = match aux_input_type {
            0 => {
                let aux_voltage = aux_input_reader.read_signed_int(16)? as f32 / 100.0;
                AuxInput::VoltageV(aux_voltage)
            }
            1 => {
                let mid_voltage = aux_input_reader.read_unsigned_int(16)? as f32 / 100.0;
                AuxInput::MidVoltageV(mid_voltage)
            }
            2 => {
                let temperature = aux_input_reader.read_unsigned_int(16)? as f32 / 100.0;
                AuxInput::TemperatureK(temperature)
            }
            3 => AuxInput::None,
            t => return Err(Error::InvalidAuxInputType(t)),
        };

        let battery_current_a = reader.read_signed_field(22, 0x3FFFFF, 0.001)?;
        let consumed_amp_hours_ah = reader.read_unsigned_field(20, 0xFFFFF, -0.1, 0.0)?;
        let state_of_charge_pct = reader.read_unsigned_field(10, 0x3FF, 0.1, 0.0)?;

        Ok(Self {
            time_to_go_mins,
            battery_voltage_v,
            alarm_reason,
            aux_input,
            battery_current_a,
            consumed_amp_hours_ah,
            state_of_charge_pct,
        })
    }
}

use super::alarm_reason::AlarmReason;
use crate::bit_reader::BitReader;
use crate::err::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BatteryMonitorState {
    pub time_to_go_mins: f32,
    pub battery_voltage_v: f32,
    pub alarm_reason: AlarmReason,
    pub aux_input: AuxInput,
    pub battery_current_a: f32,
    pub consumed_amp_hours_ah: f32,
    pub state_of_charge_pct: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum AuxInput {
    VoltageV(f32),
    MidVoltageV(f32),
    TemperatureK(f32),
    None,
}

impl BatteryMonitorState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let time_to_go_mins = reader.read_unsigned_int(16)? as f32;
        let battery_voltage_v = reader.read_signed_int(16)? as f32 / 100.0;
        let alarm_reason = AlarmReason::from_bits(reader.read_signed_int(16)?)
            .ok_or(Error::InvalidData("Unknown alarm reason.".into()))?;

        // we need to read the next two fields out of order because
        // the format of the former depends on the latter.
        let mut aux_value_reader = reader.clone();
        reader.skip(16)?;
        let aux_input = reader.read_unsigned_int(2)?;
        let aux_val = match aux_input {
            0 => {
                let aux_voltage = aux_value_reader.read_signed_int(16)? as f32 / 100.0;
                AuxInput::VoltageV(aux_voltage)
            }
            1 => {
                let mid_voltage = aux_value_reader.read_unsigned_int(16)? as f32 / 100.0;
                AuxInput::MidVoltageV(mid_voltage)
            }
            2 => {
                let temperature = aux_value_reader.read_unsigned_int(16)? as f32 / 100.0;
                AuxInput::TemperatureK(temperature)
            }
            3 => AuxInput::None,
            _ => return Err(Error::InvalidData("Unknown Aux input type.".into())),
        };

        let battery_current_a = reader.read_signed_int(22)? as f32 / 1000.0;
        let consumed_amp_hours_ah = reader.read_signed_int(20)? as f32 / -10.0;
        let state_of_charge_pct = reader.read_unsigned_int(10)? as f32 / 10.0;

        Ok(Self {
            time_to_go_mins,
            battery_voltage_v,
            alarm_reason,
            aux_input: aux_val,
            battery_current_a,
            consumed_amp_hours_ah,
            state_of_charge_pct,
        })
    }
}

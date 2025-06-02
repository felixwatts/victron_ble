use crate::err::*;
use crate::record::*;

use super::battery_monitor_state::BatteryMonitorState;
use super::inverter_state::InverterState;
use super::solar_charger_state::SolarChargerState;
use super::test_record_state::TestRecordState;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeviceState {
    TestRecord(TestRecordState),
    SolarCharger(SolarChargerState),
    BatteryMonitor(BatteryMonitorState),
    InverterState(InverterState),
}

impl DeviceState {
    pub(crate) fn parse(record: &Record) -> Result<Self> {
        match record.record_type() {
            RECORD_TYPE_TEST_RECORD => Ok(Self::TestRecord(TestRecordState::parse(
                &record.decrypt()?,
            )?)),
            RECORD_TYPE_SOLAR_CHARGER => Ok(Self::SolarCharger(SolarChargerState::parse(
                &record.decrypt()?,
            )?)),
            RECORD_TYPE_BATTERY_MONITOR => Ok(Self::BatteryMonitor(BatteryMonitorState::parse(
                &record.decrypt()?,
            )?)),
            RECORD_TYPE_INVERTER => Ok(Self::InverterState(InverterState::parse(
                &record.decrypt()?,
            )?)),
            _ => Err(Error::UnsupportedDeviceType(record.record_type())),
        }
    }
}

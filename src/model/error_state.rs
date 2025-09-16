use num_enum::TryFromPrimitive;
use strum::Display;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Copy, Clone, Ord, PartialOrd, Hash, Display)]
#[repr(u64)]
pub enum ErrorState {
    NotApplicable = 0xFF,
    NoError = 0,
    BatteryVoltsTooHigh = 2,
    ChargerTemperatureTooHigh = 17,
    ChargerOverCurrent = 18,
    ChargerCurrentReversed = 19,
    BulkTimeLimitExceeded = 20,
    CurrentSensorIssue = 21,
    TerminalsOverheated = 26,
    ConverterIssue = 28,
    InputVoltageTooHigh = 33,
    InputCurrentTooHigh = 34,
    InputShutdownExcessBatteryVoltage = 38,
    InputShutdownCurrentFlowWhileOff = 39,
    LostCommunicationWithOneOfDevices = 65,
    SynchronisedChargingDeviceConfigurationIssue = 66,
    BMSConnectionLost = 67,
    NetworkMisconfigured = 68,
    FactoryCalibrationDataLost = 116,
    InvalidFirmware = 117,
    UserSettingsInvalid = 119,
}

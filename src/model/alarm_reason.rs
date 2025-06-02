use bitflags::bitflags;

bitflags! {
    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
    pub struct AlarmReason: i64 {
        const LowVoltage            = 1;
        const HighVoltage           = 2;
        const LowStateOfCharge      = 4;
        const LowStarterVoltage     = 8;
        const HighStarterVoltage    = 16;
        const LowTemperature        = 32;
        const HighTemperature       = 64;
        const MidVoltage            = 128;
        const Overload              = 256;
        const DcRipple              = 512;
        const LowVacOut             = 1024;
        const HighVacOut            = 2048;
        const ShortCircuit          = 4096;
        const BmsLockout            = 8192;
    }
}

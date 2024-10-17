use num_enum::TryFromPrimitive;
use strum::Display;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Copy, Clone, Ord, PartialOrd, Hash, Display)]
#[repr(u64)]
pub enum Mode {
    Off = 0,
    LowPower = 1,
    Fault = 2,
    Bulk = 3,
    Absorption = 4,
    Float = 5,
    Storage = 6,
    Equalize = 7,
    Inverting = 9,
    PowerSupply = 11,
    StartingUp = 245,
    RepeatedAbsorption = 246,
    AutoEqualize = 247,
    BatterySafe = 248,
    ExternalControl = 252,
}
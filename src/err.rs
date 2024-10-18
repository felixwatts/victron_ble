use aes::cipher::StreamCipherError;
use num_enum::TryFromPrimitiveError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error{
    #[error("An error occured in the MacOS bluetooth layer: {0}")]
    Bluest(String),
    #[error("An error occured in the Linux bluetooth layer: {0}")]
    Bluer(String),
    #[error("The specified bluetooth device was not found.")]
    BluetoothDeviceNotFound,
    #[error("There was an error while receiving advertising events from the device.")]
    DeviceEventsChannelError,
    #[error("The data does not represent a Victron Manufacturer Data record. Victron devices emit multiple types of advertisment data so keep listening.")]
    WrongAdvertisement,
    #[error("The data could not be parsed: {0}")]
    InvalidData(String),
    #[error("Incorrect device encryption key. The Device encryption key provided is not correct for this device.")]
    IncorrectDeviceEncryptionKey,
    #[error("Invalid device encryption key. The Device encryption key provided is of the wrong length.")]
    InvalidDeviceEncryptionKey,
    #[error("Unsupported device type. Please raise an issue at https://github.com/felixwatts/victron_ble quoting the device type code: {0}")]
    UnsupportedDeviceType(u8)
}

#[cfg(target_os = "macos")]
impl From<bluest::Error> for Error{
    fn from(e: bluest::Error) -> Self {
        Error::Bluest(e.to_string())
    }
}

#[cfg(target_os = "linux")]
impl From<bluer::Error> for Error{
    fn from(e: bluer::Error) -> Self {
        Error::Bluer(e.to_string())
    }
}

impl From<StreamCipherError> for Error{
    fn from(e: StreamCipherError) -> Self {
        Error::InvalidData(format!("The data could not be decrypted: {e}"))
    }
}

impl<T: num_enum::TryFromPrimitive> From<TryFromPrimitiveError<T>> for Error{
    fn from(e: TryFromPrimitiveError<T>) -> Self {
        Error::InvalidData(format!("Unexpected value: {e}"))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
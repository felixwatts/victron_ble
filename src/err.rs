use aes::cipher::StreamCipherError;
use num_enum::TryFromPrimitiveError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[cfg(feature = "bluetooth")]
    #[cfg(target_os = "macos")]
    #[error("An error occurred in the MacOS bluetooth layer: {0}")]
    Bluetooth(bluest::Error),
    #[cfg(feature = "bluetooth")]
    #[cfg(target_os = "linux")]
    #[error("An error occurred in the Linux bluetooth layer: {0}")]
    Bluetooth(bluer::Error),
    #[error("No Bluetooth adapter found.")]
    BluetoothAdapterNotFound,
    #[error("The specified bluetooth device was not found.")]
    BluetoothDeviceNotFound,
    #[error("The bluetooth device event stream ended.")]
    BluetoothEventStreamClosed,
    #[error("The Manufacturer Data record is too big. It cannot exceed 24 bytes.")]
    RecordTooBig,
    #[error("The Manufacturer Data does not represent a Victron Manufacturer Data record. Victron devices emit multiple types of advertisement data so keep listening.")]
    WrongAdvertisement,
    #[error("The Manufacturer Data could not be decrypted: {0}")]
    DecryptionFailed(StreamCipherError),
    #[error("Incorrect device encryption key. The Device encryption key provided is not correct for this device.")]
    IncorrectDeviceEncryptionKey,
    #[error(
        "Invalid device encryption key. The Device encryption key provided is of the wrong length."
    )]
    InvalidDeviceEncryptionKey,
    #[error("Unsupported device type. Please raise an issue at https://github.com/felixwatts/victron_ble quoting the device type code: {0}")]
    UnsupportedDeviceType(u8),
    #[error("Channel closed by client")]
    ClientClosedChannel,
    #[error("Invalid mode: {0}")]
    InvalidMode(TryFromPrimitiveError<crate::model::Mode>),
    #[error("Invalid error state: {0}")]
    InvalidErrorState(TryFromPrimitiveError<crate::model::ErrorState>),
    #[error("Invalid alarm reason")]
    InvalidAlarmReason,
    #[error("Invalid aux input type: {0}")]
    InvalidAuxInputType(u64),
    #[error("The data was shorter than expected.")]
    DataTooShort,
    #[error("Invalid ac in state")]
    InvalidAcInState,
    #[error("Invalid alarm notification")]
    InvalidAlarmNotification,
}

#[cfg(target_os = "macos")]
#[cfg(feature = "bluetooth")]
impl From<bluest::Error> for Error {
    fn from(e: bluest::Error) -> Self {
        Error::Bluetooth(e)
    }
}

#[cfg(target_os = "linux")]
#[cfg(feature = "bluetooth")]
impl From<bluer::Error> for Error {
    fn from(e: bluer::Error) -> Self {
        Error::Bluetooth(e)
    }
}

impl From<StreamCipherError> for Error {
    fn from(e: StreamCipherError) -> Self {
        Error::DecryptionFailed(e)
    }
}

impl From<TryFromPrimitiveError<crate::model::Mode>> for Error {
    fn from(e: TryFromPrimitiveError<crate::model::Mode>) -> Self {
        Error::InvalidMode(e)
    }
}

impl From<TryFromPrimitiveError<crate::model::ErrorState>> for Error {
    fn from(e: TryFromPrimitiveError<crate::model::ErrorState>) -> Self {
        Error::InvalidErrorState(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

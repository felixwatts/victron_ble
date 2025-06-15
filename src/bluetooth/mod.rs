#![cfg(feature = "bluetooth")]

mod linux;
mod macos;

use crate::{err::*, DeviceState};
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::{wrappers::UnboundedReceiverStream, Stream};

#[cfg(target_os = "linux")]
use linux::open_stream as _open_stream;
#[cfg(target_os = "macos")]
use macos::open_stream as _open_stream;

pub(crate) const VICTRON_MANUFACTURER_ID: u16 = 737;

/// Continuously monitor device state.
///
/// Will attempt to discover the named device, then continuously listen for device state
/// bluetooth broadcasts which will each be decrypted, parsed and sent to the user
/// via a stream.
///
/// # Example
///
///  ```rust
/// # use std::{println, time::Duration};
/// # use tokio_stream::StreamExt;
/// #
/// # #[tokio::main]
/// # async fn main() {
///     let device_name = "Victron Bluetooth device name".into();
///     let device_encryption_key = hex::decode("00").unwrap();
///
///     let mut device_state_stream = victron_ble::open_stream(
///         device_name,
///         device_encryption_key
///     ).unwrap();
///
///     while let Some(result) = device_state_stream.next().await {
///         println!("{result:?}");
///     }
/// # }
/// ```
pub fn open_stream(
    device_name: String,
    device_encryption_key: Vec<u8>,
) -> Result<impl Stream<Item = Result<DeviceState>>> {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        let _ = _open_stream(device_name, device_encryption_key, sender.clone()).await;
    });

    Ok(UnboundedReceiverStream::new(receiver))
}

fn handle_manufacturer_data(
    manufacturer_data: &[u8],
    device_encryption_key: &[u8],
    sender: &mut UnboundedSender<Result<DeviceState>>,
) -> Result<()> {
    let device_state_result =
        super::parse_manufacturer_data(manufacturer_data, device_encryption_key);

    match device_state_result {
        Err(Error::WrongAdvertisement) => Ok(()), // Message irrelevant to user, wait for next advertisement
        Err(e) => {
            // Report error to user
            let _ = sender.send(Err(e.clone()));
            // Fatal error, stop
            Err(e)
        }
        Ok(device_state) => {
            if sender.send(Ok(device_state)).is_err() {
                // If consumer has dropped the channel then stop
                return Err(Error::ClientClosedChannel);
            }
            Ok(())
        }
    }
}

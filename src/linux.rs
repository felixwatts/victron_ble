#![cfg(target_os = "linux")]
#![cfg(feature = "bluetooth")]

//! Linux specific implementations

use tokio::sync::mpsc::UnboundedReceiver;
use crate::{err::*, DeviceState};
use bluer::{DeviceProperty, DeviceEvent};
use tokio_stream::StreamExt;
use crate::parse_manufacturer_data;
use tokio_stream::Stream;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio::sync::mpsc::UnboundedSender;

/// Continuously monitor device state.
///
/// Will attempt to discover the named device, then continuously listen for device state 
/// bluetooth broadcasts which will each be decrypted, parsed and sent to the user
/// via a `tokio::sync::mpsc::UnboundedReceiver`.
/// 
/// # Example
/// 
///  ```rust
/// # use std::{println, time::Duration};
/// # use tokio_stream::StreamExt;
/// #
/// # #[tokio::main]
/// # async fn main() {
///     let device_name = "Victon Bluetooth device name";
///     let device_encryption_key = hex::decode("Victron device encryption key").unwrap();
/// 
///     let mut device_state_stream = victron_ble::open_stream(
///         device_name.into(), 
///         device_encryption_key
///     ).await;
/// 
///     while let Some(result) = device_state_stream.next().await {
///         println!("{result:?}");
///     }
/// # }
/// ```
pub async fn open_stream(
    device_name: String, device_encryption_key: Vec<u8>
) -> Result<impl Stream<Item=Result<DeviceState>>> {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        let result = _open_stream(device_name, device_encryption_key, sender.clone()).await;
        if let Err(e) = result {
            let _ = sender.send(Err(e));
        }
    });

    Ok(UnboundedReceiverStream::new(receiver))
}

async fn _open_stream(
    target_device_name: String, 
    target_device_encryption_key: Vec<u8>, 
    sender: UnboundedSender<Result<DeviceState>>
) -> Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let mut adapter_events = adapter.discover_devices().await?;

    loop {
        if let Some(bluer::AdapterEvent::DeviceAdded(device_addr)) = adapter_events.next().await {
            let device = adapter.device(device_addr)?;
            let device_name = device.name().await?.unwrap_or("(unknown)".to_string());
            if device_name == target_device_name {
                println!("victron_ble: found device: {}", device_name);
                let mut device_events = device.events().await?;

                loop{
                    let device_event = device_events.next().await.ok_or(Error::DeviceEventsChannelError)?;
                    if let DeviceEvent::PropertyChanged(DeviceProperty::ManufacturerData(md)) = device_event {
                        if let Some(md) = &md.get(&crate::record::VICTRON_MANUFACTURER_ID) {
                            let parse_result = parse_manufacturer_data(&md, &target_device_encryption_key);
                            match parse_result{
                                Ok(state) => sender.send(Ok(state))?,
                                Err(Error::WrongAdvertisement) => {}, // Non fatal error, wait for next advertisement
                                Err(e) => {
                                    sender.send(Err(e))?;
                                    return Err(e) // Fatal error, stop
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

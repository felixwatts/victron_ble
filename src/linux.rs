#![cfg(target_os = "linux")]

//! Linux specific helper functions

use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::{err::*, DeviceState};
use bluer::{Adapter, Address, DeviceProperty, DeviceEvent};
use futures::StreamExt;
use crate::parse_manufacturer_data;
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
///     while let Some(result) = device_state_stream.recv().await {
///         println!("{result:?}");
///     }
/// # }
/// ```
pub async fn open_stream(
    device_name: String, device_encryption_key: Vec<u8>
) -> Result<UnboundedReceiver<Result<DeviceState>>> {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        let result = _open_stream(device_name, device_encryption_key, sender.clone()).await;
        if let Err(e) = result {
            let _ = sender.send(Err(e));
        }
    });

    Ok(receiver)
}

async fn _open_stream(
    target_device_name: String, 
    target_device_encryption_key: Vec<u8>, 
    sender: UnboundedSender<Result<DeviceState>>
) -> Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let mut device_events = adapter.discover_devices().await?;

    loop {
        if let Some(bluer::AdapterEvent::DeviceAdded(device_addr)) = device_events.next().await {
            let device = adapter.device(device_addr)?;
            let device_name = device.name().await?.unwrap_or("(unknown)".to_string());
            if device_name == target_device_name {
                let mut change_events = device.events().await?;

                loop{
                    if let DeviceEvent::PropertyChanged(props) = change_events.next().await.ok_or(Error::DeviceEventsChannelError)? {
                        if let DeviceProperty::ManufacturerData(md) = props {  
                            if let Some(md) = &md.get(&737u16) {
                                let parse_result = parse_manufacturer_data(&md, &target_device_encryption_key);
                                match parse_result{
                                    Ok(state) => {
                                        let send_result = sender.send(Ok(state));
                                        if send_result.is_err() {
                                            return Ok(());
                                        }
                                    },
                                    Err(Error::WrongAdvertisement) => {}, // Non fatal error, wait for next advertisement
                                    Err(e) => return Err(e) // Fatal error, stop
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

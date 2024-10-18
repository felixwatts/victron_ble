#![cfg(target_os = "linux")]

//! Linux specific helper functions

use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::{err::*, DeviceState};
use bluer::{Adapter, Address, DeviceProperty, DeviceEvent};
use futures::StreamExt;
use crate::parse_manufacturer_data;

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
///         println/("{result:?}");
///     }
/// # }
/// ```
pub async fn open_stream(
    device_name: String, device_encryption_key: Vec<u8>
) -> Result<UnboundedReceiver<Result<DeviceState>>> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let device_addr = find_device(&adapter, &device_name, Duration::from_secs(30)).await?;

    let device = adapter.device(device_addr)?;

    let mut device_events = device.events().await?;

    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        loop{
            match device_events.next().await {
                None => { 
                    let _ = sender.send(Err(Error::DeviceEventsChannelError));
                    return; 
                },
                Some(DeviceEvent::PropertyChanged(DeviceProperty::ManufacturerData(md))) => {
                    if let Some(md) = md.get(crate::record::VICTRON_MANUFACTURER_ID) {
                        let device_state_result = parse_manufacturer_data(&md, &device_encryption_key);
                        match device_state_result {
                            Err(Error::WrongAdvertisement) => {},
                            Err(_) => {
                                let _ = sender.send(device_state_result);
                                return;
                            },
                            Ok(device_state) => {
                                let send_result = sender.send(Ok(device_state));
                                if send_result.is_err() {
                                    // If consumer has dropped the channel then stop
                                    return;
                                }
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    });

    Ok(receiver)
}

async fn find_device(adapter: &Adapter, device_name: &str, timeout: Duration) -> Result<Address> {
    tokio::time::timeout(timeout, _find_device(adapter, device_name))
        .await
        .map_err(|_| Error::BluetoothDeviceNotFound)
        .flatten()
}

async fn _find_device(adapter: &Adapter, device_name: &str) -> Result<Address> {
    let mut adapter_events = adapter.discover_devices().await?;
    loop {
        match adapter_events.next().await {
            Some(bluer::AdapterEvent::DeviceAdded(device_addr)) => {
                let device = adapter.device(device_addr)?;
                let found_device_name = device.name().await?.unwrap_or("(unknown)".to_string());
                if device_name == found_device_name {
                    return Ok(device_addr);
                }
            },
            None => return Err(Error::BluetoothDeviceNotFound),
            _ => {}
        }
    }
}

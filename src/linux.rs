#![cfg(target_os = "linux")]

//! Linux specific helper functions

use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::{err::*, DeviceState};

const VICTRON_MANUFACTURER_ID: u16 = 737;

/// Bluer gives the manufacturer data in the form of a HashMap. This
/// function can consume that HashMap
pub (crate) fn parse_manufacturer_data(manufacturer_data: &HashMap<u16, Vec<u8>>, device_encryption_key: &[u8]) -> Result<DeviceState> {
    if let Some(md) = md.get(VICTRON_MANUFACTURER_ID) {
        crate::parse_manufacturer_data(&md, &device_encryption_key);
    } else {
        return Err(Error::NotVictron)
    }
}

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
///         device_name, 
///         &device_encryption_key, 
///         Duration::from_secs(30)
///     ).await;
/// 
///     while let Some(result) = device_state_stream.recv().await {
///         println/("{result:?}");
///     }
/// # }
/// ```
pub async fn open_stream(
    device_name: &str, device_encryption_key: &[u8], 
    discovery_timeout: Duration
) -> UnboundedReceiver<DeviceState> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let device_addr = find_device(&adapter, device_name, discovery_timeout).await?;

    let device = adapter.device(device_addr)?;

    let mut device_events = device.events().await?;

    let (sender, receiver) = tokio::sync::mpsc::channel();

    tokio::spawn!(async move {
        loop{
            match device_events.next().await {
                Err(e) => { 
                    sender.send(Err(Error::DeviceEventsChannelError)).await;
                    return; 
                },
                Ok(DeviceEvent::PropertyChanged(DeviceProperty::ManufacturerData(md))) => {
                    let device_state_result = parse_manufacturer_data(&md, &device_encryption_key);
                    match device_state_result {
                        Err(Error::WrongAdvertisement) => {},
                        Err(_) => {
                            sender.send(device_state_result).await;
                            return;
                        },
                        Ok(device_state) => {
                            sender.send(Ok(device_state)).await;
                        }
                    }
                },
                _ => {}
            }
        }
    });

    receiver
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
                    return device_addr;
                }
            },
            None => return Err(Error::BluetoothDeviceNotFound),
            _ => {}
        }
    }
}

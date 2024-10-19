#![cfg(target_os = "linux")]

//! Linux specific helper functions

use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::{err::*, DeviceState};
use bluer::{Adapter, Address, DeviceProperty, DeviceEvent};
use futures::StreamExt;
use crate::parse_manufacturer_data;
use futures_util::pin_mut;

pub async fn fetch(target_device_name: String, target_device_encryption_key: Vec<u8>) -> Result<DeviceState> {
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
                                    Ok(state) => return Ok(state),
                                    Err(Error::WrongAdvertisement) => {},
                                    Err(e) => return Err(e.into())
                                }
                            }
                        }
                    }
                }
            }
        }
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
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        let session = bluer::Session::new().await.unwrap();
        let adapter = session.default_adapter().await.unwrap();
        adapter.set_powered(true).await.unwrap();
    
        let mut device_events = adapter.discover_devices().await.unwrap();
    
        loop {
            if let Some(bluer::AdapterEvent::DeviceAdded(device_addr)) = device_events.next().await {
                let device = adapter.device(device_addr).unwrap();
                let device_name = device.name().await.unwrap().unwrap_or("(unknown)".to_string());
                if device_name == target_device_name {
                    let mut change_events = device.events().await.unwrap();
    
                    loop{
                        if let DeviceEvent::PropertyChanged(props) = change_events.next().await.ok_or(Error::DeviceEventsChannelError)? {
                            if let DeviceProperty::ManufacturerData(md) = props {  
                                if let Some(md) = &md.get(&737u16) {
                                    let parse_result = parse_manufacturer_data(&md, &target_device_encryption_key);
                                    match parse_result{
                                        Ok(state) => sender.send(Ok(state)).await,
                                        Err(Error::WrongAdvertisement) => {},
                                        Err(e) => sender.send(Err(e)).await
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    Ok(receiver)
}

async fn find_device(adapter: &Adapter, device_name: &str, timeout: Duration) -> Result<Address> {
    Ok(
        tokio::time::timeout(timeout, _find_device(adapter, device_name))
            .await
            .map_err(|_| Error::BluetoothDeviceNotFound)??
    )
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

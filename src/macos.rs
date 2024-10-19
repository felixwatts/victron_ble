#![cfg(target_os = "macos")]

//! MacOS specific implementations

use futures::StreamExt;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::{err::*, DeviceState};

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
///     ).await.unwrap();
/// 
///     while let Some(result) = device_state_stream.recv().await {
///         println/("{result:?}");
///     }
/// # }
/// ```
pub async fn open_stream(device_name: String, device_encryption_key: Vec<u8>) -> Result<UnboundedReceiver<Result<DeviceState>>> {
    let adapter = bluest::Adapter::default().await.ok_or(Error::Bluest("Default adapter not found".into()))?;
    adapter.wait_available().await?;

    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    
    tokio::spawn(async move{
        let adapter_events_result = adapter.scan(&[]).await;
        let mut adapter_events = match adapter_events_result {
            Ok(adapter_events) => adapter_events,
            Err(e) => {
                let _ = sender.send(Err(e.into()));
                return;
            }
        };
        loop {
            match adapter_events.next().await {
                Some(device) => {
                    let found_device_name = device.device.name_async().await.unwrap_or("(unknown)".into());
                    if device_name == found_device_name {
                        if let Some(md) = device.adv_data.manufacturer_data {
                            if md.company_id == crate::record::VICTRON_MANUFACTURER_ID {
                                let device_state_result = crate::parse_manufacturer_data(&md.data, &device_encryption_key);

                                match device_state_result {
                                    Err(Error::WrongAdvertisement) => {}, // Non fatal error, wait for next advertisement
                                    Err(_) => {
                                        // Fatal error, stop
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
                        }
                    }
                },
                None => {
                    // Adapter events stream has ended, stop
                    let _ = sender.send(Err(Error::BluetoothDeviceNotFound));
                    return
                }
            }
        }
    });

    Ok(receiver)
}
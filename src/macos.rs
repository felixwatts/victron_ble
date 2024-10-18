#![cfg(target_os = "macos")]

//! MacOS specific helper functions

use std::time::Duration;
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
///     ).await;
/// 
///     while let Some(result) = device_state_stream.recv().await {
///         println/("{result:?}");
///     }
/// # }
/// ```
pub async fn open_stream(device_name: &str, device_encryption_key: &[u8], discovery_timeout: Duration) -> UnboundedReceiver<Result<DeviceState>> {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    receiver
}
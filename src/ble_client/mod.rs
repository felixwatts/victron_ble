#[cfg(all(target_os = "linux", feature = "ble_client"))]
#[path = "linux.rs"]
mod ble_client;
#[cfg(all(target_os = "macos", feature = "ble_client"))]
#[path = "macos.rs"]
mod ble_client;

#[cfg(feature = "ble_client")]
pub(crate) use ble_client::*;

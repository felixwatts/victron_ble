#![cfg(target_os = "macos")]
#![cfg(feature = "bluetooth")]

//! MacOS specific implementation

use crate::{err::*, DeviceState};
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::StreamExt;

pub(crate) async fn open_stream(
    target_device_name: String,
    target_device_encryption_key: Vec<u8>,
    mut sender: UnboundedSender<Result<DeviceState>>,
) -> Result<()> {
    let adapter = bluest::Adapter::default()
        .await
        .ok_or(Error::Bluest("Default adapter not found".into()))?;
    adapter.wait_available().await?;
    let adapter_events_result = adapter.scan(&[]).await;
    let mut adapter_events = match adapter_events_result {
        Ok(adapter_events) => adapter_events,
        Err(e) => {
            let e: crate::Error = e.into();
            let _ = sender.send(Err(e.clone()));
            return Err(e);
        }
    };
    loop {
        match adapter_events.next().await {
            Some(device) => {
                let found_device_name = device
                    .device
                    .name_async()
                    .await
                    .unwrap_or("(unknown)".into());
                if target_device_name == found_device_name {
                    if let Some(md) = device.adv_data.manufacturer_data {
                        if md.company_id == super::VICTRON_MANUFACTURER_ID {
                            super::handle_manufacturer_data(
                                &md.data,
                                &target_device_encryption_key,
                                &mut sender,
                            )?;
                        }
                    }
                }
            }
            None => {
                // Adapter events stream has ended, stop
                let _ = sender.send(Err(Error::BluetoothDeviceNotFound));
                return Err(Error::BluetoothDeviceNotFound);
            }
        }
    }
}

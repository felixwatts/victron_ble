#![cfg(target_os = "linux")]

//! Linux specific implementation

use tokio::sync::mpsc::UnboundedReceiver;
use crate::{err::*, DeviceState};
use bluer::{DeviceProperty, DeviceEvent};
use tokio_stream::StreamExt;
use crate::parse_manufacturer_data;
use tokio_stream::Stream;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) async fn open_stream(
    target_device_name: String, 
    target_device_encryption_key: Vec<u8>, 
    mut sender: UnboundedSender<Result<DeviceState>>
) -> Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let mut adapter_events = adapter.discover_devices().await?;

    loop {
        let ev = adapter_events.next().await;
        if let Some(bluer::AdapterEvent::DeviceAdded(device_addr)) = ev {
            let device = adapter.device(device_addr)?;
            let device_name = device.name().await?.unwrap_or("(unknown)".to_string());
            if device_name == target_device_name {
                let mut device_events = device.events().await?;

                loop{
                    let device_event = device_events.next().await.ok_or(Error::DeviceEventsChannelError)?;
                    if let DeviceEvent::PropertyChanged(DeviceProperty::ManufacturerData(md)) = device_event {
                        if let Some(md) = &md.get(&crate::record::VICTRON_MANUFACTURER_ID) {
                            crate::handle_manufacturer_data(md, &target_device_encryption_key, &mut sender)?;
                        }
                    }
                }
            }
        }
    }
}

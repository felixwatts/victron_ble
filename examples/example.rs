#![cfg(target_os = "linux")]

use std::println;

use bluer::{DeviceEvent, DeviceProperty};
use futures::StreamExt;
use bluer::Adapter;

#[tokio::main]
async fn main() {
    // You can get both of these from the Victron Connect app, connect to the device and look in "Device Info"
    let target_device_name = "Victon Bluetooth device name";
    let target_device_encryption_key = hex::decode("Victron device encryption key").unwrap();

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
                    if let DeviceEvent::PropertyChanged(props) = change_events.next().await.unwrap() {
                        if let DeviceProperty::ManufacturerData(md) = props {
                            if let Some(md) = md.get(&737u16) {
                                let parse_result = victron_ble::parse_manufacturer_data(&md, &target_device_encryption_key);
                                println!("{parse_result:?}");
                            }
                        }
                    }
                }
            }
        }
    }
}
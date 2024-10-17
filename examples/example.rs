#![cfg(target_os = "linux")]

use std::println;

use bluer::DeviceEvent;
use bluest::Adapter;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    // You can get both of these from the Victron Connect app, connect to the device and look in "Device Info"
    let target_device_name = "mppt_cabin";
    let target_device_encryption_key = hex::decode("Victron device encryption key").unwrap();

    let session = bluer::Session::new().await.unwrap();
    let adapter = session.default_adapter().await.unwrap();
    adapter.set_powered(true).await.unwrap();

    let device_events = adapter.discover_devices().await.unwrap();
    pin_mut!(device_events);

    loop {
        if let Some(bluer::AdapterEvent::DeviceAdded(device_addr)) = device_events.next().await {
            let device = adapter.device(addr).unwrap();
            let device_name = device.name().await.unwrap().unwrap_or("(unknown)");
            if device_name == target_device_name {
                let change_events = device.events().await.unwrap();

                loop{
                    if let DeviceEvent::PropertyChanged(props) = change_events.next().await.unwrap() {
                        // let md = props.manufacturer_data();

                        println!("{props:?}");
                    }
                }
            }
        }
    }

    // let mut scan = adapter.scan(&[]).await.unwrap();

    // while let Some(discovered_device) = scan.next().await {

    //     if discovered_device.device.name_async().await.as_deref().unwrap_or("(unknown)") == device_name {
    //         let md = discovered_device.adv_data.manufacturer_data.unwrap().data.clone();
    //         let victron_device_state = victron_ble::parse_manufacturer_data(&md, &device_encryption_key);
    //         println!("{victron_device_state:?}")
    //     }
    // }
}
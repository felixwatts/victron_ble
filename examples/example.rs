use std::{println, time::Duration};

#[tokio::main]
async fn main() {
    // You can get both of these from the Victron Connect app, connect to the device and look in "Device Info"
    let device_name = "Victon Bluetooth device name";
    let device_encryption_key = hex::decode("Victron device encryption key").unwrap();

    let mut device_state_stream = victron_ble::open_stream(
        device_name, 
        &device_encryption_key, 
        Duration::from_secs(30)
    ).await;

    while let Some(result) = device_state_stream.recv().await {
        println!("{result:?}");
    }
}
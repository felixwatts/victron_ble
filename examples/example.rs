use std::println;

#[tokio::main]
async fn main() {
    // You can get both of these from the Victron Connect app, connect to the device and look in "Device Info"
    let device_name = "Victon Bluetooth device name";
    let device_encryption_key = hex::decode("Victron device encryption key").unwrap();

    let mut device_state_stream = victron_ble::open_stream(
        device_name.into(), 
        device_encryption_key
    ).await.unwrap();

    while let Some(result) = device_state_stream.recv().await {
        println!("{result:?}");
    }
}
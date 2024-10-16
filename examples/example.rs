use std::time::Duration;
use bluez_async::BluetoothSession;
use tokio::{self, time};

#[tokio::main]
async fn main() {
    // You can get both of these from the Victron Connect app.
    let device_name = "Victron Device Bluetooth name";
    let device_encryption_key = hex::decode("Victron device encryption key").unwrap();

    let (_, session) = BluetoothSession::new().await.unwrap();
    session.start_discovery().await.unwrap();
    time::sleep(Duration::from_secs(5)).await;
    session.stop_discovery().await.unwrap();

    let device = session
        .get_devices()
        .await
        .unwrap()
        .into_iter()
        .find(|device| device.name.as_deref() == Some(device_name))
        .expect("The specified Bluetooth device was not found.");

    let victron_device_state = victron_ble::parse_manufacturer_data(&device.manufacturer_data, &device_encryption_key).unwrap();

    println!("{victron_device_state:?}");
}
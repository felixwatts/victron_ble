use bluest::Adapter;
use futures::StreamExt;
use tokio;

#[tokio::main]
async fn main() {
    // You can get both of these from the Victron Connect app, connect to the device and look in "Device Info"
    let device_name = "Victron device name";
    let device_encryption_key = hex::decode("Victron device encryption key").unwrap();

    let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found").unwrap();
    adapter.wait_available().await.unwrap();

    let mut scan = adapter.scan(&[]).await.unwrap();

    while let Some(discovered_device) = scan.next().await {
        if discovered_device.device.name().as_deref().unwrap_or("(unknown)") == device_name {
            let md = discovered_device.adv_data.manufacturer_data.unwrap().data.clone();
            let victron_device_state = victron_ble::parse_manufacturer_data(&md, &device_encryption_key);
            println!("{victron_device_state:?}")
        }
    }
}
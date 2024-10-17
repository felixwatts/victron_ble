use bluest::Adapter;
use futures::StreamExt;
use tokio;

#[tokio::main]
async fn main() {
    // You can get both of these from the Victron Connect app.
    // let device_mac_address = MacAddress::from_str("Victron Device MAC address").unwrap();
    let device_encryption_key = hex::decode("49b7d10803c5efc4164ca9757cc64214").unwrap();

    let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found").unwrap();
    adapter.wait_available().await.unwrap();

    let mut scan = adapter.scan(&[]).await.unwrap();

    while let Some(discovered_device) = scan.next().await {
        // println!(
        //     "{}{}: {:?}",
        //     discovered_device.device.name().as_deref().unwrap_or("(unknown)"),
        //     discovered_device
        //         .rssi
        //         .map(|x| format!(" ({}dBm)", x))
        //         .unwrap_or_default(),
        //     discovered_device.adv_data
        // );

        if discovered_device.device.name().as_deref().unwrap_or("(unknown)") == "MPPT Cabin" {
            let md = discovered_device.adv_data.manufacturer_data.unwrap().data.clone();
            let victron_device_state = victron_ble::parse_manufacturer_data(&md, &device_encryption_key);
            println!("{victron_device_state:?}")
        }
    }

    // let (_, session) = BluetoothSession::new().await.unwrap();
    // session.start_discovery().await.unwrap();
    // time::sleep(Duration::from_secs(5)).await;
    // session.stop_discovery().await.unwrap();

    // let device = session
    //     .get_devices()
    //     .await
    //     .unwrap()
    //     .into_iter()
    //     .find(|device| device.mac_address == device_mac_address)
    //     .expect("The specified Bluetooth device was not found.");

    // let victron_device_state = victron_ble::parse_manufacturer_data(&device.manufacturer_data, &device_encryption_key).unwrap();

    // println!("{victron_device_state:?}");
}
use std::{env, println};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run --example example <victron device name> <victron device encryption key>");
        return;
    }

    let device_name = args.get(1).unwrap();
    let device_encryption_key = hex::decode(args.get(2).unwrap()).unwrap();

    let result = victron_ble::fetch(device_name.clone(), device_encryption_key).await;
    println!("{result:?}");

    // let mut device_state_stream = victron_ble::open_stream(
    //     device_name.into(), 
    //     device_encryption_key
    // ).await.unwrap();

    // while let Some(result) = device_state_stream.recv().await {
    //     println!("{result:?}");
    // }
}
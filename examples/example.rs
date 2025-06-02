use std::{env, println};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run --example example <victron device name> <victron device encryption key>");
        return;
    }

    let device_name = args.get(1).unwrap();
    let device_encryption_key = hex::decode(args.get(2).unwrap())
        .expect("Invalid device encryption key, it should be hex encoded.");

    let mut device_state_stream =
        victron_ble::open_stream(device_name.into(), device_encryption_key).unwrap();

    while let Some(result) = device_state_stream.next().await {
        match result {
            Ok(_device_state) => println!("{result:?}"),
            Err(e) => println!("{e}"),
        }
    }
}

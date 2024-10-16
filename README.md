# Victron BLE

Read data from Victron devices over Bluetooth Low Energy.

Some Victron devices use the _BLE advertising data_ protocol to broadcast some aspects of their state on a regular basis. This crate enables you to easily decrypt and parse the broadcast data into usable form.

See the provided example for how to use.

Currently only Solar Charger type devices are supported but support can be added for other device types if requested.

## Encryption Key

The data broadcast by Victron devices is encrypted. In order to decrypt it you will need the device's encryption key. This can be found using the Victron Connect app on Android or iOS. Connect to the device using the app, then look through the device settings to find the encryption key.

## Sources

https://communityarchive.victronenergy.com/questions/187303/victron-bluetooth-advertising-protocol.html

And see the file is this repo: `extra-manufacturer-data-2022-12-14.pdf`

## Thanks

The `BitReader` is copied from https://github.com/keshavdv/victron-ble

Some structs are copied from https://github.com/PeterGrace/vedirect_rs
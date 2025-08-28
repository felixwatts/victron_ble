# 0.5.1

- Chg: make `AcInState` and `AlarmNotification` public.

# 0.5.0

- Add: support for VE.Bus device type.
- Add: `AcInState` and `AlarmNotification` enums for VE.Bus specific states.

# 0.4.1

- Fix: `open_stream` hangs at 100% CPU if the bluetooth adapter is disconnected.

# 0.4.0

- Chg: crate is no_std when bluetooth feature is off.
- Chg: put all bluetooth functionality behind a new 'bluetooth' feature.
- Fix: make `TestRecordState` public.
- Fix: rename `DeviceState::InverterState` to `DeviceState::Inverter`

# 0.3.1

- Fix: make more model structs public.
- Fix: doc tests

# 0.3.0

- Fix: `open_stream` stream ends after one item.
- Add: support for Inverter device type.
- Add: support for Battery Monitor device type.

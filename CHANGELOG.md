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
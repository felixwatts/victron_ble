use crate::err::*;
use aes::cipher::StreamCipher;
use ctr::cipher::KeyIvInit;

pub(crate) const RECORD_TYPE_TEST_RECORD: u8 = 0x00;
pub(crate) const RECORD_TYPE_SOLAR_CHARGER: u8 = 0x01;
pub(crate) const RECORD_TYPE_BATTERY_MONITOR: u8 = 0x02;
pub(crate) const VICTRON_MANUFACTURER_ID: u16 = 737;

const MANUFACTURER_DATA_RECORD_TYPE: u8 = 0x10;

type EncryptionAlgorithm = ctr::Ctr128LE<aes::Aes128>;

pub(crate) struct Record<'d, 'k> {
    data: &'d [u8],
    encryption_key: &'k [u8],
}

/// The content of a Victron extra manufacturer data record. Provides
/// methods to validate the record and decrypt the payload.
///
/// Some Victron devices use the BLE advertising protocol to send a
/// manufacturer data record that represents the current device state.
/// The record has this form:
///
/// Bytes | Value | Meaning
/// 0     | 0x10  | This is a Victron device status message
/// 1     | ?     | ?
/// 2-3   | ?     | Device model ID. (Not used in this crate.)
/// 4     | ?     | Record type, such as SolarCharger or Inverter.
/// 5-6   | ?     | The IV used in decryption in little endian form.
/// 7     | ?     | The first byte of the decryption key. Used to validate the given decyption key.
/// 8..   | ?     | Payload encrypted using AES128 in CTR mode with the given IV.
impl<'d, 'k> Record<'d, 'k> {
    pub(crate) fn new(data: &'d [u8], encryption_key: &'k [u8]) -> Result<Self> {
        let record = Self {
            data,
            encryption_key,
        };

        if !record.is_victron_extra_manufacturer_data() {
            return Err(Error::WrongAdvertisement);
        }

        if !record.is_correct_encryption_key() {
            return Err(Error::IncorrectDeviceEncryptionKey);
        }

        Ok(record)
    }

    pub(crate) fn decrypt(&self) -> Result<Vec<u8>> {
        let mut algo = EncryptionAlgorithm::new(self.encryption_key.into(), &self.iv().into());

        let ciper = self.cipher();
        let mut data = vec![0; ciper.len()];
        algo.apply_keystream_b2b(&ciper, &mut data)?;

        Ok(data)
    }

    pub(crate) fn record_type(&self) -> u8 {
        self.data[4]
    }

    fn is_victron_extra_manufacturer_data(&self) -> bool {
        self.data[0] == MANUFACTURER_DATA_RECORD_TYPE
    }

    fn iv(&self) -> [u8; 16] {
        [
            self.data[5],
            self.data[6],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    }

    fn is_correct_encryption_key(&self) -> bool {
        self.data[7] == self.encryption_key[0]
    }

    fn cipher(&self) -> Vec<u8> {
        assert!(self.data[8..].len() <= 16);

        let mut padded = Vec::with_capacity(16);
        padded.extend_from_slice(&self.data[8..]);
        let pad_value = 16 - padded.len() as u8;
        padded.resize(16, pad_value);

        padded
    }
}

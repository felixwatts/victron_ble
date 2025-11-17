use crate::err::*;
use aes::cipher::StreamCipher;
use ctr::cipher::KeyIvInit;

pub(crate) const RECORD_TYPE_TEST_RECORD: u8 = 0x00;
pub(crate) const RECORD_TYPE_SOLAR_CHARGER: u8 = 0x01;
pub(crate) const RECORD_TYPE_BATTERY_MONITOR: u8 = 0x02;
pub(crate) const RECORD_TYPE_INVERTER: u8 = 0x03;
pub(crate) const RECORD_TYPE_AC_CHARGER: u8 = 0x08;
pub(crate) const RECORD_TYPE_VE_BUS: u8 = 0x0C;

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
/// 7     | ?     | The first byte of the decryption key. Used to validate the given decryption key.
/// 8..   | ?     | Payload encrypted using AES128 in CTR mode with the given IV.
impl<'d, 'k> Record<'d, 'k> {
    pub(crate) fn new(data: &'d [u8], encryption_key: &'k [u8]) -> Result<Self> {
        let record = Self {
            data,
            encryption_key,
        };

        if record.is_data_too_big() {
            return Err(Error::RecordTooBig)
        }

        if !record.is_victron_extra_manufacturer_data() {
            return Err(Error::WrongAdvertisement);
        }

        if !record.is_correct_encryption_key() {
            return Err(Error::IncorrectDeviceEncryptionKey);
        }

        Ok(record)
    }

    pub(crate) fn decrypt(&self) -> Result<[u8; 16]> {
        let mut algo = EncryptionAlgorithm::new(self.encryption_key.into(), &self.iv().into());

        let cipher = self.cipher();
        let mut data = [0; 16];
        algo.apply_keystream_b2b(&cipher, &mut data)?;

        Ok(data)
    }

    pub(crate) fn record_type(&self) -> u8 {
        self.data[4]
    }

    fn is_data_too_big(&self) -> bool {
        self.data.len() > 24
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

    fn cipher(&self) -> [u8; 16] {
        let data = &self.data[8..];
        let data_len = data.len();
        assert!(data_len <= 16);

        let mut padded = [0u8; 16];
        padded[..data_len].copy_from_slice(data);
        let pad_value = 16 - data_len as u8;
        padded[data_len..].fill(pad_value);

        padded
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decrypt() {
        let plaintext = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let key = [
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let iv = [33, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut algo = EncryptionAlgorithm::new(key.as_slice().into(), &iv.into());
        let mut cipher = [0; 16];
        algo.apply_keystream_b2b(&plaintext, &mut cipher).unwrap();

        let manufacturer_data = [
            MANUFACTURER_DATA_RECORD_TYPE,
            0x00,
            0x00,
            0x00,
            RECORD_TYPE_TEST_RECORD,
            iv[0],
            iv[1],
            key[0],
            cipher[0],
            cipher[1],
            cipher[2],
            cipher[3],
            cipher[4],
            cipher[5],
            cipher[6],
            cipher[7],
            cipher[8],
            cipher[9],
            cipher[10],
            cipher[11],
            cipher[12],
            cipher[13],
            cipher[14],
            cipher[15],
        ];
        let record = Record::new(&manufacturer_data, &key).unwrap();
        let decrypted = record.decrypt().unwrap();
        assert_eq!(decrypted, plaintext);
    }
}

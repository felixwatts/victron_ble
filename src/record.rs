use crate::err::*;
use aes::{cipher::StreamCipher};
use ctr::cipher::KeyIvInit;

pub (crate) const RECORD_TYPE_TEST_RECORD: u8 = 0x00;
pub (crate) const RECORD_TYPE_SOLAR_CHARGER: u8 = 0x01;

const MANUFACTURER_DATA_RECORD_TYPE: u8 = 0x10;

type EncryptionAlgorithm = ctr::Ctr128LE<aes::Aes128>;

pub (crate) struct Record<'d, 'k>{
    data: &'d [u8],
    encryption_key: &'k[u8]
}

impl<'d, 'k> Record<'d, 'k>{

    // prefix=struct.unpack("<H", data[:2])[0],
    // model_id=struct.unpack("<H", data[2:4])[0],
    // readout_type=struct.unpack("<B", data[4:5])[0],
    // iv=struct.unpack("<H", data[5:7])[0],
    // encrypted_data=data[7:],

    pub (crate) fn new(data: &'d[u8], encryption_key: &'k[u8]) -> Result<Self> {
        let record = Self{ data, encryption_key };

        if !record.is_victron_extra_manufacturer_data() {
            return Err(Error::WrongAdvertisement);
        }

        if !record.is_correct_encryption_key() {
            return Err(Error::IncorrectDeviceEncryptionKey);
        }

        Ok(record)
    }

    pub (crate) fn decrypt(&self) -> Result<Vec<u8>> {
        let mut algo = EncryptionAlgorithm::new(
            self.encryption_key.into(), 
            &self.iv().into()
        );

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
        [ self.data[5], self.data[6], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
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
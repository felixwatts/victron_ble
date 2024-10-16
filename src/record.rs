use std::collections::HashMap;
use crate::err::*;
use aes::cipher::StreamCipher;
use ctr::cipher::KeyIvInit;

pub (crate) const RECORD_TYPE_TEST_RECORD: u8 = 0x00;
pub (crate) const RECORD_TYPE_SOLAR_CHARGER: u8 = 0x01;

const MANUFACTURER_DATA_RECORD_TYPE: u16 = 0x10;

type EncryptionAlgorithm = ctr::Ctr64LE<aes::Aes128>;

pub (crate) struct Record<'d, 'k>{
    data: &'d Vec<u8>,
    encryption_key: &'k[u8]
}

impl<'d, 'k> Record<'d, 'k>{
    pub (crate) fn new(manufacturer_data: &'d HashMap<u16, Vec<u8>>, encryption_key: &'k[u8]) -> Result<Self> {
        let data = manufacturer_data
            .get(&MANUFACTURER_DATA_RECORD_TYPE)
            .ok_or(Error::InvalidData("The Victron record was not found in the manufacturer data.".into()))?;

        let record = Self{ data, encryption_key };

        if record.encryption_key_byte_0() != record.encryption_key[0] {
            return Err(Error::IncorrectDeviceEncryptionKey);
        }

        Ok(record)
    }

    pub (crate) fn decrypt(&self) -> Result<Vec<u8>> {
        let mut cipher = EncryptionAlgorithm::new(
            self.encryption_key.into(), 
            self.nonce().into()
        );

        let mut payload = Vec::with_capacity(self.encrypted_payload().len());
        cipher.apply_keystream_b2b(self.encrypted_payload(), &mut payload)?;
        Ok(payload)
    }

    pub(crate) fn record_type(&self) -> u8 { 
        self.data[0] 
    }

    fn nonce(&self) -> &[u8] { 
        // TODO might need to swap bytes here
        &self.data[1..3] 
    }

    fn encryption_key_byte_0(&self) -> u8 { 
        self.data[3] 
    }

    fn encrypted_payload(&self) -> &[u8] {
        &self.data[4..]
    }
}
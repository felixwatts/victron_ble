use crate::err::*;

#[derive(Clone)]
pub(crate) struct BitReader<'a>{cursor: usize, data: &'a[u8]}

/// The Victron BLE data uses a packed binary format in which numbers can be
/// represented by artitrary numbers of bits.
/// 
/// This struct provides methods for reading numbers from data in the packed
/// format.
/// 
/// Copied from <https://github.com/keshavdv/victron-ble>
impl<'a> BitReader<'a>{
    pub fn new(data: &'a[u8]) -> Self{
        Self{cursor: 0, data}
    }

    pub fn read_unsigned_int(&mut self, num_bits: usize) -> Result<u64> {
        let mut value = 0u64;
        for position in 0..num_bits {
            value |= if self.read_bit()? { 1u64 } else { 0u64 } << position
        }
        Ok(value)
    }

    pub fn read_signed_int(&mut self, num_bits: usize) -> Result<i64> {
        let mut value = 0i64;
        for position in 0..num_bits-1 {
            value |= if self.read_bit()? { 1i64 } else { 0i64 } << position
        }
        if self.read_bit()? {
            value -= 1i64 << (num_bits - 1)
        }
        Ok(value)
    }

    pub fn skip(&mut self, num_bits: usize) -> Result<()> {
        self.read_unsigned_int(num_bits)?;
        Ok(())
    }

    fn read_bit(&mut self) -> Result<bool> {
        if self.cursor == self.data.len() * 8 {
            return Err(Error::InvalidData("The data was shorter than expected.".into()));
        }

        let byte = self.cursor / 8;
        let bit = self.cursor % 8;
        let byte_val = self.data[byte];
        let bit_val = (byte_val >> bit) & 1;
        let is_bit_set = bit_val == 1;

        self.cursor += 1;

        Ok(is_bit_set)
    }
}

mod test {
    #[test]
    fn test_read(){
        use crate::bit_reader::BitReader;
        
        let data = hex::decode("1a2b3c4d5e6f7890").unwrap();
        let mut reader = BitReader::new(&data[..]);

        assert!(!reader.read_bit().unwrap());
        assert!(reader.read_bit().unwrap());
        assert!(!reader.read_bit().unwrap());
        assert!(reader.read_bit().unwrap());
        assert!(reader.read_unsigned_int(6).unwrap() == 0x31);
        assert!(reader.read_signed_int(6).unwrap() == 0x0A);
        assert_eq!(reader.read_signed_int(4).unwrap(), -0x04);
        assert!(reader.read_unsigned_int(11).unwrap() == 0x4D3);
        assert!(!reader.read_bit().unwrap());
        assert!(reader.read_unsigned_int(32).unwrap() == 0x90786F5E);
    }
}
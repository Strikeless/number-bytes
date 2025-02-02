use crate::{FromBytesResult, NumberBytes};

pub enum NumberBytesEndianness {
    NativeEndian,
    BigEndian,
    LittleEndian,
}

trait NumberBytesEndiannessExt: Sized {
    fn from_bytes(bytes: &[u8], endianness: NumberBytesEndianness) -> FromBytesResult<Self>;
    fn to_bytes(self, endianness: NumberBytesEndianness) -> Vec<u8>;
}

impl<T> NumberBytesEndiannessExt for T where T: NumberBytes {
    fn from_bytes(bytes: &[u8], endianness: NumberBytesEndianness) -> FromBytesResult<Self> {
        match endianness {
            NumberBytesEndianness::NativeEndian => Self::from_ne_bytes(bytes),
            NumberBytesEndianness::BigEndian => Self::from_be_bytes(bytes),
            NumberBytesEndianness::LittleEndian => Self::from_le_bytes(bytes),
        }
    }

    fn to_bytes(self, endianness: NumberBytesEndianness) -> Vec<u8> {
        match endianness {
            NumberBytesEndianness::NativeEndian => self.to_ne_bytes(),
            NumberBytesEndianness::BigEndian => self.to_be_bytes(),
            NumberBytesEndianness::LittleEndian => self.to_le_bytes(),
        }
    }
}

use core::array::TryFromSliceError;

pub mod endianness;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "endianness")]
pub mod endianness;

pub type FromBytesError = TryFromSliceError;
pub type FromBytesResult<T> = Result<T, FromBytesError>;

pub trait NumberBytes: Sized {
    /// How many bytes a value of this type consists of.
    /// 
    /// A single byte explicitly refers to 8 bits and the builtin [`u8`] type.
    const BYTES: usize;

    /// Returns the memory representation of this integer as a byte array in native byte order.
    /// 
    /// Type-generic wrapper around [`to_ne_bytes`] for inbuilt number types. See [`usize::to_ne_bytes`].
    /// 
    /// Size of the output vector shall always be the value of [`Self::BYTES`].
    fn to_ne_bytes(self) -> Vec<u8>;

    /// Creates a native endian value from its memory representation as a byte array in native endianness.
    /// 
    /// Type-generic wrapper around [`from_ne_bytes`] for inbuilt number types. See [`usize::from_ne_bytes`].
    /// 
    /// Returns [`FromBytesError`] if the size of the given byte-slice is incorrect.
    fn from_ne_bytes(bytes: &[u8]) -> FromBytesResult<Self>;

    /// Returns the memory representation of this integer as a byte array in big-endian (network) byte order.
    /// 
    /// Type-generic wrapper around [`to_be_bytes`] for inbuilt number types. See [`usize::to_be_bytes`].
    /// 
    /// Size of the output vector shall always be the value of [`Self::BYTES`].
    fn to_be_bytes(self) -> Vec<u8>;

    /// Creates a native endian integer value from its representation as a byte array in big endian.
    /// 
    /// Type-generic wrapper around [`from_be_bytes`] for inbuilt number types. See [`usize::from_be_bytes`].
    /// 
    /// Returns [`FromBytesError`] if the size of the given byte-slice is incorrect.
    fn from_be_bytes(bytes: &[u8]) -> FromBytesResult<Self>;

    /// Returns the memory representation of this integer as a byte array in little-endian byte order.
    /// 
    /// Type-generic wrapper around [`to_le_bytes`] for inbuilt number types. See [`usize::to_le_bytes`].
    /// 
    /// Size of the output vector shall always be the value of [`Self::BYTES`].
    fn to_le_bytes(self) -> Vec<u8>;

    /// Creates a native endian integer value from its representation as a byte array in little endian.
    /// 
    /// Type-generic wrapper around [`from_le_bytes`] for inbuilt number types. See [`usize::from_le_bytes`].
    /// 
    /// Returns [`FromBytesError`] if the size of the given byte-slice is incorrect.
    fn from_le_bytes(bytes: &[u8]) -> FromBytesResult<Self>;
}

macro_rules! impl_number_bytes {
    ($number_type:ty) => {
        impl_number_bytes!(
            $number_type,
            <$number_type>::BITS as usize / 8
        );
    };

    ($number_type:ty, $bytes:expr) => {
        impl NumberBytes for $number_type {
            const BYTES: usize = $bytes;

            fn from_ne_bytes(bytes: &[u8]) -> FromBytesResult<Self> {
                wrap_from_bytes(bytes, Self::from_ne_bytes)
            }

            fn to_ne_bytes(self) -> Vec<u8> {
                wrap_to_bytes(self, Self::to_ne_bytes)
            }

            fn from_be_bytes(bytes: &[u8]) -> FromBytesResult<Self> {
                wrap_from_bytes(bytes, Self::from_be_bytes)
            }

            fn to_be_bytes(self) -> Vec<u8> {
                wrap_to_bytes(self, Self::to_be_bytes)
            }

            fn from_le_bytes(bytes: &[u8]) -> FromBytesResult<Self> {
                wrap_from_bytes(bytes, Self::from_le_bytes)
            }

            fn to_le_bytes(self) -> Vec<u8> {
                wrap_to_bytes(self, Self::to_le_bytes)
            }
        }
    };
}

#[inline(always)]
fn wrap_from_bytes<T, const BYTES: usize>(bytes: &[u8], from_bytes_fn: fn([u8; BYTES]) -> T) -> FromBytesResult<T> {
    let bytes_array = bytes.try_into()?;
    Ok(from_bytes_fn(bytes_array))
}

#[inline(always)]
fn wrap_to_bytes<T, const BYTES: usize>(instance: T, to_bytes_fn: fn(T) -> [u8; BYTES]) -> Vec<u8> {
    to_bytes_fn(instance).to_vec()
}

impl_number_bytes!(u8);
impl_number_bytes!(u16);
impl_number_bytes!(u32);
impl_number_bytes!(u64);
impl_number_bytes!(u128);
impl_number_bytes!(usize);

impl_number_bytes!(i8);
impl_number_bytes!(i16);
impl_number_bytes!(i32);
impl_number_bytes!(i64);
impl_number_bytes!(i128);
impl_number_bytes!(isize);

// Floating point types don't provide the BITS constant, so we must define the byte count manually.
impl_number_bytes!(f32, 32 / 8);
impl_number_bytes!(f64, 64 / 8);

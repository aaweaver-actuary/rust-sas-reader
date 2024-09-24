use crate::sas::byte_swap::byteswap;
use num::PrimInt;

/// Reads an integer of type T (u16, u32, or u64) from a byte slice with optional byte swapping.
///
/// # Arguments
/// * `data` - A reference to a byte slice containing the data to read.
/// * `bswap` - If true, the data will be byte-swapped to handle endianness.
pub fn sas_read<T>(data: &[u8], bswap: bool) -> T
where
    T: num::Unsigned
        + std::ops::BitOr<Output = T>
        + std::ops::Shl<usize, Output = T>
        + Copy
        + PrimInt,
{
    let size = std::mem::size_of::<T>();
    assert!(
        data.len() >= size,
        "Not enough bytes to read a value of type T"
    );

    let mut tmp: T = T::zero();

    for i in 0..size {
        tmp = tmp << 8;
        tmp = tmp | T::from(data[i]).unwrap();
    }

    if bswap {
        byteswap(tmp)
    } else {
        tmp
    }
}

/// Special case for reading 8-byte (64-bit) integer from a string slice with optional byte swapping.
///
/// # Arguments
/// * `data` - A reference to a string slice containing the data to read.
/// * `bswap` - If true, the data will be byte-swapped to handle endianness.
pub fn sas_read8(data: &str, bswap: bool) -> u64 {
    sas_read(data.as_bytes(), bswap)
}

/// Special case for reading 4-byte (32-bit) integer from a string slice with optional byte swapping.
///
/// # Arguments
/// * `data` - A reference to a string slice containing the data to read.
/// * `bswap` - If true, the data will be byte-swapped to handle endianness.
pub fn sas_read4(data: &str, bswap: bool) -> u32 {
    sas_read(data.as_bytes(), bswap)
}

/// Special case for reading 2-byte (16-bit) integer from a string slice with optional byte swapping.
///
/// # Arguments
/// * `data` - A reference to a string slice containing the data to read.
/// * `bswap` - If true, the data will be byte-swapped to handle endianness.
pub fn sas_read2(data: &str, bswap: bool) -> u16 {
    sas_read(data.as_bytes(), bswap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sas::byte_swap::byteswap;

    #[test]
    fn test_sas_read8() {
        let data = "\x01\x02\x03\x04\x05\x06\x07\x08"; // Represents a u64 value
        let result = sas_read8(data, false);
        assert_eq!(result, 0x0102030405060708);

        let swapped_result = sas_read8(data, true);
        assert_eq!(swapped_result, byteswap(0x0102030405060708));
    }

    #[test]
    fn test_sas_read4() {
        let data = "\x01\x02\x03\x04"; // Represents a u32 value
        let result = sas_read4(data, false);
        assert_eq!(result, 0x01020304);

        let swapped_result = sas_read4(data, true);
        assert_eq!(swapped_result, byteswap(0x01020304));
    }

    #[test]
    fn test_sas_read2() {
        let data = "\x01\x02"; // Represents a u16 value
        let result = sas_read2(data, false);
        assert_eq!(result, 0x0102);

        let swapped_result = sas_read2(data, true);
        assert_eq!(swapped_result, byteswap(0x0102));
    }

    #[test]
    fn test_sas_read_invalid_size() {
        let data = "\x01"; // Not enough bytes for u16, u32, or u64
        let result = std::panic::catch_unwind(|| sas_read2(data, false));
        assert!(
            result.is_err(),
            "Expected an error for insufficient data size"
        );
    }
}

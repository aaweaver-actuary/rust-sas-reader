use num::{Num, PrimInt, Unsigned};
use std::ops::BitOr;

/// Checks if the machine is little-endian by testing the byte order of
/// a 32-bit integer. If the first byte is 1, the machine is little-endian.
/// Otherwise, the machine is big-endian.
pub fn is_machine_little_endian() -> bool {
    let test_byte_order: u32 = 1;
    test_byte_order.to_le_bytes()[0] == 1
}

/// Converts one's complement to two's complement for any signed integer type.
pub fn ones_to_twos_complement<T: Num + PartialOrd>(num: T) -> T {
    if num < T::zero() {
        num + T::one()
    } else {
        num
    }
}

/// Converts two's complement to one's complement for any signed integer type.
pub fn twos_to_ones_complement<T: Num + PartialOrd>(num: T) -> T {
    if num < T::zero() {
        num - T::one()
    } else {
        num
    }
}

/// Byte-swaps a value for any type that supports `swap_bytes`.
pub fn byteswap<T>(num: T) -> T
where
    T: Unsigned + BitOr<Output = T> + Copy + PrimInt,
{
    num.swap_bytes()
}

/// Byte-swaps a 32-bit floating point number.
pub fn byteswap_float(num: f32) -> f32 {
    let bytes = num.to_bits().swap_bytes();
    f32::from_bits(bytes)
}

/// Byte-swaps a 64-bit floating point number.
pub fn byteswap_double(num: f64) -> f64 {
    let bytes = num.to_bits().swap_bytes();
    f64::from_bits(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endianness() {
        assert_eq!(is_machine_little_endian(), true);
    }

    #[test]
    fn test_ones_to_twos_complement() {
        assert_eq!(ones_to_twos_complement(-1_i8), 0);
        assert_eq!(ones_to_twos_complement(-1_i16), 0);
        assert_eq!(ones_to_twos_complement(-1_i32), 0);
    }

    #[test]
    fn test_twos_to_ones_complement() {
        assert_eq!(twos_to_ones_complement(-1_i8), -2);
        assert_eq!(twos_to_ones_complement(-1_i16), -2);
        assert_eq!(twos_to_ones_complement(-1_i32), -2);
    }

    #[test]
    fn test_byteswap() {
        assert_eq!(byteswap(0x1234_u16), 0x3412);
        assert_eq!(byteswap(0x12345678_u32), 0x78563412);
        assert_eq!(byteswap(0x123456789ABCDEF0_u64), 0xF0DEBC9A78563412);
    }

    #[test]
    fn test_byteswap_float() {
        let num: f32 = 1.2345678;
        let swapped = byteswap_float(num);
        assert_eq!(byteswap_float(swapped), num);
    }

    #[test]
    fn test_byteswap_double() {
        let num: f64 = 1.2345678901234567;
        let swapped = byteswap_double(num);
        assert_eq!(byteswap_double(swapped), num);
    }
}

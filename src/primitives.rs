use std::{
    fmt::{Display, LowerHex},
    mem::{align_of, size_of},
    ops::Add,
    usize,
};

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;

#[cfg(not(target_pointer_width = "64"))]
type WordSize = u32;
#[cfg(target_pointer_width = "64")]
type t_word = u64;

const WORD_COUNT: usize = (160 / t_word::BITS + 1) as usize;

#[cfg(target_arch = "x86")]
#[cfg(not(target_pointer_width = "64"))]
#[inline]
fn add_carry(c_in: u8, a: u32, b: u32, out: &mut u32) -> u8 {
    unsafe { arch::_addcarry_u32(c_in, a, b, out) }
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_pointer_width = "64")]
#[inline]
fn add_carry(c_in: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    unsafe { arch::_addcarry_u64(c_in, a, b, out) }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn add_carry(c_in: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    let (a, b) = a.overflowing_add(b);
    let (c, d) = a.overflowing_add(c_in as t_word);
    *out = c;
    u8::from(b || d)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct GUID {
    bytes: [t_word; WORD_COUNT],
}

impl GUID {
    const MIN: GUID = GUID {
        bytes: [0; WORD_COUNT],
    };

    const MAX: GUID = GUID {
        bytes: [t_word::MAX; WORD_COUNT],
    };

    fn saturating_add(self, rhs: Self) -> Self {
        todo!()
    }

    fn from_bytes_be(bytes: &[u8]) -> Self {
        let word_size = size_of::<t_word>();

        assert!(bytes.len() <= WORD_COUNT * word_size);

        let mut guid = GUID::MIN;
        let mut offset = 0;

        for (j, byte) in bytes.iter().rev().enumerate() {
            guid.bytes[j / word_size] |= (*byte as t_word) << offset;
            offset = (offset + 8) % t_word::BITS;
        }

        guid
    }

    fn from_bytes_le(bytes: &[u8]) -> Self {
        let word_size = size_of::<t_word>();

        assert!(bytes.len() <= WORD_COUNT * word_size);

        let mut guid = GUID::MIN;
        let mut offset = 0;

        for (j, byte) in bytes.iter().enumerate() {
            guid.bytes[j / word_size] |= (*byte as t_word) << offset;
            offset = (offset + 8) % t_word::BITS;
        }

        guid
    }
}

impl LowerHex for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hex = String::with_capacity(WORD_COUNT * size_of::<t_word>());

        for byte in self.bytes.iter().filter(|b| **b > 0) {
            hex.push_str(&format!("{byte:x}"));
        }

        if hex.is_empty() {
            hex.push('0')
        }

        write!(f, "{hex}")
    }
}

impl From<u8> for GUID {
    fn from(value: u8) -> Self {
        Self::from_bytes_be(&value.to_be_bytes())
    }
}

impl From<u16> for GUID {
    fn from(value: u16) -> Self {
        Self::from_bytes_be(&value.to_be_bytes())
    }
}

impl From<u32> for GUID {
    fn from(value: u32) -> Self {
        Self::from_bytes_be(&value.to_be_bytes())
    }
}

impl From<u64> for GUID {
    fn from(value: u64) -> Self {
        Self::from_bytes_be(&value.to_be_bytes())
    }
}

impl From<u128> for GUID {
    fn from(value: u128) -> Self {
        Self::from_bytes_be(&value.to_be_bytes())
    }
}

#[cfg(test)]
pub mod test {
    use super::GUID;

    #[test]
    fn from_bytes() {
        let guid_be = GUID::from_bytes_be(&42u32.to_be_bytes());
        let guid_le = GUID::from_bytes_le(&42u32.to_le_bytes());

        assert_eq!(format!("{guid_be:x}"), format!("{guid_le:x}"));
        assert_eq!(format!("{guid_be:x}"), format!("{:x}", 42));
    }

    #[test]
    fn empty() {
        let guid = GUID::default();
        assert_eq!(format!("{guid:x}"), "0");
    }

    #[test]
    fn from_u8() {
        let guid = GUID::from(u8::MAX);
        assert_eq!(format!("{guid:x}"), format!("{:x}", u8::MAX));
    }

    #[test]
    fn from_u16() {
        let guid = GUID::from(u16::MAX);
        println!("{:?}", guid.bytes);
        assert_eq!(format!("{guid:x}"), format!("{:x}", u16::MAX));
    }

    #[test]
    fn from_u32() {
        let guid = GUID::from(u32::MAX);
        assert_eq!(format!("{guid:x}"), format!("{:x}", u32::MAX));
    }

    #[test]
    fn from_u64() {
        let guid = GUID::from(u64::MAX);
        assert_eq!(format!("{guid:x}"), format!("{:x}", u64::MAX));
    }

    #[test]
    fn from_u128() {
        let guid = GUID::from(u128::MAX);
        assert_eq!(format!("{guid:x}"), format!("{:x}", u128::MAX));
    }
}

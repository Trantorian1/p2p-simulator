use std::{
    fmt::{Display, LowerHex},
    mem::{align_of, size_of},
    ops::{Add, AddAssign, Sub, SubAssign},
    usize,
};

use crate::primitives::{add::add_carry, sub::sub_carry};

use super::{t_word, WORD_COUNT};

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

    fn saturating_add(&self, rhs: &Self) -> Self {
        let mut result = GUID::default();
        let mut carry = 0;

        let bytes_a = self.bytes;
        let bytes_b = rhs.bytes;
        let bytes_c = &mut result.bytes;

        #[cfg(target_pointer_width = "64")]
        {
            carry = add_carry(carry, bytes_a[2], bytes_b[2], &mut bytes_c[2]);
            carry = add_carry(carry, bytes_a[1], bytes_b[1], &mut bytes_c[1]);
            carry = add_carry(carry, bytes_a[0], bytes_b[0], &mut bytes_c[0]);
        }
        #[cfg(not(target_pointer_width = "64"))]
        {
            carry = add_carry(carry, bytes_a[5], bytes_b[5], &mut bytes_c[5]);
            carry = add_carry(carry, bytes_a[4], bytes_b[4], &mut bytes_c[4]);
            carry = add_carry(carry, bytes_a[3], bytes_b[3], &mut bytes_c[3]);
            carry = add_carry(carry, bytes_a[2], bytes_b[2], &mut bytes_c[2]);
            carry = add_carry(carry, bytes_a[1], bytes_b[1], &mut bytes_c[1]);
            carry = add_carry(carry, bytes_a[0], bytes_b[0], &mut bytes_c[0]);
        }

        if carry > 0 {
            GUID::MAX
        } else {
            result
        }
    }

    fn saturating_sub(&self, rhs: &Self) -> Self {
        let mut result = GUID::default();
        let mut carry = 0;

        let bytes_a = self.bytes;
        let bytes_b = rhs.bytes;
        let bytes_c = &mut result.bytes;

        #[cfg(target_pointer_width = "64")]
        {
            carry = sub_carry(carry, bytes_a[2], bytes_b[2], &mut bytes_c[2]);
            carry = sub_carry(carry, bytes_a[1], bytes_b[1], &mut bytes_c[1]);
            carry = sub_carry(carry, bytes_a[0], bytes_b[0], &mut bytes_c[0]);
        }
        #[cfg(not(target_pointer_width = "64"))]
        {
            carry = sub_carry(carry, bytes_a[5], bytes_b[5], &mut bytes_c[5]);
            carry = sub_carry(carry, bytes_a[4], bytes_b[4], &mut bytes_c[4]);
            carry = sub_carry(carry, bytes_a[3], bytes_b[3], &mut bytes_c[3]);
            carry = sub_carry(carry, bytes_a[2], bytes_b[2], &mut bytes_c[2]);
            carry = sub_carry(carry, bytes_a[1], bytes_b[1], &mut bytes_c[1]);
            carry = sub_carry(carry, bytes_a[0], bytes_b[0], &mut bytes_c[0]);
        }

        if carry > 0 {
            GUID::MIN
        } else {
            result
        }
    }

    fn from_bytes_be(bytes: &[u8]) -> Self {
        let word_size = size_of::<t_word>();

        assert!(bytes.len() <= WORD_COUNT * word_size);

        let mut guid = GUID::MIN;
        let mut offset = 0;

        for (i, byte) in bytes.iter().rev().enumerate() {
            let j = WORD_COUNT - 1 - i / word_size;
            let byte_offset = (*byte as t_word) << offset;

            guid.bytes[j] |= byte_offset;
            offset = (offset + 8) % t_word::BITS;
        }

        guid
    }

    fn from_bytes_le(bytes: &[u8]) -> Self {
        let word_size = size_of::<t_word>();

        assert!(bytes.len() <= WORD_COUNT * word_size);

        let mut guid = GUID::MIN;
        let mut offset = 0;

        for (i, byte) in bytes.iter().enumerate() {
            let j = WORD_COUNT - 1 - i / word_size;
            let byte_offset = (*byte as t_word) << offset;

            guid.bytes[j] |= byte_offset;
            offset = (offset + 8) % t_word::BITS;
        }

        guid
    }
}

impl Add for GUID {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.saturating_add(&rhs)
    }
}

impl AddAssign for GUID {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.saturating_add(&rhs);
    }
}

impl Sub for GUID {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(&rhs)
    }
}

impl SubAssign for GUID {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.saturating_sub(&rhs);
    }
}

impl LowerHex for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hex = String::with_capacity(WORD_COUNT * size_of::<t_word>());

        for byte in self.bytes.iter().skip_while(|b| **b == 0) {
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
    fn saturating_add() {
        let guid_a = GUID::from(u128::MAX);
        let guid_b = GUID::from(u128::MAX);
        let guid_c = guid_a.saturating_add(&guid_b);

        assert_eq!(format!("{guid_c:x}"), "1fffffffffffffffffffffffffffffffe");
    }

    #[test]
    fn saturating_add_saturate() {
        let guid_a = GUID::MAX;
        let guid_b = GUID::from(1u32);
        let guid_c = guid_a.saturating_add(&guid_b);

        assert_eq!(guid_c, GUID::MAX);
    }

    #[test]
    fn saturating_sub() {
        let guid_a = GUID::from(u128::MAX) + GUID::from(u128::MAX);
        let guid_b = GUID::from(u128::MAX / 2);
        let guid_c = guid_a.saturating_sub(&guid_b);

        assert_eq!(format!("{guid_c:x}"), "17fffffffffffffffffffffffffffffff");
    }

    #[test]
    fn saturating_sub_saturate() {
        let guid_a = GUID::MIN;
        let guid_b = GUID::from(1u32);
        let guid_c = guid_a.saturating_sub(&guid_b);

        assert_eq!(format!("{guid_c:x}"), "0");
    }

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
        println!("bytes: {:?}", guid.bytes);
        assert_eq!(format!("{guid:x}"), format!("{:x}", u128::MAX));
    }
}

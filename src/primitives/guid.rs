use std::mem::size_of;

use crate::primitives::{add::add_carry, sub::sub_carry};

use super::{t_word, GuidError, WORD_COUNT};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct GUID {
    bytes: [t_word; WORD_COUNT],
}

impl GUID {
    const MIN: GUID = GUID {
        bytes: [0; WORD_COUNT],
    };

    const MAX: GUID = GUID {
        // TODO: do the same for 32 bit
        bytes: [4294967295, 18446744073709551615, 18446744073709551615],
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

    fn from_hex_str(hex: &str) -> Result<Self, GuidError> {
        ///////////////////////////////////////////////////////////////////////
        //////////////////////////// START: HELPERS ///////////////////////////
        ///////////////////////////////////////////////////////////////////////
        #[inline(always)]
        fn conv(c: char) -> u8 {
            if c.is_ascii_digit() {
                c as u8 - '0' as u8
            } else {
                c as u8 - 'a' as u8 + 10
            }
        }

        #[inline(always)]
        fn check(c: char) -> Result<(), GuidError> {
            if !c.is_ascii_hexdigit() {
                return Err(GuidError::HexFormatInvalid);
            } else {
                Ok(())
            }
        }
        ///////////////////////////////////////////////////////////////////////
        ///////////////////////////// END: HELPERS ////////////////////////////
        ///////////////////////////////////////////////////////////////////////

        if hex.is_empty() {
            return Err(GuidError::HexFormatEmpty);
        }

        let n = hex.find("0x").map(|n| n + 2).unwrap_or_default();
        let hex = &hex.to_lowercase()[n..];
        let n = hex.find(|c| c != '0').map(|n| n).unwrap_or_default();

        let len = hex.len() - n;
        let mut iter = hex.chars().skip(n);
        let mut bytes = vec![0; len / 2 + len % 2];
        let mut i = 0;

        if len % 2 != 0 {
            if let Some(c_a) = iter.next() {
                check(c_a)?;

                bytes[i] = conv(c_a);
                i = 1;
            }
        }

        while let Some(c_a) = iter.next() {
            if let Some(c_b) = iter.next() {
                check(c_a)?;
                check(c_b)?;

                bytes[i] = conv(c_a) << 4 | conv(c_b);
                i += 1;
            }
        }

        Ok(Self::from_bytes_be(&bytes))
    }
}

impl std::ops::Add for GUID {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.saturating_add(&rhs)
    }
}

impl std::ops::AddAssign for GUID {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.saturating_add(&rhs);
    }
}

impl std::ops::Sub for GUID {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(&rhs)
    }
}

impl std::ops::SubAssign for GUID {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.saturating_sub(&rhs);
    }
}

impl std::fmt::UpperHex for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hex = String::new();
        let mut iter = self.bytes.iter().skip_while(|b| **b == 0);

        if let Some(byte) = iter.next() {
            hex.push_str(&format!("{byte:X}"));
        }

        let target: t_word = 1 << (t_word::BITS - 3);

        for byte in iter {
            if *byte == 0 {
                hex.push_str(&"0".repeat(size_of::<t_word>() * 2));
            } else if *byte < target {
                hex.push_str(&"0".repeat((byte.leading_zeros() / 4) as usize));
                hex.push_str(&format!("{byte:X}"));
            } else {
                hex.push_str(&format!("{byte:X}"));
            }
        }

        if hex.is_empty() {
            hex.push('0')
        }

        write!(f, "{hex}")
    }
}

impl std::fmt::LowerHex for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ///////////////////////////////////////////////////////////////////////
        //////////////////////////// START: HELPERS ///////////////////////////
        ///////////////////////////////////////////////////////////////////////
        #[inline(always)]
        fn hex(byte: t_word, target: t_word) -> String {
            if byte == 0 {
                let utf8 = vec![b'0'; size_of::<t_word>() * 2];
                unsafe { String::from_utf8_unchecked(utf8) }
            } else if byte < target {
                let utf8 = vec![b'0'; (byte.leading_zeros() / 4) as usize];
                let padding = unsafe { String::from_utf8_unchecked(utf8) };
                format!("{padding}{byte:x}")
            } else {
                format!("{byte:x}")
            }
        }
        ///////////////////////////////////////////////////////////////////////
        ///////////////////////////// END: HELPERS ////////////////////////////
        ///////////////////////////////////////////////////////////////////////

        let target: t_word = 1 << (t_word::BITS - 3);
        let bytes = self.bytes;

        #[cfg(target_pointer_width = "64")]
        return {
            if bytes[0] > 0 {
                write!(
                    f,
                    "{:x}{}{}",
                    bytes[0],
                    hex(bytes[1], target),
                    hex(bytes[2], target)
                )
            } else if bytes[1] > 0 {
                write!(f, "{:x}{}", bytes[1], hex(bytes[2], target))
            } else if bytes[2] > 0 {
                write!(f, "{:x}", bytes[2])
            } else {
                write!(f, "0")
            }
        };
        #[cfg(not(target_pointer_width = "64"))]
        return {
            if bytes[0] > 0 {
                write!(
                    f,
                    "{:x}{}{}{}{}{}",
                    bytes[0],
                    hex(bytes[1], target),
                    hex(bytes[2], target),
                    hex(bytes[3], target),
                    hex(bytes[4], target),
                    hex(bytes[5], target)
                )
            } else if bytes[1] > 0 {
                write!(
                    f,
                    "{:x}{}{}{}{}",
                    bytes[1],
                    hex(bytes[2], target),
                    hex(bytes[3], target),
                    hex(bytes[4], target),
                    hex(bytes[5], target)
                )
            } else if bytes[2] > 0 {
                write!(
                    f,
                    "{:x}{}{}{}",
                    bytes[2],
                    hex(bytes[3], target),
                    hex(bytes[4], target),
                    hex(bytes[5], target)
                )
            } else if bytes[3] > 0 {
                write!(
                    f,
                    "{:x}{}{}",
                    bytes[3],
                    hex(bytes[4], target),
                    hex(bytes[5], target)
                )
            } else if bytes[4] > 0 {
                write!(f, "{:x}{}", bytes[4], hex(bytes[5], target))
            } else if bytes[5] > 0 {
                write!(f, "{:x}", bytes[5])
            } else {
                write!(f, "0")
            }
        };
    }
}

impl std::fmt::Binary for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hex = String::new();
        let mut iter = self.bytes.iter().skip_while(|b| **b == 0);

        if let Some(byte) = iter.next() {
            hex.push_str(&format!("{byte:b}"));
        }

        let target: t_word = 1 << (t_word::BITS - 1);

        for byte in iter {
            if *byte == 0 {
                hex.push_str(&"0".repeat(t_word::BITS as usize));
            } else if *byte < target {
                hex.push_str(&"0".repeat(byte.leading_zeros() as usize));
                hex.push_str(&format!("{byte:b}"));
            } else {
                hex.push_str(&format!("{byte:b}"));
            }
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
    use std::mem::size_of;

    use crate::primitives::GuidError;

    use super::GUID;

    #[test]
    fn display_hex_lower() {
        println!("{}", size_of::<GUID>());

        let guid_a = GUID::MAX;
        assert_eq!(
            format!("{guid_a:x}"),
            "ffffffffffffffffffffffffffffffffffffffff"
        );

        let guid_b = GUID::from(1u32);
        assert_eq!(format!("{guid_b:x}"), "1");

        let guid_c = GUID::from(u128::MAX) + GUID::from(2u32);
        assert_eq!(format!("{guid_c:x}"), "100000000000000000000000000000001");

        let guid_d = GUID::from_hex_str("110000001000000010000000100000001").unwrap();
        assert_eq!(format!("{guid_d:x}"), "110000001000000010000000100000001");

        let guid_e = GUID::from_hex_str("10000001100000011000000110000001").unwrap();
        assert_eq!(format!("{guid_e:x}"), "10000001100000011000000110000001");

        let guid_f = GUID::from(0u32);
        assert_eq!(format!("{guid_f:x}"), "0");
    }

    #[test]
    fn display_hex_upper() {
        let guid_a = GUID::MAX;
        assert_eq!(
            format!("{guid_a:X}"),
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        );

        let guid_b = GUID::from(1u32);
        assert_eq!(format!("{guid_b:X}"), "1");

        let guid_c = GUID::from(u128::MAX) + GUID::from(2u32);
        assert_eq!(format!("{guid_c:X}"), "100000000000000000000000000000001");

        let guid_d = GUID::from_hex_str("110000001000000010000000100000001").unwrap();
        assert_eq!(format!("{guid_d:X}"), "110000001000000010000000100000001");

        let guid_e = GUID::from_hex_str("10000001100000011000000110000001").unwrap();
        assert_eq!(format!("{guid_e:X}"), "10000001100000011000000110000001");

        let guid_f = GUID::from(0u32);
        assert_eq!(format!("{guid_f:X}"), "0");
    }

    #[test]
    fn display_binary() {
        let guid_a = GUID::MAX;
        assert_eq!(
            format!("{guid_a:b}"),
            "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111"
        );

        let guid_b = GUID::from(1u32);
        assert_eq!(format!("{guid_b:b}"), "1");

        let guid_c = GUID::from(u128::MAX) + GUID::from(2u32);
        assert_eq!(format!("{guid_c:b}"), "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");

        let guid_d = GUID::from_hex_str("110000001000000010000000100000001").unwrap();
        assert_eq!(format!("{guid_d:b}"), "100010000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");

        let guid_e = GUID::from_hex_str("10000001100000011000000110000001").unwrap();
        assert_eq!(format!("{guid_e:b}"), "10000000000000000000000000001000100000000000000000000000000010001000000000000000000000000000100010000000000000000000000000001");

        let guid_f = GUID::from(0u32);
        assert_eq!(format!("{guid_f:b}"), "0");
    }

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
    fn from_hex_str() {
        assert_eq!(Ok(GUID::from(0u32)), GUID::from_hex_str("0"));
        assert_eq!(Ok(GUID::from(1u32)), GUID::from_hex_str("1"));
        assert_eq!(Ok(GUID::from(2u32)), GUID::from_hex_str("2"));
        assert_eq!(Ok(GUID::from(3u32)), GUID::from_hex_str("3"));
        assert_eq!(Ok(GUID::from(4u32)), GUID::from_hex_str("4"));
        assert_eq!(Ok(GUID::from(5u32)), GUID::from_hex_str("5"));
        assert_eq!(Ok(GUID::from(6u32)), GUID::from_hex_str("6"));
        assert_eq!(Ok(GUID::from(7u32)), GUID::from_hex_str("7"));
        assert_eq!(Ok(GUID::from(8u32)), GUID::from_hex_str("8"));
        assert_eq!(Ok(GUID::from(9u32)), GUID::from_hex_str("9"));
        assert_eq!(Ok(GUID::from(10u32)), GUID::from_hex_str("a"));
        assert_eq!(Ok(GUID::from(11u32)), GUID::from_hex_str("b"));
        assert_eq!(Ok(GUID::from(12u32)), GUID::from_hex_str("c"));
        assert_eq!(Ok(GUID::from(13u32)), GUID::from_hex_str("d"));
        assert_eq!(Ok(GUID::from(14u32)), GUID::from_hex_str("e"));
        assert_eq!(Ok(GUID::from(15u32)), GUID::from_hex_str("f"));
    }

    #[test]
    fn from_hex_str_0x() {
        assert_eq!(Ok(GUID::from(0u32)), GUID::from_hex_str("0x0"));
        assert_eq!(Ok(GUID::from(1u32)), GUID::from_hex_str("0x1"));
        assert_eq!(Ok(GUID::from(2u32)), GUID::from_hex_str("0x2"));
        assert_eq!(Ok(GUID::from(3u32)), GUID::from_hex_str("0x3"));
        assert_eq!(Ok(GUID::from(4u32)), GUID::from_hex_str("0x4"));
        assert_eq!(Ok(GUID::from(5u32)), GUID::from_hex_str("0x5"));
        assert_eq!(Ok(GUID::from(6u32)), GUID::from_hex_str("0x6"));
        assert_eq!(Ok(GUID::from(7u32)), GUID::from_hex_str("0x7"));
        assert_eq!(Ok(GUID::from(8u32)), GUID::from_hex_str("0x8"));
        assert_eq!(Ok(GUID::from(9u32)), GUID::from_hex_str("0x9"));
        assert_eq!(Ok(GUID::from(10u32)), GUID::from_hex_str("0xa"));
        assert_eq!(Ok(GUID::from(11u32)), GUID::from_hex_str("0xb"));
        assert_eq!(Ok(GUID::from(12u32)), GUID::from_hex_str("0xc"));
        assert_eq!(Ok(GUID::from(13u32)), GUID::from_hex_str("0xd"));
        assert_eq!(Ok(GUID::from(14u32)), GUID::from_hex_str("0xe"));
        assert_eq!(Ok(GUID::from(15u32)), GUID::from_hex_str("0xf"));
    }

    #[test]
    fn from_hex_str_upper() {
        assert_eq!(Ok(GUID::from(10u32)), GUID::from_hex_str("A"));
        assert_eq!(Ok(GUID::from(11u32)), GUID::from_hex_str("B"));
        assert_eq!(Ok(GUID::from(12u32)), GUID::from_hex_str("C"));
        assert_eq!(Ok(GUID::from(13u32)), GUID::from_hex_str("D"));
        assert_eq!(Ok(GUID::from(14u32)), GUID::from_hex_str("E"));
        assert_eq!(Ok(GUID::from(15u32)), GUID::from_hex_str("F"));
    }

    #[test]
    fn from_hex_str_padded() {
        assert_eq!(Ok(GUID::from(100u32)), GUID::from_hex_str("00064"));
    }

    #[test]
    fn from_hex_invalid() {
        assert_eq!(GUID::from_hex_str(""), Err(GuidError::HexFormatEmpty));
        assert_eq!(GUID::from_hex_str("z"), Err(GuidError::HexFormatInvalid));
    }

    #[test]
    fn from_hex_long() {
        let guid_a = GUID::from_hex_str("1fffffffffffffffffffffffffffffffe").unwrap_or_default();
        let guid_b = GUID::from(u128::MAX) + GUID::from(u128::MAX);

        assert_eq!(format!("{guid_a:x}"), format!("{guid_b:x}"));
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

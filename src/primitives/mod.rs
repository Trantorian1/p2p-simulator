mod add;
mod guid;
mod sub;

pub use guid::GUID;

#[cfg(not(target_pointer_width = "64"))]
type WordSize = u32;
#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
type t_word = u64;

const WORD_COUNT: usize = (160 / t_word::BITS + 1) as usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GuidError {
    HexFormatInvalid,
    HexFormatEmpty,
}

impl std::fmt::Display for GuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuidError::HexFormatInvalid => {
                write!(
                    f,
                    concat!(
                        "Invalid hex format, ",
                        "a GUID can only be constructed from a pure hex string"
                    )
                )
            }
            GuidError::HexFormatEmpty => {
                write!(
                    f,
                    concat!(
                        "Invalid hex format, ",
                        "a GUID cannot be constructed from an empty string"
                    )
                )
            }
        }
    }
}

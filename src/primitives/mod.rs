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

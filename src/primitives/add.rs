#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

#[cfg(target_arch = "x86")]
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub(super) fn add_carry(c_in: u8, a: u32, b: u32, out: &mut u32) -> u8 {
    unsafe { arch::_addcarry_u32(c_in, a, b, out) }
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_pointer_width = "64")]
#[inline]
pub(super) fn add_carry(c_in: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    unsafe { arch::_addcarry_u64(c_in, a, b, out) }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub(super) fn add_carry(c_in: u8, a: u32, b: u32, out: &mut u32) -> u8 {
    use super::t_word;

    let (a, b) = a.overflowing_add(b);
    let (c, d) = a.overflowing_add(c_in as t_word);
    *out = c;
    u8::from(b || d)
}

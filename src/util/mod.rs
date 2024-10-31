#[macro_use]
pub mod dictionary;
pub mod channel_layout;
pub mod chroma;
pub mod color;
pub mod error;
pub mod format;
pub mod frame;
pub mod interrupt;
pub mod log;
pub mod mathematics;
pub mod media;
pub mod option;
pub mod picture;
pub mod rational;
pub mod time;

use crate::ffi::*;
use crate::utils;

#[inline(always)]
pub fn version() -> u32 {
    unsafe { avutil_version() }
}

#[inline(always)]
pub fn configuration() -> &'static str {
    unsafe { utils::str_from_c_ptr(avutil_configuration()) }
}

#[inline(always)]
pub fn license() -> &'static str {
    unsafe { utils::str_from_c_ptr(avutil_license()) }
}

pub mod flag;
pub use self::flag::Flags;

pub mod color_space;
pub use self::color_space::ColorSpace;

pub mod support;

pub mod vector;
pub use self::vector::Vector;

pub mod filter;
pub use self::filter::Filter;

pub mod context;
pub use self::context::Context;

mod extensions;

use crate::ffi::*;
use crate::utils;

pub fn version() -> u32 {
    unsafe { swscale_version() }
}

pub fn configuration() -> &'static str {
    unsafe { utils::str_from_c_ptr(swscale_configuration()) }
}

pub fn license() -> &'static str {
    unsafe { utils::str_from_c_ptr(swscale_license()) }
}

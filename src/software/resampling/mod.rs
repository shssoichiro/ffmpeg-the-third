pub mod flag;
pub use self::flag::Flags;

pub mod dither;
pub use self::dither::Dither;

pub mod engine;
pub use self::engine::Engine;

pub mod filter;
pub use self::filter::Filter;

pub mod delay;
pub use self::delay::Delay;

pub mod context;
pub use self::context::Context;

mod extensions;

use crate::ffi::*;
use crate::utils;

pub fn version() -> u32 {
    unsafe { swresample_version() }
}

pub fn configuration() -> &'static str {
    unsafe { utils::str_from_c_ptr(swresample_configuration()) }
}

pub fn license() -> &'static str {
    unsafe { utils::str_from_c_ptr(swresample_license()) }
}

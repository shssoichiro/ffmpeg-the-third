#[cfg(feature = "ffmpeg_8_1")]
pub use crate::util::format::AlphaMode;
pub use crate::util::format::{pixel, Pixel};
pub use crate::util::format::{sample, Sample};

pub mod interrupt;

pub mod stream;

pub mod chapter;

pub mod context;
use context::builder::InputBuilder;

pub mod format;
pub use self::format::{flag, Flags};
pub use self::format::{Input, Output};

pub mod network;

use std::ffi::{CString, OsStr};
use std::ptr;

use crate::ffi::*;
use crate::utils;
use crate::{AsMutPtr, Error};

pub fn version() -> u32 {
    unsafe { avformat_version() }
}

pub fn configuration() -> &'static str {
    unsafe { utils::str_from_c_ptr(avformat_configuration()) }
}

pub fn license() -> &'static str {
    unsafe { utils::str_from_c_ptr(avformat_license()) }
}

pub fn input<'d, P: AsRef<OsStr>>(path_or_url: P) -> InputBuilder<'d> {
    InputBuilder::new(path_or_url)
}

pub fn input_with_interrupt<P, F>(path_or_url: P, closure: F) -> Result<context::Input, Error>
where
    P: AsRef<OsStr>,
    F: FnMut() -> bool,
{
    unsafe {
        let mut ps = avformat_alloc_context();
        let path = from_os_str(path_or_url);
        (*ps).interrupt_callback = interrupt::new(Box::new(closure)).interrupt;

        match avformat_open_input(&mut ps, path.as_ptr(), ptr::null_mut(), ptr::null_mut()) {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::from_raw(ps).expect("ps is non-null")),
                e => {
                    avformat_close_input(&mut ps);
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

fn from_os_str(path_or_url: impl AsRef<OsStr>) -> CString {
    CString::new(path_or_url.as_ref().as_encoded_bytes()).unwrap()
}

pub fn output<P: AsRef<OsStr>>(path_or_url: P) -> Result<context::Output, Error> {
    let mut ctx = context::Output::from_filename(&path_or_url)?;
    ctx.open_file(path_or_url)?;
    Ok(ctx)
}

pub fn output_with<P, Dict>(path_or_url: P, options: Dict) -> Result<context::Output, Error>
where
    P: AsRef<OsStr>,
    Dict: AsMutPtr<*mut AVDictionary>,
{
    let mut ctx = context::Output::from_filename(&path_or_url)?;
    ctx.open_file_with(path_or_url, options)?;
    Ok(ctx)
}

pub fn output_as<P: AsRef<OsStr>>(path_or_url: P, format: &str) -> Result<context::Output, Error> {
    let mut ctx = context::Output::from_format_name(format)?;
    ctx.open_file(path_or_url)?;
    Ok(ctx)
}

pub fn output_as_with<P, Dict>(
    path_or_url: P,
    format: &str,
    options: Dict,
) -> Result<context::Output, Error>
where
    P: AsRef<OsStr>,
    Dict: AsMutPtr<*mut AVDictionary>,
{
    let mut ctx = context::Output::from_format_name(format)?;
    ctx.open_file_with(path_or_url, options)?;
    Ok(ctx)
}

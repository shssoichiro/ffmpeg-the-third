#[cfg(feature = "ffmpeg_8_1")]
pub use crate::util::format::AlphaMode;
pub use crate::util::format::{pixel, Pixel};
pub use crate::util::format::{sample, Sample};

pub mod interrupt;

pub mod stream;

pub mod chapter;

pub mod context;

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

pub fn input<P: AsRef<OsStr>>(path_or_url: P) -> Result<context::Input, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_os_str(path_or_url);

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

pub fn input_with_dictionary<P, Dict>(
    path_or_url: P,
    mut options: Dict,
) -> Result<context::Input, Error>
where
    Dict: AsMutPtr<*mut AVDictionary>,
    P: AsRef<OsStr>,
{
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_os_str(path_or_url);
        let res = avformat_open_input(
            &mut ps,
            path.as_ptr(),
            ptr::null_mut(),
            options.as_mut_ptr(),
        );

        match res {
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

fn alloc_context(
    format_name: *const libc::c_char,
    filename: *const libc::c_char,
) -> Result<context::Output, Error> {
    let mut ps = ptr::null_mut();

    unsafe {
        let res = avformat_alloc_output_context2(&mut ps, ptr::null(), format_name, filename);
        if res >= 0 {
            Ok(context::Output::from_raw(ps).expect("ps is non-null"))
        } else {
            Err(Error::from(res))
        }
    }
}

fn open_context_write(
    ctx: &mut context::Output,
    filename: *const libc::c_char,
    opts: *mut *mut AVDictionary,
) -> Result<(), Error> {
    let res = unsafe {
        avio_open2(
            &mut (*ctx.as_mut_ptr()).pb,
            filename,
            AVIO_FLAG_WRITE,
            ptr::null(),
            opts,
        )
    };

    if res >= 0 {
        Ok(())
    } else {
        Err(Error::from(res))
    }
}

pub fn output<P: AsRef<OsStr>>(path_or_url: P) -> Result<context::Output, Error> {
    let filename = from_os_str(path_or_url);
    let mut ctx = alloc_context(ptr::null(), filename.as_ptr())?;

    if !ctx.format().flags().contains(Flags::NO_FILE) {
        open_context_write(&mut ctx, filename.as_ptr(), ptr::null_mut())?;
    }

    Ok(ctx)
}

pub fn output_with<P, Dict>(path_or_url: P, mut options: Dict) -> Result<context::Output, Error>
where
    P: AsRef<OsStr>,
    Dict: AsMutPtr<*mut AVDictionary>,
{
    let path = from_os_str(path_or_url);
    let mut ctx = alloc_context(ptr::null(), path.as_ptr())?;

    if !ctx.format().flags().contains(Flags::NO_FILE) {
        open_context_write(&mut ctx, path.as_ptr(), options.as_mut_ptr())?;
    }

    Ok(ctx)
}

pub fn output_as<P: AsRef<OsStr>>(path_or_url: P, format: &str) -> Result<context::Output, Error> {
    let path = from_os_str(path_or_url);
    let format = CString::new(format).unwrap();
    let mut ctx = alloc_context(format.as_ptr(), path.as_ptr())?;

    if !ctx.format().flags().contains(Flags::NO_FILE) {
        open_context_write(&mut ctx, path.as_ptr(), ptr::null_mut())?;
    }

    Ok(ctx)
}

pub fn output_as_with<P, Dict>(
    path_or_url: P,
    format: &str,
    mut options: Dict,
) -> Result<context::Output, Error>
where
    P: AsRef<OsStr>,
    Dict: AsMutPtr<*mut AVDictionary>,
{
    let path = from_os_str(path_or_url);
    let format = CString::new(format).unwrap();
    let mut ctx = alloc_context(format.as_ptr(), path.as_ptr())?;

    if !ctx.format().flags().contains(Flags::NO_FILE) {
        open_context_write(&mut ctx, path.as_ptr(), options.as_mut_ptr())?;
    }

    Ok(ctx)
}

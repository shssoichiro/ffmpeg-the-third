//! NOTE: this will be much better once specialization comes

use std::ffi::CString;
use std::mem;

use crate::ffi::*;
use crate::util::format;
use crate::{AsMutPtr, AsPtr, Error, Rational};
use libc::c_int;

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;

macro_rules! check {
    ($expr:expr) => {
        match $expr {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    };
}

pub trait Settable<T>: AsPtr<T> + AsMutPtr<T> {
    fn set<V: 'static>(&mut self, name: &str, value: &V) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_bin(
                self.as_mut_ptr() as *mut _,
                name.as_ptr(),
                value as *const _ as *const _,
                mem::size_of::<V>() as c_int,
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();
            let value = CString::new(value).unwrap();

            check!(av_opt_set(
                self.as_mut_ptr() as *mut _,
                name.as_ptr(),
                value.as_ptr(),
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_int(&mut self, name: &str, value: i64) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_int(
                self.as_mut_ptr() as *mut _,
                name.as_ptr(),
                value,
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_double(&mut self, name: &str, value: f64) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_double(
                self.as_mut_ptr() as *mut _,
                name.as_ptr(),
                value,
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_rational<V: Into<Rational>>(&mut self, name: &str, value: V) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_q(
                self.as_mut_ptr() as *mut _,
                name.as_ptr(),
                value.into().into(),
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_image_size(&mut self, name: &str, w: u32, h: u32) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_image_size(
                self.as_mut_ptr() as *mut _,
                name.as_ptr(),
                w as c_int,
                h as c_int,
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_pixel_format(&mut self, name: &str, format: format::Pixel) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_pixel_fmt(
                self.as_mut_ptr() as *mut _,
                name.as_ptr(),
                format.into(),
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_sample_format(&mut self, name: &str, format: format::Sample) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_sample_fmt(
                self.as_mut_ptr() as *mut _,
                name.as_ptr(),
                format.into(),
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    fn set_channel_layout(&mut self, name: &str, layout: ChannelLayoutMask) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_channel_layout(
                self.as_mut_ptr() as *mut _,
                name.as_ptr(),
                layout.bits() as i64,
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }
}

pub trait Gettable<T>: AsPtr<T> + AsMutPtr<T> {}

pub trait Iterable<T>: AsPtr<T> + AsMutPtr<T> {}

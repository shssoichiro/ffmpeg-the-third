use std::path::Path;

use std::ffi::CString;
use std::ptr::{self, NonNull};

use super::Flags;
use crate::ffi::*;
use crate::{codec, media, utils};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Output {
    ptr: NonNull<AVOutputFormat>,
}

impl Output {
    pub unsafe fn from_raw(ptr: *const AVOutputFormat) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self { ptr })
    }

    pub fn as_ptr(self) -> *const AVOutputFormat {
        self.ptr.as_ptr()
    }

    pub fn name(self) -> &'static str {
        unsafe { utils::str_from_c_ptr((*self.as_ptr()).name) }
    }

    pub fn description(self) -> &'static str {
        unsafe { utils::optional_str_from_c_ptr((*self.as_ptr()).long_name).unwrap_or("") }
    }

    pub fn flags(self) -> Flags {
        unsafe { Flags::from_bits_truncate((*self.as_ptr()).flags) }
    }

    pub fn extensions(self) -> Vec<&'static str> {
        unsafe {
            let ptr = (*self.as_ptr()).extensions;

            if ptr.is_null() {
                Vec::new()
            } else {
                utils::str_from_c_ptr(ptr).split(',').collect()
            }
        }
    }

    pub fn mime_types(self) -> Vec<&'static str> {
        unsafe {
            let ptr = (*self.as_ptr()).mime_type;

            if ptr.is_null() {
                Vec::new()
            } else {
                utils::str_from_c_ptr(ptr).split(',').collect()
            }
        }
    }

    pub fn codec<P: AsRef<Path>>(self, path: P, kind: media::Type) -> codec::Id {
        // XXX: use to_cstring when stable
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        unsafe {
            codec::Id::from(av_guess_codec(
                self.as_ptr() as *mut _,
                ptr::null(),
                path.as_ptr(),
                ptr::null(),
                kind.into(),
            ))
        }
    }
}

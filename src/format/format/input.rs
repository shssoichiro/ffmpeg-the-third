use std::ptr::NonNull;

use crate::ffi::*;
use crate::utils;

use super::Flags;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Input {
    ptr: NonNull<AVInputFormat>,
}

impl Input {
    pub unsafe fn from_raw(ptr: *const AVInputFormat) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self { ptr })
    }

    pub fn as_ptr(self) -> *const AVInputFormat {
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
}

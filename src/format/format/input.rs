use crate::ffi::*;
use crate::utils;

pub struct Input {
    ptr: *mut AVInputFormat,
}

impl Input {
    pub unsafe fn wrap(ptr: *mut AVInputFormat) -> Self {
        Input { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVInputFormat {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVInputFormat {
        self.ptr
    }
}

impl Input {
    pub fn name(&self) -> &str {
        unsafe { utils::str_from_c_ptr((*self.as_ptr()).name) }
    }

    pub fn description(&self) -> &str {
        unsafe { utils::optional_str_from_c_ptr((*self.as_ptr()).long_name).unwrap_or("") }
    }

    pub fn extensions(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).extensions;

            if ptr.is_null() {
                Vec::new()
            } else {
                utils::str_from_c_ptr(ptr).split(',').collect()
            }
        }
    }

    pub fn mime_types(&self) -> Vec<&str> {
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

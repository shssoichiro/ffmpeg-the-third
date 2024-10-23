pub mod extensions;
pub mod input;
pub mod output;

use std::marker::PhantomData;

use crate::ffi::*;
use crate::utils;

pub struct Info<'a> {
    ptr: *mut AVDeviceInfo,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Info<'a> {
    pub unsafe fn wrap(ptr: *mut AVDeviceInfo) -> Self {
        Info {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVDeviceInfo {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVDeviceInfo {
        self.ptr
    }
}

impl<'a> Info<'a> {
    pub fn name(&self) -> &str {
        unsafe { utils::str_from_c_ptr((*self.as_ptr()).device_name) }
    }

    pub fn description(&self) -> &str {
        unsafe { utils::str_from_c_ptr((*self.as_ptr()).device_description) }
    }
}

pub fn register_all() {
    unsafe {
        avdevice_register_all();
    }
}

pub fn version() -> u32 {
    unsafe { avdevice_version() }
}

pub fn configuration() -> &'static str {
    unsafe { utils::str_from_c_ptr(avdevice_configuration()) }
}

pub fn license() -> &'static str {
    unsafe { utils::str_from_c_ptr(avdevice_license()) }
}

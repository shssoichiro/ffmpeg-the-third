use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::ffi::*;

pub struct ParametersMut<'p> {
    ptr: NonNull<AVCodecParameters>,
    _marker: PhantomData<&'p mut AVCodecParameters>,
}

impl<'p> ParametersMut<'p> {
    pub unsafe fn from_raw(ptr: *mut AVCodecParameters) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }

    pub fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVCodecParameters {
        self.ptr.as_ptr()
    }
}
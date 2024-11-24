use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::ffi::*;

pub struct ParametersRef<'p> {
    ptr: NonNull<AVCodecParameters>,
    _marker: PhantomData<&'p AVCodecParameters>,
}

impl<'p> ParametersRef<'p> {
    pub unsafe fn from_raw(ptr: *const AVCodecParameters) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }

    pub fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr.as_ptr()
    }
}

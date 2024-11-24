use std::ptr::NonNull;

use crate::codec::Context;
use crate::ffi::*;

pub struct Parameters {
    ptr: NonNull<AVCodecParameters>,
}

unsafe impl Send for Parameters {}

impl Parameters {
    pub unsafe fn from_raw(ptr: *mut AVCodecParameters) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    pub fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVCodecParameters {
        self.ptr.as_ptr()
    }
}

impl Parameters {
    pub fn new() -> Self {
        let ptr = unsafe { avcodec_parameters_alloc() };

        Self {
            ptr: NonNull::new(ptr).expect("can allocate AVCodecParameters"),
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Parameters {
    fn drop(&mut self) {
        unsafe {
            avcodec_parameters_free(&mut self.as_mut_ptr());
        }
    }
}

impl Clone for Parameters {
    fn clone(&self) -> Self {
        let mut ctx = Parameters::new();
        ctx.clone_from(self);

        ctx
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            avcodec_parameters_copy(self.as_mut_ptr(), source.as_ptr());
        }
    }
}

impl<C: AsRef<Context>> From<C> for Parameters {
    fn from(context: C) -> Parameters {
        let mut parameters = Parameters::new();
        let context = context.as_ref();
        unsafe {
            avcodec_parameters_from_context(parameters.as_mut_ptr(), context.as_ptr());
        }
        parameters
    }
}

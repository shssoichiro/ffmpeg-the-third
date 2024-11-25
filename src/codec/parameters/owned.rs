use std::ptr::NonNull;

use crate::codec::Context;
use crate::ffi::*;
use crate::{AsMutPtr, AsPtr};

pub struct Parameters {
    ptr: NonNull<AVCodecParameters>,
}

unsafe impl Send for Parameters {}

impl Parameters {
    /// # Safety
    ///
    /// Ensure that
    /// - it is valid for the returned struct to take ownership of the [`AVCodecParameters`]
    ///   and that
    /// - `ptr` is not used to break Rust's ownership rules after calling this function.
    pub unsafe fn from_raw(ptr: *mut AVCodecParameters) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Exposes a pointer to the contained [`AVCodecParameters`] for FFI purposes.
    ///
    /// This is guaranteed to be a non-null pointer.
    pub fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr.as_ptr()
    }

    /// Exposes a mutable pointer to the contained [`AVCodecParameters`] for FFI purposes.
    ///
    /// This is guaranteed to be a non-null pointer.
    pub fn as_mut_ptr(&mut self) -> *mut AVCodecParameters {
        self.ptr.as_ptr()
    }
}

impl Parameters {
    /// Allocates a new set of codec parameters set to default values.
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

impl AsPtr<AVCodecParameters> for Parameters {
    fn as_ptr(&self) -> *const AVCodecParameters {
        self.as_ptr()
    }
}

impl AsMutPtr<AVCodecParameters> for Parameters {
    fn as_mut_ptr(&mut self) -> *mut AVCodecParameters {
        self.as_mut_ptr()
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

use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::ffi::*;
use crate::{AsMutPtr, AsPtr};

pub struct ParametersMut<'p> {
    ptr: NonNull<AVCodecParameters>,
    _marker: PhantomData<&'p mut AVCodecParameters>,
}

impl<'p> ParametersMut<'p> {
    /// # Safety
    ///
    /// Ensure that
    /// - `ptr` is either null or valid,
    /// - the mutable borrow represented by `ptr` follows Rust borrow rules and
    /// - the lifetime of the returned struct is correctly bounded.
    pub unsafe fn from_raw(ptr: *mut AVCodecParameters) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
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

impl<'p> AsPtr<AVCodecParameters> for ParametersMut<'p> {
    fn as_ptr(&self) -> *const AVCodecParameters {
        self.as_ptr()
    }
}

impl<'p> AsMutPtr<AVCodecParameters> for ParametersMut<'p> {
    fn as_mut_ptr(&mut self) -> *mut AVCodecParameters {
        self.as_mut_ptr()
    }
}

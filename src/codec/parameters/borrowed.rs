use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::ffi::*;
use crate::AsPtr;

pub struct ParametersRef<'p> {
    ptr: NonNull<AVCodecParameters>,
    _marker: PhantomData<&'p AVCodecParameters>,
}

impl<'p> ParametersRef<'p> {
    /// # Safety
    ///
    /// Ensure that
    /// - `ptr` is either null or valid,
    /// - the shared borrow represented by `ptr` follows Rust borrow rules and
    /// - the lifetime of the returned struct is correctly bounded.
    pub unsafe fn from_raw(ptr: *const AVCodecParameters) -> Option<Self> {
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
}

impl<'p> AsPtr<AVCodecParameters> for ParametersRef<'p> {
    fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr.as_ptr()
    }
}

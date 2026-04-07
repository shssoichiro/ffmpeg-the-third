use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::ffi::*;
use crate::utils;
use crate::AsPtr;

#[derive(Debug, PartialEq, Eq)]
pub struct Stream<'a> {
    ptr: NonNull<AVStream>,
    _marker: PhantomData<&'a AVFormatContext>,
}

impl<'a> Stream<'a> {
    /// # Safety
    /// The pointer returned by `ctx` must be non-null and valid.
    pub unsafe fn from_ctx_and_idx<C: AsPtr<AVFormatContext>>(
        ctx: &'a C,
        idx: usize,
    ) -> Option<Self> {
        // SAFETY: Lifetime is correctly bounded (constraint on type parameter `C`).
        unsafe {
            utils::c_slice_or_empty((*ctx.as_ptr()).streams, (*ctx.as_ptr()).nb_streams as usize)
                .get(idx)
                .and_then(|&ptr| Self::from_raw(ptr))
        }
    }

    /// # Safety
    /// `ptr` must be null or valid. Ensure the returned lifetime is correctly bounded.
    pub unsafe fn from_raw(ptr: *const AVStream) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }
}

impl<'a> AsPtr<AVStream> for Stream<'a> {
    fn as_ptr(&self) -> *const AVStream {
        self.ptr.as_ptr()
    }
}

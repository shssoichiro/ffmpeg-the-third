use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::ffi::*;
use crate::utils;
use crate::AsPtr;

#[derive(Debug, PartialEq, Eq)]
pub struct Chapter<'a> {
    ptr: NonNull<AVChapter>,
    _marker: PhantomData<&'a AVFormatContext>,
}

impl<'a> Chapter<'a> {
    /// # Safety
    /// The pointer returned by `ctx` must be non-null and valid.
    pub unsafe fn from_ctx_and_idx<C: AsPtr<AVFormatContext>>(
        ctx: &'a C,
        idx: usize,
    ) -> Option<Self> {
        // SAFETY: Lifetime is correctly bounded (constraint on type parameter `C`).
        unsafe {
            utils::c_slice_or_empty(
                (*ctx.as_ptr()).chapters,
                (*ctx.as_ptr()).nb_chapters as usize,
            )
            .get(idx)
            .and_then(|&ptr| Self::from_raw(ptr))
        }
    }

    /// # Safety
    /// `ptr` must be null or valid. Ensure the returned lifetime is correctly bounded.
    pub unsafe fn from_raw(ptr: *const AVChapter) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }
}

impl<'a> AsPtr<AVChapter> for Chapter<'a> {
    fn as_ptr(&self) -> *const AVChapter {
        self.ptr.as_ptr()
    }
}

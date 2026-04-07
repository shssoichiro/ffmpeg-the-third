use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::ffi::*;
use crate::utils;
use crate::{AsMutPtr, AsPtr};
use crate::{DictionaryMut, Rational};

pub struct ChapterMut<'a> {
    ptr: NonNull<AVChapter>,
    _marker: PhantomData<&'a mut AVFormatContext>,
}

impl<'a> ChapterMut<'a> {
    /// # Safety
    /// The pointer returned by `ctx` must be non-null and valid.
    pub unsafe fn from_ctx_and_idx<C: AsMutPtr<AVFormatContext>>(
        ctx: &'a mut C,
        idx: usize,
    ) -> Option<Self> {
        // SAFETY: Lifetime is correctly bounded (constraint on type parameter `C`).
        unsafe {
            utils::c_slice_or_empty(
                (*ctx.as_mut_ptr()).chapters,
                (*ctx.as_mut_ptr()).nb_chapters as usize,
            )
            .get(idx)
            .and_then(|&ptr| Self::from_raw(ptr))
        }
    }

    /// # Safety
    /// `ptr` must be null or valid. Ensure the returned lifetime is correctly bounded.
    pub unsafe fn from_raw(ptr: *mut AVChapter) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }
}

impl<'a> ChapterMut<'a> {
    pub fn set_id(&mut self, value: i64) {
        unsafe {
            (*self.as_mut_ptr()).id = value as _;
        }
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = value.into().into();
        }
    }

    pub fn set_start(&mut self, value: i64) {
        unsafe {
            (*self.as_mut_ptr()).start = value;
        }
    }

    pub fn set_end(&mut self, value: i64) {
        unsafe {
            (*self.as_mut_ptr()).end = value;
        }
    }

    pub fn metadata_mut(&mut self) -> DictionaryMut<'_> {
        unsafe { DictionaryMut::from_raw(&mut (*self.as_mut_ptr()).metadata) }
    }
}

impl<'a> AsPtr<AVChapter> for ChapterMut<'a> {
    fn as_ptr(&self) -> *const AVChapter {
        self.ptr.as_ptr()
    }
}

impl<'a> AsMutPtr<AVChapter> for ChapterMut<'a> {
    fn as_mut_ptr(&mut self) -> *mut AVChapter {
        self.ptr.as_ptr()
    }
}

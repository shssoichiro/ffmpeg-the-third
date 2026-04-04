use std::marker::PhantomData;
use std::ptr::NonNull;

use super::Subtitle;
use crate::ffi::*;
use crate::AsPtr;

pub struct RectRef<'s> {
    ptr: NonNull<AVSubtitleRect>,
    _marker: PhantomData<&'s Subtitle>,
}

impl<'s> RectRef<'s> {
    /// # Safety
    /// `ptr` must be a valid pointer to an [`AVSubtitleRect`].
    /// Ensure that the returned lifetime is correctly bounded.
    pub unsafe fn from_ptr(ptr: NonNull<AVSubtitleRect>) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }
}

impl<'s> AsPtr<AVSubtitleRect> for RectRef<'s> {
    fn as_ptr(&self) -> *const AVSubtitleRect {
        self.ptr.as_ptr()
    }
}

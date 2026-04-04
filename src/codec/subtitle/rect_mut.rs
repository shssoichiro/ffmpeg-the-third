use libc::c_int;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr::NonNull;

use super::{Flags, RectRef, Subtitle};
use crate::ffi::*;
use crate::{AsMutPtr, AsPtr};

pub struct RectMut<'s> {
    ptr: NonNull<AVSubtitleRect>,
    _marker: PhantomData<&'s mut Subtitle>,
}

impl<'s> RectMut<'s> {
    /// # Safety
    /// `ptr` must be a valid pointer to an [`AVSubtitleRect`].
    /// Ensure that the returned lifetime is correctly bounded.
    pub unsafe fn from_ptr(ptr: NonNull<AVSubtitleRect>) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn as_ref(&self) -> RectRef<'s> {
        unsafe { RectRef::from_ptr(self.ptr) }
    }

    pub fn set_flags(&mut self, flags: Flags) {
        unsafe {
            (*self.as_mut_ptr()).flags = flags.bits();
        }
    }

    pub fn set_x(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).x = value as c_int;
        }
    }

    pub fn set_y(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).y = value as c_int;
        }
    }

    pub fn set_width(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).w = value as c_int;
        }
    }

    pub fn set_height(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).h = value as c_int;
        }
    }

    pub fn set_colors(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).nb_colors = value as c_int;
        }
    }

    pub fn set_text(&mut self, value: &str) {
        let value = CString::new(value).unwrap();

        unsafe {
            (*self.as_mut_ptr()).text = av_strdup(value.as_ptr());
        }
    }

    pub fn set_ass(&mut self, value: &str) {
        let value = CString::new(value).unwrap();

        unsafe {
            (*self.as_mut_ptr()).ass = av_strdup(value.as_ptr());
        }
    }
}

impl<'s> AsPtr<AVSubtitleRect> for RectMut<'s> {
    fn as_ptr(&self) -> *const AVSubtitleRect {
        self.ptr.as_ptr()
    }
}

impl<'s> AsMutPtr<AVSubtitleRect> for RectMut<'s> {
    fn as_mut_ptr(&mut self) -> *mut AVSubtitleRect {
        self.ptr.as_ptr()
    }
}

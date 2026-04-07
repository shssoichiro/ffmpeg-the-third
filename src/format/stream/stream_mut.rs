use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::ffi::*;
use crate::utils;
use crate::{codec, DictionaryMut, Rational};
use crate::{AsMutPtr, AsPtr};

#[derive(Debug, PartialEq, Eq)]
pub struct StreamMut<'a> {
    ptr: NonNull<AVStream>,
    _marker: PhantomData<&'a mut AVFormatContext>,
}

impl<'a> StreamMut<'a> {
    /// # Safety
    /// The pointer returned by `ctx` must be non-null and valid.
    pub unsafe fn from_ctx_and_idx<C: AsMutPtr<AVFormatContext>>(
        ctx: &'a mut C,
        idx: usize,
    ) -> Option<Self> {
        // SAFETY: Lifetime is correctly bounded (constraint on type parameter `C`).
        unsafe {
            utils::c_mut_slice_or_empty(
                (*ctx.as_mut_ptr()).streams,
                (*ctx.as_mut_ptr()).nb_streams as usize,
            )
            .get(idx)
            .and_then(|&ptr| Self::from_raw(ptr))
        }
    }

    /// # Safety
    /// `ptr` must be null or valid. Ensure the returned lifetime is correctly bounded.
    pub unsafe fn from_raw(ptr: *mut AVStream) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = value.into().into();
        }
    }

    pub fn set_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).r_frame_rate = value.into().into();
        }
    }

    pub fn set_avg_frame_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).avg_frame_rate = value.into().into();
        }
    }

    pub fn parameters_mut(&mut self) -> codec::ParametersMut<'_> {
        unsafe {
            codec::ParametersMut::from_raw((*self.as_mut_ptr()).codecpar)
                .expect("codecpar is non-null")
        }
    }

    pub fn set_parameters<P: AsPtr<AVCodecParameters>>(&mut self, parameters: P) {
        unsafe {
            avcodec_parameters_copy((*self.as_mut_ptr()).codecpar, parameters.as_ptr());
        }
    }

    pub fn copy_parameters_from_context(&mut self, ctx: &codec::Context) {
        unsafe {
            avcodec_parameters_from_context((*self.as_mut_ptr()).codecpar, ctx.as_ptr());
        }
    }

    pub fn metadata_mut(&mut self) -> DictionaryMut<'_> {
        unsafe { DictionaryMut::from_raw(&mut (*self.as_mut_ptr()).metadata) }
    }

    pub fn set_sample_aspect_ratio(&mut self, sar: Rational) {
        unsafe {
            (*self.as_mut_ptr()).sample_aspect_ratio = sar.into();
        }
    }
}

impl<'a> AsPtr<AVStream> for StreamMut<'a> {
    fn as_ptr(&self) -> *const AVStream {
        self.ptr.as_ptr()
    }
}

impl<'a> AsMutPtr<AVStream> for StreamMut<'a> {
    fn as_mut_ptr(&mut self) -> *mut AVStream {
        self.ptr.as_ptr()
    }
}

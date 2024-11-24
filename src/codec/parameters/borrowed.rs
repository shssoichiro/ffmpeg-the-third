use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::codec::Id;
use crate::ffi::*;
use crate::media;

pub struct ParametersRef<'p> {
    ptr: NonNull<AVCodecParameters>,
    _marker: PhantomData<&'p AVCodecParameters>,
}

impl<'p> ParametersRef<'p> {
    pub unsafe fn from_raw(ptr: *const AVCodecParameters) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }

    pub fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr.as_ptr()
    }
}

impl<'p> ParametersRef<'p> {
    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).codec_type) }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).codec_id) }
    }
}

use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::codec::Id;
use crate::ffi::*;
use crate::media;

pub struct ParametersMut<'p> {
    ptr: NonNull<AVCodecParameters>,
    _marker: PhantomData<&'p mut AVCodecParameters>,
}

impl<'p> ParametersMut<'p> {
    pub unsafe fn from_raw(ptr: *mut AVCodecParameters) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }

    pub fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVCodecParameters {
        self.ptr.as_ptr()
    }
}

impl<'p> ParametersMut<'p> {
    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).codec_type) }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).codec_id) }
    }
}

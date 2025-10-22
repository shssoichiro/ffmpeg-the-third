use std::any::Any;
use std::ptr;
use std::rc::Rc;

use super::decoder::Decoder;
use super::encoder::Encoder;
use super::{threading, Compliance, Debug, Flags, Id};
use crate::ffi::*;
use crate::media;
use crate::option;
use crate::{AsMutPtr, AsPtr};
use crate::{Codec, Error};
use libc::c_int;

pub struct Context {
    ptr: *mut AVCodecContext,
    owner: Option<Rc<dyn Any>>,
}

unsafe impl Send for Context {}

impl Context {
    pub unsafe fn wrap(ptr: *mut AVCodecContext, owner: Option<Rc<dyn Any>>) -> Self {
        Context { ptr, owner }
    }

    pub unsafe fn as_ptr(&self) -> *const AVCodecContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodecContext {
        self.ptr
    }
}

impl Context {
    pub fn new() -> Self {
        unsafe {
            Context {
                ptr: avcodec_alloc_context3(ptr::null()),
                owner: None,
            }
        }
    }

    pub fn new_with_codec(codec: Codec) -> Self {
        unsafe {
            Context {
                ptr: avcodec_alloc_context3(codec.as_ptr()),
                owner: None,
            }
        }
    }

    pub fn from_parameters<P: AsPtr<AVCodecParameters>>(parameters: P) -> Result<Self, Error> {
        let mut context = Self::new();

        unsafe {
            match avcodec_parameters_to_context(context.as_mut_ptr(), parameters.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(context),
            }
        }
    }

    pub fn decoder(self) -> Decoder {
        Decoder(self)
    }

    pub fn encoder(self) -> Encoder {
        Encoder(self)
    }

    pub fn codec(&self) -> Option<Codec> {
        unsafe { Codec::from_raw((*self.as_ptr()).codec) }
    }

    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).codec_type) }
    }

    pub fn set_flags(&mut self, value: Flags) {
        unsafe {
            (*self.as_mut_ptr()).flags = value.bits() as c_int;
        }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).codec_id) }
    }

    pub fn compliance(&mut self, value: Compliance) {
        unsafe {
            (*self.as_mut_ptr()).strict_std_compliance = value.into();
        }
    }

    pub fn debug(&mut self, value: Debug) {
        unsafe {
            (*self.as_mut_ptr()).debug = value.bits();
        }
    }

    pub fn set_threading(&mut self, config: threading::Config) {
        unsafe {
            (*self.as_mut_ptr()).thread_type = config.kind.into();
            (*self.as_mut_ptr()).thread_count = config.count as c_int;
            #[cfg(not(feature = "ffmpeg_6_0"))]
            {
                (*self.as_mut_ptr()).thread_safe_callbacks = i32::from(config.safe);
            }
        }
    }

    pub fn threading(&self) -> threading::Config {
        unsafe {
            threading::Config {
                kind: threading::Type::from((*self.as_ptr()).active_thread_type),
                count: (*self.as_ptr()).thread_count as usize,
                #[cfg(not(feature = "ffmpeg_6_0"))]
                safe: (*self.as_ptr()).thread_safe_callbacks != 0,
            }
        }
    }

    pub fn set_parameters<P: AsPtr<AVCodecParameters>>(
        &mut self,
        parameters: P,
    ) -> Result<(), Error> {
        unsafe {
            match avcodec_parameters_to_context(self.as_mut_ptr(), parameters.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            if self.owner.is_none() {
                avcodec_free_context(&mut self.ptr);
            }
        }
    }
}

#[cfg(not(feature = "ffmpeg_5_0"))]
impl Clone for Context {
    fn clone(&self) -> Self {
        let mut ctx = Context::new();
        ctx.clone_from(self);

        ctx
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            // Removed in ffmpeg >= 5.0.
            avcodec_copy_context(self.as_mut_ptr(), source.as_ptr());
        }
    }
}

/// `AVCodecContext` in `Context` is the target of `option` operations.
impl AsPtr<AVCodecContext> for Context {
    fn as_ptr(&self) -> *const AVCodecContext {
        self.ptr as *const _
    }
}

impl AsMutPtr<AVCodecContext> for Context {
    fn as_mut_ptr(&mut self) -> *mut AVCodecContext {
        self.ptr as *mut _
    }
}

impl option::Settable<AVCodecContext> for Context {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{decoder, Id};

    // This test exercises opening a decoder and dropping it.
    // It should not segfault when the `Context` and `Opened` wrappers are dropped.
    #[test]
    fn open_and_drop_pgs_decoder_does_not_segfault() {
        // Find the PGS subtitle decoder and open it using a fresh context.
        let pgs = decoder::find(Id::HDMV_PGS_SUBTITLE)
            .expect("PGS decoder must be available in linked FFmpeg");

        let ctx = Context::new();
        let _opened = ctx.decoder().open_as(pgs).expect("can open PGS decoder");

        // Drop occurs at end of scope; success is lack of crash.
    }
}

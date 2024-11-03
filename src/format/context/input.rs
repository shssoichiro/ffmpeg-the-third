use std::ffi::CString;
use std::mem;
use std::ops::{Bound, Deref, DerefMut, RangeBounds};

use super::common::Context;
use super::destructor;
use crate::ffi::*;
#[cfg(not(feature = "ffmpeg_5_0"))]
use crate::Codec;
use crate::{format, Error, Packet, Stream};

pub struct Input {
    ptr: *mut AVFormatContext,
    ctx: Context,
}

unsafe impl Send for Input {}

impl Input {
    pub unsafe fn wrap(ptr: *mut AVFormatContext) -> Self {
        Input {
            ptr,
            ctx: Context::wrap(ptr, destructor::Mode::Input),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }
}

impl Input {
    pub fn format(&self) -> format::Input {
        unsafe { format::Input::from_raw((*self.as_ptr()).iformat).expect("iformat is non-null") }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn video_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).video_codec;
            Codec::from_raw(ptr)
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn audio_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).audio_codec;
            Codec::from_raw(ptr)
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn subtitle_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).subtitle_codec;
            Codec::from_raw(ptr)
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn data_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).data_codec;
            Codec::from_raw(ptr)
        }
    }

    pub fn probe_score(&self) -> i32 {
        unsafe { (*self.as_ptr()).probe_score }
    }

    pub fn packets(&mut self) -> PacketIter {
        PacketIter::new(self)
    }

    pub fn pause(&mut self) -> Result<(), Error> {
        unsafe {
            match av_read_pause(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn play(&mut self) -> Result<(), Error> {
        unsafe {
            match av_read_play(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn seek<R: RangeBounds<i64>>(&mut self, ts: i64, range: R) -> Result<(), Error> {
        unsafe {
            let start = match range.start_bound().cloned() {
                Bound::Included(i) => i,
                Bound::Excluded(i) => i.saturating_add(1),
                Bound::Unbounded => i64::MIN,
            };

            let end = match range.end_bound().cloned() {
                Bound::Included(i) => i,
                Bound::Excluded(i) => i.saturating_sub(1),
                Bound::Unbounded => i64::MAX,
            };

            match avformat_seek_file(self.as_mut_ptr(), -1, start, ts, end, 0) {
                s if s >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

impl Deref for Input {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

pub struct PacketIter<'a> {
    context: &'a mut Input,
}

impl<'a> PacketIter<'a> {
    pub fn new(context: &mut Input) -> PacketIter {
        PacketIter { context }
    }
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = Result<(Stream<'a>, Packet), Error>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut packet = Packet::empty();

        match packet.read(self.context) {
            Ok(..) => unsafe {
                Some(Ok((
                    Stream::wrap(mem::transmute_copy(&self.context), packet.stream()),
                    packet,
                )))
            },

            Err(Error::Eof) => None,

            Err(e) => Some(Err(e)),
        }
    }
}

pub fn dump(ctx: &Input, index: i32, url: Option<&str>) {
    let url = url.map(|u| CString::new(u).unwrap());

    unsafe {
        av_dump_format(
            ctx.as_ptr() as *mut _,
            index,
            url.unwrap_or_else(|| CString::new("").unwrap()).as_ptr(),
            0,
        );
    }
}

use std::ffi::CString;
use std::ops::{Bound, RangeBounds};
use std::ptr::NonNull;

use crate::ffi::*;
use crate::{format, Error, Packet};
use crate::{AsMutPtr, AsPtr};

pub struct Input {
    ptr: NonNull<AVFormatContext>,
}

unsafe impl Send for Input {}

impl Input {
    pub unsafe fn from_raw(ptr: *mut AVFormatContext) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    pub fn format(&self) -> format::Input {
        unsafe { format::Input::from_raw((*self.as_ptr()).iformat).expect("iformat is non-null") }
    }

    pub fn probe_score(&self) -> i32 {
        unsafe { (*self.as_ptr()).probe_score }
    }

    pub fn packets(&mut self) -> PacketIter<'_> {
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

impl AsPtr<AVFormatContext> for Input {
    fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr.as_ptr()
    }
}

impl AsMutPtr<AVFormatContext> for Input {
    fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr.as_ptr()
    }
}

pub struct PacketIter<'a> {
    context: &'a mut Input,
}

impl<'a> PacketIter<'a> {
    pub fn new(context: &mut Input) -> PacketIter<'_> {
        PacketIter { context }
    }
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = Result<Packet, Error>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut packet = Packet::empty();

        match packet.read(self.context) {
            Ok(()) => Some(Ok(packet)),

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

impl Drop for Input {
    fn drop(&mut self) {
        unsafe {
            avformat_close_input(&mut self.as_mut_ptr());
        }
    }
}

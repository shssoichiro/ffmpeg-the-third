use std::ops::Deref;

use super::codec::Codec;
use crate::ffi::*;
use crate::{format, ChannelLayoutMask};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Audio {
    codec: Codec,
}

impl Audio {
    pub unsafe fn new(codec: Codec) -> Audio {
        Audio { codec }
    }
}

impl Audio {
    pub fn rates(&self) -> Option<RateIter> {
        unsafe {
            if (*self.as_ptr()).supported_samplerates.is_null() {
                None
            } else {
                Some(RateIter::new((*self.codec.as_ptr()).supported_samplerates))
            }
        }
    }

    pub fn formats(&self) -> Option<FormatIter> {
        unsafe {
            if (*self.codec.as_ptr()).sample_fmts.is_null() {
                None
            } else {
                Some(FormatIter::new((*self.codec.as_ptr()).sample_fmts))
            }
        }
    }

    pub fn channel_layouts(&self) -> Option<ChannelLayoutMaskIter> {
        unsafe {
            if (*self.codec.as_ptr()).channel_layouts.is_null() {
                None
            } else {
                Some(ChannelLayoutMaskIter::new(
                    (*self.codec.as_ptr()).channel_layouts,
                ))
            }
        }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn ch_layouts(&self) -> Option<ChannelLayoutIter> {
        unsafe { ChannelLayoutIter::from_raw((*self.codec.as_ptr()).ch_layouts) }
    }
}

impl Deref for Audio {
    type Target = Codec;

    fn deref(&self) -> &Self::Target {
        &self.codec
    }
}

pub struct RateIter {
    ptr: *const i32,
}

impl RateIter {
    pub fn new(ptr: *const i32) -> Self {
        RateIter { ptr }
    }
}

impl Iterator for RateIter {
    type Item = i32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == 0 {
                return None;
            }

            let rate = *self.ptr;
            self.ptr = self.ptr.offset(1);

            Some(rate)
        }
    }
}

pub struct FormatIter {
    ptr: *const AVSampleFormat,
}

impl FormatIter {
    pub fn new(ptr: *const AVSampleFormat) -> Self {
        FormatIter { ptr }
    }
}

impl Iterator for FormatIter {
    type Item = format::Sample;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == AVSampleFormat::AV_SAMPLE_FMT_NONE {
                return None;
            }

            let format = (*self.ptr).into();
            self.ptr = self.ptr.offset(1);

            Some(format)
        }
    }
}

pub struct ChannelLayoutMaskIter {
    ptr: *const u64,
}

impl ChannelLayoutMaskIter {
    pub fn new(ptr: *const u64) -> Self {
        ChannelLayoutMaskIter { ptr }
    }

    pub fn best(self, max: i32) -> ChannelLayoutMask {
        self.fold(ChannelLayoutMask::MONO, |acc, cur| {
            if cur.channels() > acc.channels() && cur.channels() <= max {
                cur
            } else {
                acc
            }
        })
    }
}

impl Iterator for ChannelLayoutMaskIter {
    type Item = ChannelLayoutMask;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == 0 {
                return None;
            }

            let layout = ChannelLayoutMask::from_bits_truncate(*self.ptr);
            self.ptr = self.ptr.offset(1);

            Some(layout)
        }
    }
}

#[cfg(feature = "ffmpeg_5_1")]
pub use ch_layout::ChannelLayoutIter;

#[cfg(feature = "ffmpeg_5_1")]
mod ch_layout {
    use super::*;
    use crate::ChannelLayout;

    pub struct ChannelLayoutIter<'a> {
        next: &'a AVChannelLayout,
    }

    impl<'a> ChannelLayoutIter<'a> {
        pub unsafe fn from_raw(ptr: *const AVChannelLayout) -> Option<Self> {
            ptr.as_ref().map(|next| Self { next })
        }
    }

    impl<'a> Iterator for ChannelLayoutIter<'a> {
        type Item = ChannelLayout<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                let curr = self.next;
                if *curr == zeroed_layout() {
                    return None;
                }

                // SAFETY: We trust that there is always an initialized layout up until
                // the zeroed-out AVChannelLayout, which signals the end of iteration.
                self.next = (curr as *const AVChannelLayout).add(1).as_ref().unwrap();
                Some(ChannelLayout::from(curr))
            }
        }
    }

    // TODO: Remove this with a const variable when zeroed() is const (1.75.0)
    unsafe fn zeroed_layout() -> AVChannelLayout {
        std::mem::zeroed()
    }
}

use std::marker::PhantomData;
use std::ptr::NonNull;

use super::{Capabilities, Id, Profile};
use crate::ffi::*;
use crate::{format, media, utils};

pub type Audio = Codec<AudioType>;
pub type Video = Codec<VideoType>;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Codec<Type = UnknownType> {
    ptr: NonNull<AVCodec>,
    _marker: PhantomData<Type>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct UnknownType;
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct AudioType;
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct VideoType;

unsafe impl<T> Send for Codec<T> {}
unsafe impl<T> Sync for Codec<T> {}

impl Codec<UnknownType> {
    /// Create a new reference to a codec from a raw pointer.
    ///
    /// Returns `None` if `ptr` is null.
    pub unsafe fn from_raw(ptr: *const AVCodec) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }
}

impl<T> Codec<T> {
    pub fn as_ptr(&self) -> *const AVCodec {
        self.ptr.as_ptr()
    }

    pub fn is_encoder(&self) -> bool {
        unsafe { av_codec_is_encoder(self.as_ptr()) != 0 }
    }

    pub fn is_decoder(&self) -> bool {
        unsafe { av_codec_is_decoder(self.as_ptr()) != 0 }
    }

    pub fn name(&self) -> &str {
        unsafe { utils::str_from_c_ptr((*self.as_ptr()).name) }
    }

    pub fn description(&self) -> &str {
        unsafe { utils::optional_str_from_c_ptr((*self.as_ptr()).long_name).unwrap_or("") }
    }

    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).type_) }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).id) }
    }

    pub fn is_video(&self) -> bool {
        self.medium() == media::Type::Video
    }

    pub fn video(self) -> Option<Video> {
        if self.medium() == media::Type::Video {
            Some(Codec {
                ptr: self.ptr,
                _marker: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn is_audio(&self) -> bool {
        self.medium() == media::Type::Audio
    }

    pub fn audio(self) -> Option<Audio> {
        if self.medium() == media::Type::Audio {
            Some(Codec {
                ptr: self.ptr,
                _marker: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn max_lowres(&self) -> i32 {
        unsafe { (*self.as_ptr()).max_lowres.into() }
    }

    pub fn capabilities(&self) -> Capabilities {
        unsafe { Capabilities::from_bits_truncate((*self.as_ptr()).capabilities as u32) }
    }

    pub fn profiles(&self) -> Option<ProfileIter> {
        unsafe {
            if (*self.as_ptr()).profiles.is_null() {
                None
            } else {
                Some(ProfileIter::new(self.id(), (*self.as_ptr()).profiles))
            }
        }
    }
}

pub struct ProfileIter {
    id: Id,
    ptr: *const AVProfile,
}

impl ProfileIter {
    pub fn new(id: Id, ptr: *const AVProfile) -> Self {
        ProfileIter { id, ptr }
    }
}

impl Iterator for ProfileIter {
    type Item = Profile;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if (*self.ptr).profile == FF_PROFILE_UNKNOWN {
                return None;
            }

            let profile = Profile::from((self.id, (*self.ptr).profile));
            self.ptr = self.ptr.offset(1);

            Some(profile)
        }
    }
}

impl Codec<AudioType> {
    pub fn rates(&self) -> Option<SampleRateIter> {
        unsafe { SampleRateIter::from_raw((*self.as_ptr()).supported_samplerates) }
    }

    pub fn formats(&self) -> Option<SampleFormatIter> {
        unsafe { SampleFormatIter::from_raw((*self.as_ptr()).sample_fmts) }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channel_layouts(&self) -> Option<ChannelLayoutMaskIter> {
        unsafe {
            if (*self.as_ptr()).channel_layouts.is_null() {
                None
            } else {
                Some(ChannelLayoutMaskIter::new((*self.as_ptr()).channel_layouts))
            }
        }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn ch_layouts(&self) -> Option<ChannelLayoutIter> {
        unsafe { ChannelLayoutIter::from_raw((*self.as_ptr()).ch_layouts) }
    }
}

impl Codec<VideoType> {
    pub fn rates(&self) -> Option<FrameRateIter> {
        unsafe { FrameRateIter::from_raw((*self.as_ptr()).supported_framerates) }
    }

    pub fn formats(&self) -> Option<PixelFormatIter> {
        unsafe { PixelFormatIter::from_raw((*self.as_ptr()).pix_fmts) }
    }
}

pub struct FrameRateIter {
    ptr: NonNull<AVRational>,
}

impl FrameRateIter {
    pub fn from_raw(ptr: *const AVRational) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self { ptr })
    }
}

impl Iterator for FrameRateIter {
    type Item = Rational;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if (*self.ptr.as_ptr()).num == 0 && (*self.ptr.as_ptr()).den == 0 {
                return None;
            }

            let rate = (*self.ptr.as_ptr()).into();
            self.ptr = NonNull::new_unchecked(self.ptr.as_ptr().add(1));

            Some(rate)
        }
    }
}

pub struct PixelFormatIter {
    ptr: NonNull<AVPixelFormat>,
}

impl PixelFormatIter {
    pub fn from_raw(ptr: *const AVPixelFormat) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self { ptr })
    }
}

impl Iterator for PixelFormatIter {
    type Item = format::Pixel;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if (*self.ptr.as_ptr()) == AVPixelFormat::AV_PIX_FMT_NONE {
                return None;
            }

            let format = (*self.ptr.as_ptr()).into();
            self.ptr = NonNull::new_unchecked(self.ptr.as_ptr().add(1));

            Some(format)
        }
    }
}

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;
use crate::Rational;

pub struct SampleRateIter {
    ptr: NonNull<i32>,
}

impl SampleRateIter {
    pub fn from_raw(ptr: *const i32) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self { ptr })
    }
}

impl Iterator for SampleRateIter {
    type Item = i32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if (*self.ptr.as_ptr()) == 0 {
                return None;
            }

            let rate = *self.ptr.as_ptr();
            self.ptr = NonNull::new_unchecked(self.ptr.as_ptr().add(1));

            Some(rate)
        }
    }
}

pub struct SampleFormatIter {
    ptr: NonNull<AVSampleFormat>,
}

impl SampleFormatIter {
    pub fn from_raw(ptr: *const AVSampleFormat) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self { ptr })
    }
}

impl Iterator for SampleFormatIter {
    type Item = format::Sample;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if (*self.ptr.as_ptr()) == AVSampleFormat::AV_SAMPLE_FMT_NONE {
                return None;
            }

            let format = (*self.ptr.as_ptr()).into();
            self.ptr = NonNull::new_unchecked(self.ptr.as_ptr().add(1));

            Some(format)
        }
    }
}

#[cfg(not(feature = "ffmpeg_7_0"))]
pub struct ChannelLayoutMaskIter {
    ptr: *const u64,
}

#[cfg(not(feature = "ffmpeg_7_0"))]
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

#[cfg(not(feature = "ffmpeg_7_0"))]
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

    impl<'a> ChannelLayoutIter<'a> {
        pub fn best(self, max: u32) -> ChannelLayout<'a> {
            self.fold(ChannelLayout::MONO, |acc, cur| {
                if cur.channels() > acc.channels() && cur.channels() <= max {
                    cur
                } else {
                    acc
                }
            })
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

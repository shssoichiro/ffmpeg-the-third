use std::marker::PhantomData;
use std::ptr::NonNull;

use super::config::{
    FrameRateIter, PixelFormatIter, SampleFormatIter, SampleRateIter, TerminatedPtrIter,
};
use super::descriptor::{CodecDescriptor, CodecDescriptorIter};
use super::profile::ProfileIter;
use super::{Capabilities, Id};
use crate::ffi::*;
use crate::{media, utils};

#[cfg(feature = "ffmpeg_7_1")]
use crate::codec::config::{ColorRangeIter, ColorSpaceIter, Supported};

pub fn list_descriptors() -> CodecDescriptorIter {
    CodecDescriptorIter::new()
}

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

    pub fn descriptor(self) -> Option<CodecDescriptor> {
        unsafe {
            let ptr = avcodec_descriptor_get(self.id().into());
            CodecDescriptor::from_raw(ptr)
        }
    }
}

impl Codec<AudioType> {
    /// Checks if the given sample rate is supported by this audio codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_rate(self, rate: libc::c_int) -> bool {
        self.supported_rates().supports(rate)
    }

    /// Returns a [`Supported`] representing the supported sample rates.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_rates(self) -> Supported<SampleRateIter<'static>> {
        use super::config::supported_sample_rates;
        supported_sample_rates(self, None).expect("audio codec returns supported sample rates")
    }

    pub fn rates(&self) -> Option<SampleRateIter> {
        unsafe { SampleRateIter::from_raw((*self.as_ptr()).supported_samplerates) }
    }

    /// Checks if the given sample format is supported by this audio codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_format(self, format: crate::format::Sample) -> bool {
        self.supported_formats().supports(format)
    }

    /// Returns a [`Supported`] representing the supported sample formats.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_formats(self) -> Supported<SampleFormatIter<'static>> {
        use super::config::supported_sample_formats;
        supported_sample_formats(self, None).expect("audio codec returns supported sample formats")
    }

    pub fn formats(&self) -> Option<SampleFormatIter> {
        unsafe { SampleFormatIter::from_raw((*self.as_ptr()).sample_fmts) }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channel_layouts(&self) -> Option<ChannelLayoutMaskIter> {
        unsafe { ChannelLayoutMaskIter::from_raw((*self.as_ptr()).channel_layouts) }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn ch_layouts(&self) -> Option<ChannelLayoutIter> {
        unsafe { ChannelLayoutIter::from_raw((*self.as_ptr()).ch_layouts) }
    }
}

impl Codec<VideoType> {
    /// Checks if the given frame rate is supported by this video codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_rate(self, rate: crate::Rational) -> bool {
        self.supported_rates().supports(rate)
    }

    /// Returns a [`Supported`] representing the supported frame rates.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_rates(self) -> Supported<FrameRateIter<'static>> {
        use crate::codec::config::supported_frame_rates;
        supported_frame_rates(self, None).expect("video codec returns supported frame rates")
    }

    pub fn rates(&self) -> Option<FrameRateIter> {
        unsafe { FrameRateIter::from_raw((*self.as_ptr()).supported_framerates) }
    }

    /// Checks if the given pixel format is supported by this video codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_format(self, format: crate::format::Pixel) -> bool {
        self.supported_formats().supports(format)
    }

    /// Returns a [`Supported`] representing the supported pixel formats.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_formats(self) -> Supported<PixelFormatIter<'static>> {
        use crate::codec::config::supported_pixel_formats;
        supported_pixel_formats(self, None).expect("video codec returns supported pixel formats")
    }

    pub fn formats(&self) -> Option<PixelFormatIter> {
        unsafe { PixelFormatIter::from_raw((*self.as_ptr()).pix_fmts) }
    }

    /// Checks if the given color space is supported by this video codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_color_space(self, space: crate::color::Space) -> bool {
        self.supported_color_spaces().supports(space)
    }

    /// Returns a [`Supported`] representing the supported color spaces.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_color_spaces(self) -> Supported<ColorSpaceIter<'static>> {
        use crate::codec::config::supported_color_spaces;
        supported_color_spaces(self, None).expect("video codec returns supported color spaces")
    }

    /// Checks if the given color range is supported by this video codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_color_range(self, range: crate::color::Range) -> bool {
        self.supported_color_ranges().supports(range)
    }

    /// Returns a [`Supported`] representing the supported color ranges.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_color_ranges(self) -> Supported<ColorRangeIter<'static>> {
        use crate::codec::config::supported_color_ranges;
        supported_color_ranges(self, None).expect("video codec returns supported color ranges")
    }
}

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;

#[cfg(not(feature = "ffmpeg_7_0"))]
pub struct ChannelLayoutMaskIter {
    ptr: NonNull<u64>,
}

#[cfg(not(feature = "ffmpeg_7_0"))]
impl ChannelLayoutMaskIter {
    pub fn from_raw(ptr: *const u64) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self { ptr })
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
            let ptr = self.ptr.as_ptr();
            if *ptr == 0 {
                return None;
            }

            let layout = ChannelLayoutMask::from_bits_truncate(*ptr);
            self.ptr = NonNull::new_unchecked(ptr.add(1));

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

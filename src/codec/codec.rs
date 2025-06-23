use std::marker::PhantomData;
use std::ptr::NonNull;

use super::config::{FrameRateIter, PixelFormatIter, SampleFormatIter, SampleRateIter};
use super::descriptor::{CodecDescriptor, CodecDescriptorIter};
use super::profile::ProfileIter;
use super::{Capabilities, Id};
use crate::ffi::*;
use crate::iters::TerminatedPtrIter;
use crate::AsPtr;
use crate::{media, utils};

#[cfg(feature = "ffmpeg_7_1")]
use crate::codec::config::{ColorRangeIter, ColorSpaceIter, Supported};

pub fn list_descriptors() -> CodecDescriptorIter {
    CodecDescriptorIter::new()
}

pub type Audio<A> = Codec<A, AudioType>;
pub type Video<A> = Codec<A, VideoType>;
pub type Data<A> = Codec<A, DataType>;
pub type Subtitle<A> = Codec<A, SubtitleType>;
pub type Attachment<A> = Codec<A, AttachmentType>;

pub type Decoder<T> = Codec<DecodingAction, T>;
pub type UnknownDecoder = Codec<DecodingAction, UnknownType>;
pub type AudioDecoder = Codec<DecodingAction, AudioType>;
pub type VideoDecoder = Codec<DecodingAction, VideoType>;
pub type DataDecoder = Codec<DecodingAction, DataType>;
pub type SubtitleDecoder = Codec<DecodingAction, SubtitleType>;
pub type AttachmentDecoder = Codec<DecodingAction, AttachmentType>;

pub type Encoder<T> = Codec<EncodingAction, T>;
pub type UnknownEncoder = Codec<EncodingAction, UnknownType>;
pub type AudioEncoder = Codec<EncodingAction, AudioType>;
pub type VideoEncoder = Codec<EncodingAction, VideoType>;
pub type DataEncoder = Codec<EncodingAction, DataType>;
pub type SubtitleEncoder = Codec<EncodingAction, SubtitleType>;
pub type AttachmentEncoder = Codec<EncodingAction, AttachmentType>;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Codec<Action, Type> {
    ptr: NonNull<AVCodec>,
    _marker: PhantomData<(Action, Type)>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct UnknownAction;
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct DecodingAction;
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct EncodingAction;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct UnknownType;
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct VideoType;
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct AudioType;
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct DataType;
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct SubtitleType;
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct AttachmentType;

unsafe impl<A, T> Send for Codec<A, T> {}
unsafe impl<A, T> Sync for Codec<A, T> {}

impl<A, T> Codec<A, T> {
    /// Create a new reference to a codec from a raw pointer.
    ///
    /// Returns `None` if `ptr` is null.
    pub unsafe fn from_raw(ptr: *const AVCodec) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }

    // Helper function to easily convert to another codec type.
    // TODO: Does this need to be unsafe?
    /// Ensure that `self.medium()` is correct for `Codec<U>`.
    fn as_other_codec<U, B>(&self) -> Codec<U, B> {
        Codec {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }

    pub fn is_encoder(self) -> bool {
        unsafe { av_codec_is_encoder(self.as_ptr()) != 0 }
    }

    pub fn is_decoder(self) -> bool {
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

impl<A: Copy, T: Copy> Codec<A, T> {
    pub fn as_encoder(&self) -> Option<Encoder<T>> {
        self.is_encoder().then(|| self.as_other_codec())
    }

    pub fn as_decoder(&self) -> Option<Decoder<T>> {
        self.is_decoder().then(|| self.as_other_codec())
    }
}

impl<A> Codec<A, UnknownType> {
    pub fn is_video(self) -> bool {
        self.medium() == media::Type::Video
    }

    pub fn is_audio(self) -> bool {
        self.medium() == media::Type::Audio
    }

    pub fn is_data(self) -> bool {
        self.medium() == media::Type::Data
    }

    pub fn is_subtitle(self) -> bool {
        self.medium() == media::Type::Subtitle
    }

    pub fn is_attachment(self) -> bool {
        self.medium() == media::Type::Attachment
    }
}

impl<A: Copy> Codec<A, UnknownType> {
    pub fn video(self) -> Option<Codec<A, VideoType>> {
        self.is_video().then(|| self.as_other_codec())
    }

    pub fn audio(self) -> Option<Codec<A, AudioType>> {
        self.is_audio().then(|| self.as_other_codec())
    }

    pub fn data(self) -> Option<Codec<A, DataType>> {
        self.is_data().then(|| self.as_other_codec())
    }

    pub fn subtitle(self) -> Option<Codec<A, SubtitleType>> {
        self.is_subtitle().then(|| self.as_other_codec())
    }

    pub fn attachment(self) -> Option<Codec<A, AttachmentType>> {
        self.is_attachment().then(|| self.as_other_codec())
    }
}

impl<A> Codec<A, AudioType> {
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

    pub fn rates(&self) -> Option<SampleRateIter<'_>> {
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

    pub fn formats(&self) -> Option<SampleFormatIter<'_>> {
        unsafe { SampleFormatIter::from_raw((*self.as_ptr()).sample_fmts) }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channel_layouts(&self) -> Option<ChannelLayoutMaskIter> {
        unsafe { ChannelLayoutMaskIter::from_raw((*self.as_ptr()).channel_layouts) }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn ch_layouts(&self) -> Option<ChannelLayoutIter<'_>> {
        unsafe { ChannelLayoutIter::from_raw((*self.as_ptr()).ch_layouts) }
    }
}

impl<A> Codec<A, VideoType> {
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

    pub fn rates(&self) -> Option<FrameRateIter<'_>> {
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

    pub fn formats(&self) -> Option<PixelFormatIter<'_>> {
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

impl<A, T> AsPtr<AVCodec> for Codec<A, T> {
    fn as_ptr(&self) -> *const AVCodec {
        self.ptr.as_ptr()
    }
}

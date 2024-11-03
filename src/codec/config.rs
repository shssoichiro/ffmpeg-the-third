use std::ptr::NonNull;

#[cfg(feature = "ffmpeg_7_1")]
use crate::codec::Context;
#[cfg(feature = "ffmpeg_7_1")]
use crate::ffi::*;
#[cfg(feature = "ffmpeg_7_1")]
use crate::Codec;
#[cfg(feature = "ffmpeg_7_1")]
use crate::Error;

#[cfg(feature = "ffmpeg_7_1")]
#[derive(Debug, Clone)]
pub enum Supported<I> {
    All,
    Specific(I),
}

#[cfg(feature = "ffmpeg_7_1")]
impl<T, I> Supported<I>
where
    T: PartialEq,
    I: Iterator<Item = T>,
{
    /// Check if all possible configuration values are supported.
    ///
    /// # Example
    ///
    /// ```
    /// use ffmpeg_the_third::codec::{encoder, Id};
    ///
    /// let codec = encoder::find(Id::VP9)
    ///     .expect("Can find a VP9 encoder")
    ///     .video()
    ///     .unwrap();
    ///
    /// let supported = codec.supported_rates();
    /// assert!(supported.all())
    /// ```
    pub fn all(&self) -> bool {
        matches!(self, Supported::All)
    }

    /// Check if a specific configuration value is supported.
    ///
    /// # Example
    ///
    /// ```
    /// use ffmpeg_the_third::codec::{decoder, Id};
    /// use ffmpeg_the_third::format::sample::{Sample, Type};
    ///
    /// let codec = decoder::find(Id::MP3)
    ///     .expect("Can find an MP3 decoder")
    ///     .audio()
    ///     .unwrap();
    ///
    /// let supported = codec.supported_formats();
    /// assert!(supported.supports(Sample::F32(Type::Planar)));
    /// ```
    pub fn supports(self, t: T) -> bool {
        match self {
            Supported::All => true,
            Supported::Specific(mut iter) => iter.any(|elem| elem == t),
        }
    }
}

#[cfg(feature = "ffmpeg_7_1")]
fn supported<WrapperType, AVType, CodecType, I>(
    codec: Codec<CodecType>,
    ctx: Option<&Context>,
    cfg: AVCodecConfig,
) -> Result<Supported<I>, Error>
where
    I: TerminatedPtrIter<AVType, WrapperType>,
    AVType: Into<WrapperType>,
{
    let mut out_ptr: *const libc::c_void = std::ptr::null();

    unsafe {
        let avctx = ctx.map_or(std::ptr::null(), |ctx| ctx.as_ptr());

        let ret = avcodec_get_supported_config(
            avctx,
            codec.as_ptr(),
            cfg,
            0, // flags: unused as of 7.1, set to zero
            &mut out_ptr,
            std::ptr::null_mut(), // out_num_configs: optional, we don't support it currently
        );

        if ret < 0 {
            return Err(Error::from(ret));
        }

        match NonNull::new(out_ptr as *mut _) {
            // non-nullptr -> Specific list of values is supported.
            Some(ptr) => Ok(Supported::Specific(I::from_ptr(ptr))),
            // nullptr -> Everything is supported
            None => Ok(Supported::All),
        }
    }
}

/// Pointer-based iterator, stepped through via pointer arithmetic and ended
/// when a dereferenced pointer equals a terminator value.
pub(crate) trait TerminatedPtrIter<AVType, WrapperType>:
    Sized + Iterator<Item = WrapperType>
{
    /// Create a new iterator from a non-null pointer to any value in the iteration.
    ///
    /// # Safety
    ///
    /// `ptr` and all following pointers must be dereferenceable until the terminator is reached.
    unsafe fn from_ptr(ptr: NonNull<AVType>) -> Self;

    /// Create a new iterator from a pointer to any value in the iteration.
    ///
    /// Returns `None` if `ptr` is null. See also [`from_ptr`][TerminatedPtrIter::from_ptr].
    ///
    /// # Safety
    ///
    /// See [`from_ptr`][TerminatedPtrIter::from_ptr].
    unsafe fn from_raw(ptr: *const AVType) -> Option<Self> {
        unsafe { NonNull::new(ptr as *mut _).map(|ptr| Self::from_ptr(ptr)) }
    }
}

macro_rules! impl_config_iter {
    (
        $fn_name:ident,
        $codec_cfg:expr,
        $iter:ident,
        $ty:ty,
        $av_ty:ty,
        $terminator:expr
    ) => {
        impl_config_iter_fn!($fn_name, $iter, $codec_cfg);
        impl_config_iter_struct!($iter, $av_ty);
        impl_config_iter_traits!($iter, $ty, $av_ty, $terminator);
    };
}

macro_rules! impl_config_iter_struct {
    ($iter:ident, $av_ty:ty) => {
        #[derive(Debug, Clone)]
        pub struct $iter<'a> {
            next: std::ptr::NonNull<$av_ty>,
            _marker: std::marker::PhantomData<&'a $av_ty>,
        }
    };
}

macro_rules! impl_config_iter_fn {
    ($fn_name:ident, $iter:ident, $codec_cfg:expr) => {
        /// Low-level function interacting with the FFmpeg API via
        /// `avcodec_get_supported_config()`. Consider using one of the convenience methods
        /// on the codecs or codec contexts instead.
        #[cfg(feature = "ffmpeg_7_1")]
        pub fn $fn_name<T>(
            codec: Codec<T>,
            ctx: Option<&Context>,
        ) -> Result<Supported<$iter>, Error> {
            supported(codec, ctx, $codec_cfg)
        }
    };
}

macro_rules! impl_config_iter_traits {
    ($iter:ident, $ty:ty, $av_ty:ty, $terminator:expr) => {
        impl<'a> TerminatedPtrIter<$av_ty, $ty> for $iter<'a> {
            unsafe fn from_ptr(ptr: std::ptr::NonNull<$av_ty>) -> Self {
                Self {
                    next: ptr,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        // We make sure that this is true by not incrementing self.ptr after the
        // terminator has been reached.
        impl<'a> std::iter::FusedIterator for $iter<'a> {}

        // TODO: Maybe add ExactSizeIterator? This would require using the out_num_configs
        //       parameter and storing it inside $iter. Not sure it's too important unless
        //       many people want to use .collect() or something else that benefits from
        //       ExactSizeIterator.

        impl<'a> Iterator for $iter<'a> {
            type Item = $ty;

            fn next(&mut self) -> Option<Self::Item> {
                // SAFETY: The FFmpeg API guarantees that the pointer is safe to deref and
                //         increment until the terminator is reached.
                unsafe {
                    let curr = self.next.as_ptr();
                    if *curr == $terminator {
                        return None;
                    }

                    // TODO: Replace with the following if MSRV >= 1.80:
                    // self.next = NonNull::from(self.next).add(1).as_ref();
                    self.next = std::ptr::NonNull::new_unchecked(curr.add(1));

                    Some((*curr).into())
                }
            }
        }
    };
}

impl_config_iter!(
    supported_pixel_formats,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_PIX_FORMAT,
    PixelFormatIter,
    crate::format::Pixel,
    crate::ffi::AVPixelFormat,
    crate::ffi::AVPixelFormat::AV_PIX_FMT_NONE
);

impl_config_iter!(
    supported_frame_rates,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_FRAME_RATE,
    FrameRateIter,
    crate::Rational,
    crate::ffi::AVRational,
    crate::ffi::AVRational { num: 0, den: 0 }
);

impl_config_iter!(
    supported_sample_rates,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_SAMPLE_RATE,
    SampleRateIter,
    libc::c_int,
    libc::c_int,
    0 as libc::c_int
);

impl_config_iter!(
    supported_sample_formats,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_SAMPLE_FORMAT,
    SampleFormatIter,
    crate::format::Sample,
    crate::ffi::AVSampleFormat,
    crate::ffi::AVSampleFormat::AV_SAMPLE_FMT_NONE
);

#[cfg(feature = "ffmpeg_7_1")]
impl_config_iter!(
    supported_color_ranges,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_COLOR_RANGE,
    ColorRangeIter,
    crate::color::Range,
    crate::ffi::AVColorRange,
    crate::ffi::AVColorRange::AVCOL_RANGE_UNSPECIFIED
);

#[cfg(feature = "ffmpeg_7_1")]
impl_config_iter!(
    supported_color_spaces,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_COLOR_SPACE,
    ColorSpaceIter,
    crate::color::Space,
    crate::ffi::AVColorSpace,
    crate::ffi::AVColorSpace::AVCOL_SPC_UNSPECIFIED
);

#[cfg(test)]
#[cfg(feature = "ffmpeg_7_1")]
mod test {
    use super::*;

    use crate::codec::{decoder, encoder, Compliance, Id};
    use crate::color::Range;
    use crate::format::Pixel;
    use crate::Rational;

    // These tests can fail if the FFmpeg build does not contain the required de/encoder.
    // TODO: Check if tests can be hidden behind feature flags.

    #[test]
    fn audio_decoder() {
        let codec = decoder::find(Id::MP3).expect("can find mp3 decoder");

        // Audio decoder does not have color ranges
        assert!(supported_color_ranges(codec, None).is_err());

        let format_iter = match supported_sample_formats(codec, None) {
            Ok(Supported::Specific(f)) => f,
            sup => panic!("Should be Supported::Specific, got {sup:#?}"),
        };

        for format in format_iter {
            println!("format: {format:#?}");
        }
    }

    #[test]
    fn audio_encoder() {
        let codec = encoder::find(Id::OPUS).expect("can find opus encoder");

        // looks like every codec returns Supported::All for color space.
        // might change in a future FFmpeg release
        assert!(matches!(
            supported_color_spaces(codec, None),
            Ok(Supported::All)
        ));
        let format_iter = match supported_sample_formats(codec, None) {
            Ok(Supported::Specific(f)) => f,
            sup => panic!("Should be Supported::Specific, got {sup:#?}"),
        };

        for format in format_iter {
            println!("format: {format:#?}");
        }
    }

    #[test]
    fn video_decoder() {
        let codec = decoder::find(Id::H264).expect("can find H264 decoder");

        assert!(supported_sample_rates(codec, None).is_err());
        assert!(matches!(
            supported_color_spaces(codec, None),
            Ok(Supported::All)
        ));
    }

    #[test]
    fn video_encoder() {
        let codec = encoder::find(Id::VP9).expect("can find VP9 encoder");

        let color_ranges = match supported_color_ranges(codec, None) {
            Ok(Supported::Specific(c)) => c,
            sup => panic!("Should be Supported::Specific, got {sup:#?}"),
        };

        for range in color_ranges {
            println!("{range:#?}");
        }

        assert!(matches!(
            supported_pixel_formats(codec, None),
            Ok(Supported::Specific(_))
        ));

        assert!(matches!(
            supported_frame_rates(codec, None),
            Ok(Supported::All)
        ));
    }

    #[test]
    fn supports() {
        let codec = encoder::find(Id::FFV1).expect("can find FFV1 encoder");

        assert!(supported_color_ranges(codec, None)
            .expect("can check color range support")
            .supports(Range::MPEG));

        assert!(!supported_pixel_formats(codec, None)
            .expect("can check color range support")
            .supports(Pixel::GRAY16));

        assert!(supported_frame_rates(codec, None)
            .expect("can check frame rate support")
            .supports(Rational(123, 456)));

        supported_sample_formats(codec, None)
            .expect_err("can NOT check sample format support (video codec)");
    }

    #[test]
    fn with_context() {
        let codec = encoder::find(Id::MJPEG).expect("can find MJPEG encoder");

        let mut ctx = unsafe {
            let avctx = crate::ffi::avcodec_alloc_context3(codec.as_ptr());
            crate::codec::Context::wrap(avctx, None)
        };

        ctx.compliance(Compliance::Strict);

        assert!(!supported_color_ranges(ctx.codec().unwrap(), Some(&ctx))
            .expect("can check color range support")
            .supports(Range::MPEG));

        ctx.compliance(Compliance::Unofficial);

        // Note that we check for NOT supported above, and YES supported here
        // MJPEG encoder only supports MPEG color range if compliance is
        // Unofficial or lower (less strict)
        assert!(supported_color_ranges(ctx.codec().unwrap(), Some(&ctx))
            .expect("can check color range support")
            .supports(Range::MPEG));
    }
}

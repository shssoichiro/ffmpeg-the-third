use std::error;
use std::ffi::{CString, NulError};
use std::fmt;
use std::str::FromStr;

use crate::ffi::*;
use crate::utils;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Pixel {
    None,

    YUV420P,
    YUYV422,
    RGB24,
    BGR24,
    YUV422P,
    YUV444P,
    YUV410P,
    YUV411P,
    GRAY8,
    MonoWhite,
    MonoBlack,
    PAL8,
    YUVJ420P,
    YUVJ422P,
    YUVJ444P,
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    XVMC_MPEG2_MC,
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    XVMC_MPEG2_IDCT,
    UYVY422,
    UYYVYY411,
    BGR8,
    BGR4,
    BGR4_BYTE,
    RGB8,
    RGB4,
    RGB4_BYTE,
    NV12,
    NV21,

    ARGB,
    RGBA,
    ABGR,
    BGRA,

    GRAY16BE,
    GRAY16LE,
    YUV440P,
    YUVJ440P,
    YUVA420P,
    RGB48BE,
    RGB48LE,

    RGB565BE,
    RGB565LE,
    RGB555BE,
    RGB555LE,

    BGR565BE,
    BGR565LE,
    BGR555BE,
    BGR555LE,

    #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
    VAAPI_MOCO,
    #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
    VAAPI_IDCT,
    #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
    VAAPI_VLD,
    #[cfg(any(not(feature = "ff_api_vaapi"), feature = "ffmpeg_5_0"))]
    VAAPI,

    YUV420P16LE,
    YUV420P16BE,
    YUV422P16LE,
    YUV422P16BE,
    YUV444P16LE,
    YUV444P16BE,
    DXVA2_VLD,

    RGB444LE,
    RGB444BE,
    BGR444LE,
    BGR444BE,
    YA8,

    BGR48BE,
    BGR48LE,

    YUV420P9BE,
    YUV420P9LE,
    YUV420P10BE,
    YUV420P10LE,
    YUV422P10BE,
    YUV422P10LE,
    YUV444P9BE,
    YUV444P9LE,
    YUV444P10BE,
    YUV444P10LE,
    YUV422P9BE,
    YUV422P9LE,

    GBRP,
    GBRP9BE,
    GBRP9LE,
    GBRP10BE,
    GBRP10LE,
    GBRP16BE,
    GBRP16LE,

    YUVA420P9BE,
    YUVA420P9LE,
    YUVA422P9BE,
    YUVA422P9LE,
    YUVA444P9BE,
    YUVA444P9LE,
    YUVA420P10BE,
    YUVA420P10LE,
    YUVA422P10BE,
    YUVA422P10LE,
    YUVA444P10BE,
    YUVA444P10LE,
    YUVA420P16BE,
    YUVA420P16LE,
    YUVA422P16BE,
    YUVA422P16LE,
    YUVA444P16BE,
    YUVA444P16LE,

    VDPAU,

    XYZ12LE,
    XYZ12BE,
    NV16,
    NV20LE,
    NV20BE,

    RGBA64BE,
    RGBA64LE,
    BGRA64BE,
    BGRA64LE,

    YVYU422,

    YA16BE,
    YA16LE,

    QSV,
    MMAL,

    D3D11VA_VLD,

    CUDA,

    ZRGB,
    RGBZ,
    ZBGR,
    BGRZ,
    YUVA444P,
    YUVA422P,

    YUV420P12BE,
    YUV420P12LE,
    YUV420P14BE,
    YUV420P14LE,
    YUV422P12BE,
    YUV422P12LE,
    YUV422P14BE,
    YUV422P14LE,
    YUV444P12BE,
    YUV444P12LE,
    YUV444P14BE,
    YUV444P14LE,
    GBRP12BE,
    GBRP12LE,
    GBRP14BE,
    GBRP14LE,
    GBRAP,
    GBRAP16BE,
    GBRAP16LE,
    YUVJ411P,

    BAYER_BGGR8,
    BAYER_RGGB8,
    BAYER_GBRG8,
    BAYER_GRBG8,
    BAYER_BGGR16LE,
    BAYER_BGGR16BE,
    BAYER_RGGB16LE,
    BAYER_RGGB16BE,
    BAYER_GBRG16LE,
    BAYER_GBRG16BE,
    BAYER_GRBG16LE,
    BAYER_GRBG16BE,

    YUV440P10LE,
    YUV440P10BE,
    YUV440P12LE,
    YUV440P12BE,
    AYUV64LE,
    AYUV64BE,

    VIDEOTOOLBOX,

    // --- defaults
    #[cfg(not(feature = "ffmpeg_7_0"))]
    XVMC,

    RGB32,
    RGB32_1,
    BGR32,
    BGR32_1,
    ZRGB32,
    ZBGR32,

    GRAY16,
    YA16,
    RGB48,
    RGB565,
    RGB555,
    RGB444,
    BGR48,
    BGR565,
    BGR555,
    BGR444,

    YUV420P9,
    YUV422P9,
    YUV444P9,
    YUV420P10,
    YUV422P10,
    YUV440P10,
    YUV444P10,
    YUV420P12,
    YUV422P12,
    YUV440P12,
    YUV444P12,
    YUV420P14,
    YUV422P14,
    YUV444P14,
    YUV420P16,
    YUV422P16,
    YUV444P16,

    GBRP9,
    GBRP10,
    GBRP12,
    GBRP14,
    GBRP16,
    GBRAP16,

    BAYER_BGGR16,
    BAYER_RGGB16,
    BAYER_GBRG16,
    BAYER_GRBG16,

    YUVA420P9,
    YUVA422P9,
    YUVA444P9,
    YUVA420P10,
    YUVA422P10,
    YUVA444P10,
    YUVA420P16,
    YUVA422P16,
    YUVA444P16,

    XYZ12,
    NV20,
    AYUV64,

    P010LE,
    P010BE,
    GBRAP12BE,
    GBRAP12LE,
    GBRAP10LE,
    GBRAP10BE,
    MEDIACODEC,
    GRAY12BE,
    GRAY12LE,
    GRAY10BE,
    GRAY10LE,
    P016LE,
    P016BE,

    D3D11,
    GRAY9BE,
    GRAY9LE,
    GBRPF32BE,
    GBRPF32LE,
    GBRAPF32BE,
    GBRAPF32LE,
    DRM_PRIME,

    OPENCL,

    GRAY14BE,
    GRAY14LE,
    GRAYF32BE,
    GRAYF32LE,

    YUVA422P12BE,
    YUVA422P12LE,
    YUVA444P12BE,
    YUVA444P12LE,
    NV24,
    NV42,

    #[cfg(feature = "ffmpeg_4_3")]
    VULKAN,
    #[cfg(feature = "ffmpeg_4_3")]
    Y210BE,
    #[cfg(feature = "ffmpeg_4_3")]
    Y210LE,

    #[cfg(feature = "ffmpeg_4_4")]
    X2RGB10LE,
    #[cfg(feature = "ffmpeg_4_4")]
    X2RGB10BE,

    #[cfg(feature = "ffmpeg_5_0")]
    X2BGR10LE,
    #[cfg(feature = "ffmpeg_5_0")]
    X2BGR10BE,
    #[cfg(feature = "ffmpeg_5_0")]
    P210BE,
    #[cfg(feature = "ffmpeg_5_0")]
    P210LE,
    #[cfg(feature = "ffmpeg_5_0")]
    P410BE,
    #[cfg(feature = "ffmpeg_5_0")]
    P410LE,
    #[cfg(feature = "ffmpeg_5_0")]
    P216BE,
    #[cfg(feature = "ffmpeg_5_0")]
    P216LE,
    #[cfg(feature = "ffmpeg_5_0")]
    P416BE,
    #[cfg(feature = "ffmpeg_5_0")]
    P416LE,

    #[cfg(feature = "ffmpeg_6_0")]
    VUYA,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF16BE,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF16LE,
    #[cfg(feature = "ffmpeg_6_0")]
    VUYX,
    #[cfg(feature = "ffmpeg_6_0")]
    P012LE,
    #[cfg(feature = "ffmpeg_6_0")]
    P012BE,
    #[cfg(feature = "ffmpeg_6_0")]
    Y212BE,
    #[cfg(feature = "ffmpeg_6_0")]
    Y212LE,
    #[cfg(feature = "ffmpeg_6_0")]
    XV30BE,
    #[cfg(feature = "ffmpeg_6_0")]
    XV30LE,
    #[cfg(feature = "ffmpeg_6_0")]
    XV36BE,
    #[cfg(feature = "ffmpeg_6_0")]
    XV36LE,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBF32BE,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBF32LE,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF32BE,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF32LE,

    #[cfg(feature = "ffmpeg_6_1")]
    P212BE,
    #[cfg(feature = "ffmpeg_6_1")]
    P212LE,
    #[cfg(feature = "ffmpeg_6_1")]
    P412BE,
    #[cfg(feature = "ffmpeg_6_1")]
    P412LE,
    #[cfg(feature = "ffmpeg_6_1")]
    GBRAP14BE,
    #[cfg(feature = "ffmpeg_6_1")]
    GBRAP14LE,

    #[cfg(feature = "ffmpeg_7_0")]
    D3D12,

    #[cfg(feature = "ffmpeg_8_0")]
    AYUV,

    #[cfg(feature = "ffmpeg_8_0")]
    UYVA,

    #[cfg(feature = "ffmpeg_8_0")]
    VYU444,

    #[cfg(feature = "ffmpeg_8_0")]
    V30XBE,
    #[cfg(feature = "ffmpeg_8_0")]
    V30XLE,

    #[cfg(feature = "ffmpeg_8_0")]
    RGBF16BE,
    #[cfg(feature = "ffmpeg_8_0")]
    RGBF16LE,

    #[cfg(feature = "ffmpeg_8_0")]
    RGBA128BE,
    #[cfg(feature = "ffmpeg_8_0")]
    RGBA128LE,

    #[cfg(feature = "ffmpeg_8_0")]
    RGB96BE,
    #[cfg(feature = "ffmpeg_8_0")]
    RGB96LE,

    #[cfg(feature = "ffmpeg_8_0")]
    Y216BE,
    #[cfg(feature = "ffmpeg_8_0")]
    Y216LE,

    #[cfg(feature = "ffmpeg_8_0")]
    XV48BE,
    #[cfg(feature = "ffmpeg_8_0")]
    XV48LE,

    #[cfg(feature = "ffmpeg_8_0")]
    GBRPF16BE,
    #[cfg(feature = "ffmpeg_8_0")]
    GBRPF16LE,
    #[cfg(feature = "ffmpeg_8_0")]
    GBRAPF16BE,
    #[cfg(feature = "ffmpeg_8_0")]
    GBRAPF16LE,

    #[cfg(feature = "ffmpeg_8_0")]
    GRAYF16BE,
    #[cfg(feature = "ffmpeg_8_0")]
    GRAYF16LE,

    #[cfg(feature = "ffmpeg_8_0")]
    AMF_SURFACE,

    #[cfg(feature = "ffmpeg_8_0")]
    GRAY32BE,
    #[cfg(feature = "ffmpeg_8_0")]
    GRAY32LE,

    #[cfg(feature = "ffmpeg_8_0")]
    YAF32BE,
    #[cfg(feature = "ffmpeg_8_0")]
    YAF32LE,

    #[cfg(feature = "ffmpeg_8_0")]
    YAF16BE,
    #[cfg(feature = "ffmpeg_8_0")]
    YAF16LE,

    #[cfg(feature = "ffmpeg_8_0")]
    GBRAP32BE,
    #[cfg(feature = "ffmpeg_8_0")]
    GBRAP32LE,

    #[cfg(feature = "ffmpeg_8_0")]
    YUV444P10MSBBE,
    #[cfg(feature = "ffmpeg_8_0")]
    YUV444P10MSBLE,
    #[cfg(feature = "ffmpeg_8_0")]
    YUV444P12MSBBE,
    #[cfg(feature = "ffmpeg_8_0")]
    YUV444P12MSBLE,
    #[cfg(feature = "ffmpeg_8_0")]
    GBRP10MSBBE,
    #[cfg(feature = "ffmpeg_8_0")]
    GBRP10MSBLE,
    #[cfg(feature = "ffmpeg_8_0")]
    GBRP12MSBBE,
    #[cfg(feature = "ffmpeg_8_0")]
    GBRP12MSBLE,

    #[cfg(feature = "ffmpeg_8_0")]
    OHCODEC,

    #[cfg(feature = "rpi")]
    RPI,
    #[cfg(feature = "rpi")]
    SAND128,
    #[cfg(feature = "rpi")]
    SAND64_10,
    #[cfg(feature = "rpi")]
    SAND64_16,
    #[cfg(feature = "rpi")]
    RPI4_8,
    #[cfg(feature = "rpi")]
    RPI4_10,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Descriptor {
    ptr: *const AVPixFmtDescriptor,
}

unsafe impl Send for Descriptor {}
unsafe impl Sync for Descriptor {}

impl Pixel {
    pub const Y400A: Pixel = Pixel::YA8;
    pub const GRAY8A: Pixel = Pixel::YA8;
    pub const GBR24P: Pixel = Pixel::GBRP;
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    pub const XVMC: Pixel = Pixel::XVMC_MPEG2_IDCT;

    pub fn descriptor(self) -> Option<Descriptor> {
        unsafe {
            let ptr = av_pix_fmt_desc_get(self.into());
            ptr.as_ref().map(|ptr| Descriptor { ptr })
        }
    }
}

impl Descriptor {
    pub fn as_ptr(self) -> *const AVPixFmtDescriptor {
        self.ptr
    }

    pub fn name(self) -> &'static str {
        unsafe { utils::str_from_c_ptr((*self.as_ptr()).name) }
    }

    pub fn nb_components(self) -> u8 {
        unsafe { (*self.as_ptr()).nb_components }
    }

    pub fn log2_chroma_w(self) -> u8 {
        unsafe { (*self.as_ptr()).log2_chroma_w }
    }

    pub fn log2_chroma_h(self) -> u8 {
        unsafe { (*self.as_ptr()).log2_chroma_h }
    }
}

impl From<AVPixelFormat> for Pixel {
    #[inline]
    fn from(value: AVPixelFormat) -> Self {
        use AVPixelFormat as AV;

        match value {
            AV::NONE => Pixel::None,

            AV::YUV420P => Pixel::YUV420P,
            AV::YUYV422 => Pixel::YUYV422,
            AV::RGB24 => Pixel::RGB24,
            AV::BGR24 => Pixel::BGR24,
            AV::YUV422P => Pixel::YUV422P,
            AV::YUV444P => Pixel::YUV444P,
            AV::YUV410P => Pixel::YUV410P,
            AV::YUV411P => Pixel::YUV411P,
            AV::GRAY8 => Pixel::GRAY8,
            AV::MONOWHITE => Pixel::MonoWhite,
            AV::MONOBLACK => Pixel::MonoBlack,
            AV::PAL8 => Pixel::PAL8,
            AV::YUVJ420P => Pixel::YUVJ420P,
            AV::YUVJ422P => Pixel::YUVJ422P,
            AV::YUVJ444P => Pixel::YUVJ444P,
            #[cfg(not(feature = "ffmpeg_7_0"))]
            AV::XVMC => Pixel::XVMC,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            AV::XVMC_MPEG2_MC => Pixel::XVMC_MPEG2_MC,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            AV::XVMC_MPEG2_IDCT => Pixel::XVMC_MPEG2_IDCT,
            AV::UYVY422 => Pixel::UYVY422,
            AV::UYYVYY411 => Pixel::UYYVYY411,
            AV::BGR8 => Pixel::BGR8,
            AV::BGR4 => Pixel::BGR4,
            AV::BGR4_BYTE => Pixel::BGR4_BYTE,
            AV::RGB8 => Pixel::RGB8,
            AV::RGB4 => Pixel::RGB4,
            AV::RGB4_BYTE => Pixel::RGB4_BYTE,
            AV::NV12 => Pixel::NV12,
            AV::NV21 => Pixel::NV21,

            AV::ARGB => Pixel::ARGB,
            AV::RGBA => Pixel::RGBA,
            AV::ABGR => Pixel::ABGR,
            AV::BGRA => Pixel::BGRA,

            AV::GRAY16BE => Pixel::GRAY16BE,
            AV::GRAY16LE => Pixel::GRAY16LE,
            AV::YUV440P => Pixel::YUV440P,
            AV::YUVJ440P => Pixel::YUVJ440P,
            AV::YUVA420P => Pixel::YUVA420P,
            AV::RGB48BE => Pixel::RGB48BE,
            AV::RGB48LE => Pixel::RGB48LE,

            AV::RGB565BE => Pixel::RGB565BE,
            AV::RGB565LE => Pixel::RGB565LE,
            AV::RGB555BE => Pixel::RGB555BE,
            AV::RGB555LE => Pixel::RGB555LE,

            AV::BGR565BE => Pixel::BGR565BE,
            AV::BGR565LE => Pixel::BGR565LE,
            AV::BGR555BE => Pixel::BGR555BE,
            AV::BGR555LE => Pixel::BGR555LE,

            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            AV::VAAPI_MOCO => Pixel::VAAPI_MOCO,
            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            AV::VAAPI_IDCT => Pixel::VAAPI_IDCT,
            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            AV::VAAPI_VLD => Pixel::VAAPI_VLD,
            #[cfg(any(not(feature = "ff_api_vaapi"), feature = "ffmpeg_5_0"))]
            AV::VAAPI => Pixel::VAAPI,

            AV::YUV420P16LE => Pixel::YUV420P16LE,
            AV::YUV420P16BE => Pixel::YUV420P16BE,
            AV::YUV422P16LE => Pixel::YUV422P16LE,
            AV::YUV422P16BE => Pixel::YUV422P16BE,
            AV::YUV444P16LE => Pixel::YUV444P16LE,
            AV::YUV444P16BE => Pixel::YUV444P16BE,
            AV::DXVA2_VLD => Pixel::DXVA2_VLD,

            AV::RGB444LE => Pixel::RGB444LE,
            AV::RGB444BE => Pixel::RGB444BE,
            AV::BGR444LE => Pixel::BGR444LE,
            AV::BGR444BE => Pixel::BGR444BE,
            AV::YA8 => Pixel::YA8,

            AV::BGR48BE => Pixel::BGR48BE,
            AV::BGR48LE => Pixel::BGR48LE,

            AV::YUV420P9BE => Pixel::YUV420P9BE,
            AV::YUV420P9LE => Pixel::YUV420P9LE,
            AV::YUV420P10BE => Pixel::YUV420P10BE,
            AV::YUV420P10LE => Pixel::YUV420P10LE,
            AV::YUV422P10BE => Pixel::YUV422P10BE,
            AV::YUV422P10LE => Pixel::YUV422P10LE,
            AV::YUV444P9BE => Pixel::YUV444P9BE,
            AV::YUV444P9LE => Pixel::YUV444P9LE,
            AV::YUV444P10BE => Pixel::YUV444P10BE,
            AV::YUV444P10LE => Pixel::YUV444P10LE,
            AV::YUV422P9BE => Pixel::YUV422P9BE,
            AV::YUV422P9LE => Pixel::YUV422P9LE,

            AV::GBRP => Pixel::GBRP,
            AV::GBRP9BE => Pixel::GBRP9BE,
            AV::GBRP9LE => Pixel::GBRP9LE,
            AV::GBRP10BE => Pixel::GBRP10BE,
            AV::GBRP10LE => Pixel::GBRP10LE,
            AV::GBRP16BE => Pixel::GBRP16BE,
            AV::GBRP16LE => Pixel::GBRP16LE,

            AV::YUVA420P9BE => Pixel::YUVA420P9BE,
            AV::YUVA420P9LE => Pixel::YUVA420P9LE,
            AV::YUVA422P9BE => Pixel::YUVA422P9BE,
            AV::YUVA422P9LE => Pixel::YUVA422P9LE,
            AV::YUVA444P9BE => Pixel::YUVA444P9BE,
            AV::YUVA444P9LE => Pixel::YUVA444P9LE,
            AV::YUVA420P10BE => Pixel::YUVA420P10BE,
            AV::YUVA420P10LE => Pixel::YUVA420P10LE,
            AV::YUVA422P10BE => Pixel::YUVA422P10BE,
            AV::YUVA422P10LE => Pixel::YUVA422P10LE,
            AV::YUVA444P10BE => Pixel::YUVA444P10BE,
            AV::YUVA444P10LE => Pixel::YUVA444P10LE,
            AV::YUVA420P16BE => Pixel::YUVA420P16BE,
            AV::YUVA420P16LE => Pixel::YUVA420P16LE,
            AV::YUVA422P16BE => Pixel::YUVA422P16BE,
            AV::YUVA422P16LE => Pixel::YUVA422P16LE,
            AV::YUVA444P16BE => Pixel::YUVA444P16BE,
            AV::YUVA444P16LE => Pixel::YUVA444P16LE,

            AV::VDPAU => Pixel::VDPAU,

            AV::XYZ12LE => Pixel::XYZ12LE,
            AV::XYZ12BE => Pixel::XYZ12BE,
            AV::NV16 => Pixel::NV16,
            AV::NV20LE => Pixel::NV20LE,
            AV::NV20BE => Pixel::NV20BE,

            AV::RGBA64BE => Pixel::RGBA64BE,
            AV::RGBA64LE => Pixel::RGBA64LE,
            AV::BGRA64BE => Pixel::BGRA64BE,
            AV::BGRA64LE => Pixel::BGRA64LE,

            AV::YVYU422 => Pixel::YVYU422,

            AV::YA16BE => Pixel::YA16BE,
            AV::YA16LE => Pixel::YA16LE,

            AV::QSV => Pixel::QSV,
            AV::MMAL => Pixel::MMAL,

            AV::D3D11VA_VLD => Pixel::D3D11VA_VLD,

            AV::CUDA => Pixel::CUDA,

            AV::_0RGB => Pixel::ZRGB,
            AV::RGB0 => Pixel::RGBZ,
            AV::_0BGR => Pixel::ZBGR,
            AV::BGR0 => Pixel::BGRZ,
            AV::YUVA444P => Pixel::YUVA444P,
            AV::YUVA422P => Pixel::YUVA422P,

            AV::YUV420P12BE => Pixel::YUV420P12BE,
            AV::YUV420P12LE => Pixel::YUV420P12LE,
            AV::YUV420P14BE => Pixel::YUV420P14BE,
            AV::YUV420P14LE => Pixel::YUV420P14LE,
            AV::YUV422P12BE => Pixel::YUV422P12BE,
            AV::YUV422P12LE => Pixel::YUV422P12LE,
            AV::YUV422P14BE => Pixel::YUV422P14BE,
            AV::YUV422P14LE => Pixel::YUV422P14LE,
            AV::YUV444P12BE => Pixel::YUV444P12BE,
            AV::YUV444P12LE => Pixel::YUV444P12LE,
            AV::YUV444P14BE => Pixel::YUV444P14BE,
            AV::YUV444P14LE => Pixel::YUV444P14LE,
            AV::GBRP12BE => Pixel::GBRP12BE,
            AV::GBRP12LE => Pixel::GBRP12LE,
            AV::GBRP14BE => Pixel::GBRP14BE,
            AV::GBRP14LE => Pixel::GBRP14LE,
            AV::GBRAP => Pixel::GBRAP,
            AV::GBRAP16BE => Pixel::GBRAP16BE,
            AV::GBRAP16LE => Pixel::GBRAP16LE,
            AV::YUVJ411P => Pixel::YUVJ411P,

            AV::BAYER_BGGR8 => Pixel::BAYER_BGGR8,
            AV::BAYER_RGGB8 => Pixel::BAYER_RGGB8,
            AV::BAYER_GBRG8 => Pixel::BAYER_GBRG8,
            AV::BAYER_GRBG8 => Pixel::BAYER_GRBG8,
            AV::BAYER_BGGR16LE => Pixel::BAYER_BGGR16LE,
            AV::BAYER_BGGR16BE => Pixel::BAYER_BGGR16BE,
            AV::BAYER_RGGB16LE => Pixel::BAYER_RGGB16LE,
            AV::BAYER_RGGB16BE => Pixel::BAYER_RGGB16BE,
            AV::BAYER_GBRG16LE => Pixel::BAYER_GBRG16LE,
            AV::BAYER_GBRG16BE => Pixel::BAYER_GBRG16BE,
            AV::BAYER_GRBG16LE => Pixel::BAYER_GRBG16LE,
            AV::BAYER_GRBG16BE => Pixel::BAYER_GRBG16BE,

            AV::YUV440P10LE => Pixel::YUV440P10LE,
            AV::YUV440P10BE => Pixel::YUV440P10BE,
            AV::YUV440P12LE => Pixel::YUV440P12LE,
            AV::YUV440P12BE => Pixel::YUV440P12BE,
            AV::AYUV64LE => Pixel::AYUV64LE,
            AV::AYUV64BE => Pixel::AYUV64BE,

            AV::VIDEOTOOLBOX => Pixel::VIDEOTOOLBOX,

            AV::P010LE => Pixel::P010LE,
            AV::P010BE => Pixel::P010BE,
            AV::GBRAP12BE => Pixel::GBRAP12BE,
            AV::GBRAP12LE => Pixel::GBRAP12LE,
            AV::GBRAP10LE => Pixel::GBRAP10LE,
            AV::GBRAP10BE => Pixel::GBRAP10BE,
            AV::MEDIACODEC => Pixel::MEDIACODEC,
            AV::GRAY12BE => Pixel::GRAY12BE,
            AV::GRAY12LE => Pixel::GRAY12LE,
            AV::GRAY10BE => Pixel::GRAY10BE,
            AV::GRAY10LE => Pixel::GRAY10LE,
            AV::P016LE => Pixel::P016LE,
            AV::P016BE => Pixel::P016BE,

            AV::NB => Pixel::None,

            AV::D3D11 => Pixel::D3D11,
            AV::GRAY9BE => Pixel::GRAY9BE,
            AV::GRAY9LE => Pixel::GRAY9LE,
            AV::GBRPF32BE => Pixel::GBRPF32BE,
            AV::GBRPF32LE => Pixel::GBRPF32LE,
            AV::GBRAPF32BE => Pixel::GBRAPF32BE,
            AV::GBRAPF32LE => Pixel::GBRAPF32LE,
            AV::DRM_PRIME => Pixel::DRM_PRIME,

            AV::OPENCL => Pixel::OPENCL,

            AV::GRAY14BE => Pixel::GRAY14BE,
            AV::GRAY14LE => Pixel::GRAY14LE,
            AV::GRAYF32BE => Pixel::GRAYF32BE,
            AV::GRAYF32LE => Pixel::GRAYF32LE,

            AV::YUVA422P12BE => Pixel::YUVA422P12BE,
            AV::YUVA422P12LE => Pixel::YUVA422P12LE,
            AV::YUVA444P12BE => Pixel::YUVA444P12BE,
            AV::YUVA444P12LE => Pixel::YUVA444P12LE,
            AV::NV24 => Pixel::NV24,
            AV::NV42 => Pixel::NV42,

            #[cfg(feature = "ffmpeg_4_3")]
            AV::VULKAN => Pixel::VULKAN,
            #[cfg(feature = "ffmpeg_4_3")]
            AV::Y210BE => Pixel::Y210BE,
            #[cfg(feature = "ffmpeg_4_3")]
            AV::Y210LE => Pixel::Y210LE,

            #[cfg(feature = "ffmpeg_4_4")]
            AV::X2RGB10LE => Pixel::X2RGB10LE,
            #[cfg(feature = "ffmpeg_4_4")]
            AV::X2RGB10BE => Pixel::X2RGB10BE,

            #[cfg(feature = "ffmpeg_5_0")]
            AV::X2BGR10LE => Pixel::X2BGR10LE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::X2BGR10BE => Pixel::X2BGR10BE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::P210BE => Pixel::P210BE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::P210LE => Pixel::P210LE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::P410BE => Pixel::P410BE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::P410LE => Pixel::P410LE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::P216BE => Pixel::P216BE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::P216LE => Pixel::P216LE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::P416BE => Pixel::P416BE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::P416LE => Pixel::P416LE,

            #[cfg(feature = "ffmpeg_6_0")]
            AV::VUYA => Pixel::VUYA,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::RGBAF16BE => Pixel::RGBAF16BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::RGBAF16LE => Pixel::RGBAF16LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::VUYX => Pixel::VUYX,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::P012LE => Pixel::P012LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::P012BE => Pixel::P012BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::Y212BE => Pixel::Y212BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::Y212LE => Pixel::Y212LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::XV30BE => Pixel::XV30BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::XV30LE => Pixel::XV30LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::XV36BE => Pixel::XV36BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::XV36LE => Pixel::XV36LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::RGBF32BE => Pixel::RGBF32BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::RGBF32LE => Pixel::RGBF32LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::RGBAF32BE => Pixel::RGBAF32BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV::RGBAF32LE => Pixel::RGBAF32LE,

            #[cfg(feature = "ffmpeg_6_1")]
            AV::P212BE => Pixel::P212BE,
            #[cfg(feature = "ffmpeg_6_1")]
            AV::P212LE => Pixel::P212LE,
            #[cfg(feature = "ffmpeg_6_1")]
            AV::P412BE => Pixel::P412BE,
            #[cfg(feature = "ffmpeg_6_1")]
            AV::P412LE => Pixel::P412LE,
            #[cfg(feature = "ffmpeg_6_1")]
            AV::GBRAP14BE => Pixel::GBRAP14BE,
            #[cfg(feature = "ffmpeg_6_1")]
            AV::GBRAP14LE => Pixel::GBRAP14LE,

            #[cfg(feature = "ffmpeg_7_0")]
            AV::D3D12 => Pixel::D3D12,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::AYUV => Pixel::AYUV,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::UYVA => Pixel::UYVA,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::VYU444 => Pixel::VYU444,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::V30XBE => Pixel::V30XBE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::V30XLE => Pixel::V30XLE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::RGBF16BE => Pixel::RGBF16BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::RGBF16LE => Pixel::RGBF16LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::RGBA128BE => Pixel::RGBA128BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::RGBA128LE => Pixel::RGBA128LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::RGB96BE => Pixel::RGB96BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::RGB96LE => Pixel::RGB96LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::Y216BE => Pixel::Y216BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::Y216LE => Pixel::Y216LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::XV48BE => Pixel::XV48BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::XV48LE => Pixel::XV48LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::GBRPF16BE => Pixel::GBRPF16BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::GBRPF16LE => Pixel::GBRPF16LE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::GBRAPF16BE => Pixel::GBRAPF16BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::GBRAPF16LE => Pixel::GBRAPF16LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::GRAYF16BE => Pixel::GRAYF16BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::GRAYF16LE => Pixel::GRAYF16LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::AMF_SURFACE => Pixel::AMF_SURFACE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::GRAY32BE => Pixel::GRAY32BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::GRAY32LE => Pixel::GRAY32LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::YAF32BE => Pixel::YAF32BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::YAF32LE => Pixel::YAF32LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::YAF16BE => Pixel::YAF16BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::YAF16LE => Pixel::YAF16LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::GBRAP32BE => Pixel::GBRAP32BE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::GBRAP32LE => Pixel::GBRAP32LE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::YUV444P10MSBBE => Pixel::YUV444P10MSBBE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::YUV444P10MSBLE => Pixel::YUV444P10MSBLE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::YUV444P12MSBBE => Pixel::YUV444P12MSBBE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::YUV444P12MSBLE => Pixel::YUV444P12MSBLE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::GBRP10MSBBE => Pixel::GBRP10MSBBE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::GBRP10MSBLE => Pixel::GBRP10MSBLE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::GBRP12MSBBE => Pixel::GBRP12MSBBE,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::GBRP12MSBLE => Pixel::GBRP12MSBLE,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::OHCODEC => Pixel::OHCODEC,

            #[cfg(feature = "rpi")]
            AV::RPI => Pixel::RPI,
            #[cfg(feature = "rpi")]
            AV::SAND128 => Pixel::SAND128,
            #[cfg(feature = "rpi")]
            AV::SAND64_10 => Pixel::SAND64_10,
            #[cfg(feature = "rpi")]
            AV::SAND64_16 => Pixel::SAND64_16,
            #[cfg(feature = "rpi")]
            AV::RPI4_8 => Pixel::RPI4_8,
            #[cfg(feature = "rpi")]
            AV::RPI4_10 => Pixel::RPI4_10,

            _ => unimplemented!(),
        }
    }
}

impl From<Pixel> for AVPixelFormat {
    #[inline]
    fn from(value: Pixel) -> AVPixelFormat {
        use AVPixelFormat as AV;

        match value {
            Pixel::None => AV::NONE,

            Pixel::YUV420P => AV::YUV420P,
            Pixel::YUYV422 => AV::YUYV422,
            Pixel::RGB24 => AV::RGB24,
            Pixel::BGR24 => AV::BGR24,
            Pixel::YUV422P => AV::YUV422P,
            Pixel::YUV444P => AV::YUV444P,
            Pixel::YUV410P => AV::YUV410P,
            Pixel::YUV411P => AV::YUV411P,
            Pixel::GRAY8 => AV::GRAY8,
            Pixel::MonoWhite => AV::MONOWHITE,
            Pixel::MonoBlack => AV::MONOBLACK,
            Pixel::PAL8 => AV::PAL8,
            Pixel::YUVJ420P => AV::YUVJ420P,
            Pixel::YUVJ422P => AV::YUVJ422P,
            Pixel::YUVJ444P => AV::YUVJ444P,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            Pixel::XVMC_MPEG2_MC => AV::XVMC_MPEG2_MC,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            Pixel::XVMC_MPEG2_IDCT => AV::XVMC_MPEG2_IDCT,
            Pixel::UYVY422 => AV::UYVY422,
            Pixel::UYYVYY411 => AV::UYYVYY411,
            Pixel::BGR8 => AV::BGR8,
            Pixel::BGR4 => AV::BGR4,
            Pixel::BGR4_BYTE => AV::BGR4_BYTE,
            Pixel::RGB8 => AV::RGB8,
            Pixel::RGB4 => AV::RGB4,
            Pixel::RGB4_BYTE => AV::RGB4_BYTE,
            Pixel::NV12 => AV::NV12,
            Pixel::NV21 => AV::NV21,

            Pixel::ARGB => AV::ARGB,
            Pixel::RGBA => AV::RGBA,
            Pixel::ABGR => AV::ABGR,
            Pixel::BGRA => AV::BGRA,

            Pixel::GRAY16BE => AV::GRAY16BE,
            Pixel::GRAY16LE => AV::GRAY16LE,
            Pixel::YUV440P => AV::YUV440P,
            Pixel::YUVJ440P => AV::YUVJ440P,
            Pixel::YUVA420P => AV::YUVA420P,
            Pixel::RGB48BE => AV::RGB48BE,
            Pixel::RGB48LE => AV::RGB48LE,

            Pixel::RGB565BE => AV::RGB565BE,
            Pixel::RGB565LE => AV::RGB565LE,
            Pixel::RGB555BE => AV::RGB555BE,
            Pixel::RGB555LE => AV::RGB555LE,

            Pixel::BGR565BE => AV::BGR565BE,
            Pixel::BGR565LE => AV::BGR565LE,
            Pixel::BGR555BE => AV::BGR555BE,
            Pixel::BGR555LE => AV::BGR555LE,

            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            Pixel::VAAPI_MOCO => AV::VAAPI_MOCO,
            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            Pixel::VAAPI_IDCT => AV::VAAPI_IDCT,
            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            Pixel::VAAPI_VLD => AV::VAAPI_VLD,
            #[cfg(not(feature = "ff_api_vaapi"))]
            Pixel::VAAPI => AV::VAAPI,

            Pixel::YUV420P16LE => AV::YUV420P16LE,
            Pixel::YUV420P16BE => AV::YUV420P16BE,
            Pixel::YUV422P16LE => AV::YUV422P16LE,
            Pixel::YUV422P16BE => AV::YUV422P16BE,
            Pixel::YUV444P16LE => AV::YUV444P16LE,
            Pixel::YUV444P16BE => AV::YUV444P16BE,
            Pixel::DXVA2_VLD => AV::DXVA2_VLD,

            Pixel::RGB444LE => AV::RGB444LE,
            Pixel::RGB444BE => AV::RGB444BE,
            Pixel::BGR444LE => AV::BGR444LE,
            Pixel::BGR444BE => AV::BGR444BE,
            Pixel::YA8 => AV::YA8,

            Pixel::BGR48BE => AV::BGR48BE,
            Pixel::BGR48LE => AV::BGR48LE,

            Pixel::YUV420P9BE => AV::YUV420P9BE,
            Pixel::YUV420P9LE => AV::YUV420P9LE,
            Pixel::YUV420P10BE => AV::YUV420P10BE,
            Pixel::YUV420P10LE => AV::YUV420P10LE,
            Pixel::YUV422P10BE => AV::YUV422P10BE,
            Pixel::YUV422P10LE => AV::YUV422P10LE,
            Pixel::YUV444P9BE => AV::YUV444P9BE,
            Pixel::YUV444P9LE => AV::YUV444P9LE,
            Pixel::YUV444P10BE => AV::YUV444P10BE,
            Pixel::YUV444P10LE => AV::YUV444P10LE,
            Pixel::YUV422P9BE => AV::YUV422P9BE,
            Pixel::YUV422P9LE => AV::YUV422P9LE,

            Pixel::GBRP => AV::GBRP,
            Pixel::GBRP9BE => AV::GBRP9BE,
            Pixel::GBRP9LE => AV::GBRP9LE,
            Pixel::GBRP10BE => AV::GBRP10BE,
            Pixel::GBRP10LE => AV::GBRP10LE,
            Pixel::GBRP16BE => AV::GBRP16BE,
            Pixel::GBRP16LE => AV::GBRP16LE,

            Pixel::YUVA420P9BE => AV::YUVA420P9BE,
            Pixel::YUVA420P9LE => AV::YUVA420P9LE,
            Pixel::YUVA422P9BE => AV::YUVA422P9BE,
            Pixel::YUVA422P9LE => AV::YUVA422P9LE,
            Pixel::YUVA444P9BE => AV::YUVA444P9BE,
            Pixel::YUVA444P9LE => AV::YUVA444P9LE,
            Pixel::YUVA420P10BE => AV::YUVA420P10BE,
            Pixel::YUVA420P10LE => AV::YUVA420P10LE,
            Pixel::YUVA422P10BE => AV::YUVA422P10BE,
            Pixel::YUVA422P10LE => AV::YUVA422P10LE,
            Pixel::YUVA444P10BE => AV::YUVA444P10BE,
            Pixel::YUVA444P10LE => AV::YUVA444P10LE,
            Pixel::YUVA420P16BE => AV::YUVA420P16BE,
            Pixel::YUVA420P16LE => AV::YUVA420P16LE,
            Pixel::YUVA422P16BE => AV::YUVA422P16BE,
            Pixel::YUVA422P16LE => AV::YUVA422P16LE,
            Pixel::YUVA444P16BE => AV::YUVA444P16BE,
            Pixel::YUVA444P16LE => AV::YUVA444P16LE,

            Pixel::VDPAU => AV::VDPAU,

            Pixel::XYZ12LE => AV::XYZ12LE,
            Pixel::XYZ12BE => AV::XYZ12BE,
            Pixel::NV16 => AV::NV16,
            Pixel::NV20LE => AV::NV20LE,
            Pixel::NV20BE => AV::NV20BE,

            Pixel::RGBA64BE => AV::RGBA64BE,
            Pixel::RGBA64LE => AV::RGBA64LE,
            Pixel::BGRA64BE => AV::BGRA64BE,
            Pixel::BGRA64LE => AV::BGRA64LE,

            Pixel::YVYU422 => AV::YVYU422,

            Pixel::YA16BE => AV::YA16BE,
            Pixel::YA16LE => AV::YA16LE,

            Pixel::QSV => AV::QSV,
            Pixel::MMAL => AV::MMAL,

            Pixel::D3D11VA_VLD => AV::D3D11VA_VLD,

            Pixel::CUDA => AV::CUDA,

            Pixel::ZRGB => AV::_0RGB,
            Pixel::RGBZ => AV::RGB0,
            Pixel::ZBGR => AV::_0BGR,
            Pixel::BGRZ => AV::BGR0,
            Pixel::YUVA444P => AV::YUVA444P,
            Pixel::YUVA422P => AV::YUVA422P,

            Pixel::YUV420P12BE => AV::YUV420P12BE,
            Pixel::YUV420P12LE => AV::YUV420P12LE,
            Pixel::YUV420P14BE => AV::YUV420P14BE,
            Pixel::YUV420P14LE => AV::YUV420P14LE,
            Pixel::YUV422P12BE => AV::YUV422P12BE,
            Pixel::YUV422P12LE => AV::YUV422P12LE,
            Pixel::YUV422P14BE => AV::YUV422P14BE,
            Pixel::YUV422P14LE => AV::YUV422P14LE,
            Pixel::YUV444P12BE => AV::YUV444P12BE,
            Pixel::YUV444P12LE => AV::YUV444P12LE,
            Pixel::YUV444P14BE => AV::YUV444P14BE,
            Pixel::YUV444P14LE => AV::YUV444P14LE,
            Pixel::GBRP12BE => AV::GBRP12BE,
            Pixel::GBRP12LE => AV::GBRP12LE,
            Pixel::GBRP14BE => AV::GBRP14BE,
            Pixel::GBRP14LE => AV::GBRP14LE,
            Pixel::GBRAP => AV::GBRAP,
            Pixel::GBRAP16BE => AV::GBRAP16BE,
            Pixel::GBRAP16LE => AV::GBRAP16LE,
            Pixel::YUVJ411P => AV::YUVJ411P,

            Pixel::BAYER_BGGR8 => AV::BAYER_BGGR8,
            Pixel::BAYER_RGGB8 => AV::BAYER_RGGB8,
            Pixel::BAYER_GBRG8 => AV::BAYER_GBRG8,
            Pixel::BAYER_GRBG8 => AV::BAYER_GRBG8,
            Pixel::BAYER_BGGR16LE => AV::BAYER_BGGR16LE,
            Pixel::BAYER_BGGR16BE => AV::BAYER_BGGR16BE,
            Pixel::BAYER_RGGB16LE => AV::BAYER_RGGB16LE,
            Pixel::BAYER_RGGB16BE => AV::BAYER_RGGB16BE,
            Pixel::BAYER_GBRG16LE => AV::BAYER_GBRG16LE,
            Pixel::BAYER_GBRG16BE => AV::BAYER_GBRG16BE,
            Pixel::BAYER_GRBG16LE => AV::BAYER_GRBG16LE,
            Pixel::BAYER_GRBG16BE => AV::BAYER_GRBG16BE,

            Pixel::YUV440P10LE => AV::YUV440P10LE,
            Pixel::YUV440P10BE => AV::YUV440P10BE,
            Pixel::YUV440P12LE => AV::YUV440P12LE,
            Pixel::YUV440P12BE => AV::YUV440P12BE,
            Pixel::AYUV64LE => AV::AYUV64LE,
            Pixel::AYUV64BE => AV::AYUV64BE,

            Pixel::VIDEOTOOLBOX => AV::VIDEOTOOLBOX,

            // --- defaults
            #[cfg(not(feature = "ffmpeg_7_0"))]
            Pixel::XVMC => AV::XVMC,

            Pixel::RGB32 => AV::RGB32,
            Pixel::RGB32_1 => AV::RGB32_1,
            Pixel::BGR32 => AV::BGR32,
            Pixel::BGR32_1 => AV::BGR32_1,
            Pixel::ZRGB32 => AV::_0RGB32,
            Pixel::ZBGR32 => AV::_0BGR32,

            Pixel::GRAY16 => AV::GRAY16,
            Pixel::YA16 => AV::YA16,
            Pixel::RGB48 => AV::RGB48,
            Pixel::RGB565 => AV::RGB565,
            Pixel::RGB555 => AV::RGB555,
            Pixel::RGB444 => AV::RGB444,
            Pixel::BGR48 => AV::BGR48,
            Pixel::BGR565 => AV::BGR565,
            Pixel::BGR555 => AV::BGR555,
            Pixel::BGR444 => AV::BGR444,

            Pixel::YUV420P9 => AV::YUV420P9,
            Pixel::YUV422P9 => AV::YUV422P9,
            Pixel::YUV444P9 => AV::YUV444P9,
            Pixel::YUV420P10 => AV::YUV420P10,
            Pixel::YUV422P10 => AV::YUV422P10,
            Pixel::YUV440P10 => AV::YUV440P10,
            Pixel::YUV444P10 => AV::YUV444P10,
            Pixel::YUV420P12 => AV::YUV420P12,
            Pixel::YUV422P12 => AV::YUV422P12,
            Pixel::YUV440P12 => AV::YUV440P12,
            Pixel::YUV444P12 => AV::YUV444P12,
            Pixel::YUV420P14 => AV::YUV420P14,
            Pixel::YUV422P14 => AV::YUV422P14,
            Pixel::YUV444P14 => AV::YUV444P14,
            Pixel::YUV420P16 => AV::YUV420P16,
            Pixel::YUV422P16 => AV::YUV422P16,
            Pixel::YUV444P16 => AV::YUV444P16,

            Pixel::GBRP9 => AV::GBRP9,
            Pixel::GBRP10 => AV::GBRP10,
            Pixel::GBRP12 => AV::GBRP12,
            Pixel::GBRP14 => AV::GBRP14,
            Pixel::GBRP16 => AV::GBRP16,
            Pixel::GBRAP16 => AV::GBRAP16,

            Pixel::BAYER_BGGR16 => AV::BAYER_BGGR16,
            Pixel::BAYER_RGGB16 => AV::BAYER_RGGB16,
            Pixel::BAYER_GBRG16 => AV::BAYER_GBRG16,
            Pixel::BAYER_GRBG16 => AV::BAYER_GRBG16,

            Pixel::YUVA420P9 => AV::YUVA420P9,
            Pixel::YUVA422P9 => AV::YUVA422P9,
            Pixel::YUVA444P9 => AV::YUVA444P9,
            Pixel::YUVA420P10 => AV::YUVA420P10,
            Pixel::YUVA422P10 => AV::YUVA422P10,
            Pixel::YUVA444P10 => AV::YUVA444P10,
            Pixel::YUVA420P16 => AV::YUVA420P16,
            Pixel::YUVA422P16 => AV::YUVA422P16,
            Pixel::YUVA444P16 => AV::YUVA444P16,

            Pixel::XYZ12 => AV::XYZ12,
            Pixel::NV20 => AV::NV20,
            Pixel::AYUV64 => AV::AYUV64,

            Pixel::P010LE => AV::P010LE,
            Pixel::P010BE => AV::P010BE,
            Pixel::GBRAP12BE => AV::GBRAP12BE,
            Pixel::GBRAP12LE => AV::GBRAP12LE,
            Pixel::GBRAP10LE => AV::GBRAP10LE,
            Pixel::GBRAP10BE => AV::GBRAP10BE,
            Pixel::MEDIACODEC => AV::MEDIACODEC,
            Pixel::GRAY12BE => AV::GRAY12BE,
            Pixel::GRAY12LE => AV::GRAY12LE,
            Pixel::GRAY10BE => AV::GRAY10BE,
            Pixel::GRAY10LE => AV::GRAY10LE,
            Pixel::P016LE => AV::P016LE,
            Pixel::P016BE => AV::P016BE,

            Pixel::D3D11 => AV::D3D11,
            Pixel::GRAY9BE => AV::GRAY9BE,
            Pixel::GRAY9LE => AV::GRAY9LE,
            Pixel::GBRPF32BE => AV::GBRPF32BE,
            Pixel::GBRPF32LE => AV::GBRPF32LE,
            Pixel::GBRAPF32BE => AV::GBRAPF32BE,
            Pixel::GBRAPF32LE => AV::GBRAPF32LE,
            Pixel::DRM_PRIME => AV::DRM_PRIME,

            Pixel::OPENCL => AV::OPENCL,

            Pixel::GRAY14BE => AV::GRAY14BE,
            Pixel::GRAY14LE => AV::GRAY14LE,
            Pixel::GRAYF32BE => AV::GRAYF32BE,
            Pixel::GRAYF32LE => AV::GRAYF32LE,

            Pixel::YUVA422P12BE => AV::YUVA422P12BE,
            Pixel::YUVA422P12LE => AV::YUVA422P12LE,
            Pixel::YUVA444P12BE => AV::YUVA444P12BE,
            Pixel::YUVA444P12LE => AV::YUVA444P12LE,
            Pixel::NV24 => AV::NV24,
            Pixel::NV42 => AV::NV42,

            #[cfg(feature = "ffmpeg_4_3")]
            Pixel::VULKAN => AV::VULKAN,
            #[cfg(feature = "ffmpeg_4_3")]
            Pixel::Y210BE => AV::Y210BE,
            #[cfg(feature = "ffmpeg_4_3")]
            Pixel::Y210LE => AV::Y210LE,

            #[cfg(feature = "ffmpeg_4_4")]
            Pixel::X2RGB10LE => AV::X2RGB10LE,
            #[cfg(feature = "ffmpeg_4_4")]
            Pixel::X2RGB10BE => AV::X2RGB10BE,

            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::X2BGR10LE => AV::X2BGR10LE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::X2BGR10BE => AV::X2BGR10BE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P210BE => AV::P210BE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P210LE => AV::P210LE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P410BE => AV::P410BE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P410LE => AV::P410LE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P216BE => AV::P216BE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P216LE => AV::P216LE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P416BE => AV::P416BE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P416LE => AV::P416LE,

            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::VUYA => AV::VUYA,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBAF16BE => AV::RGBAF16BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBAF16LE => AV::RGBAF16LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::VUYX => AV::VUYX,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::P012LE => AV::P012LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::P012BE => AV::P012BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::Y212BE => AV::Y212BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::Y212LE => AV::Y212LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::XV30BE => AV::XV30BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::XV30LE => AV::XV30LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::XV36BE => AV::XV36BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::XV36LE => AV::XV36LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBF32BE => AV::RGBF32BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBF32LE => AV::RGBF32LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBAF32BE => AV::RGBAF32BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBAF32LE => AV::RGBAF32LE,

            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::P212BE => AV::P212BE,
            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::P212LE => AV::P212LE,
            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::P412BE => AV::P412BE,
            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::P412LE => AV::P412LE,
            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::GBRAP14BE => AV::GBRAP14BE,
            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::GBRAP14LE => AV::GBRAP14LE,

            #[cfg(feature = "ffmpeg_7_0")]
            Pixel::D3D12 => AV::D3D12,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::AYUV => AV::AYUV,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::UYVA => AV::UYVA,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::VYU444 => AV::VYU444,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::V30XBE => AV::V30XBE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::V30XLE => AV::V30XLE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::RGBF16BE => AV::RGBF16BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::RGBF16LE => AV::RGBF16LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::RGBA128BE => AV::RGBA128BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::RGBA128LE => AV::RGBA128LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::RGB96BE => AV::RGB96BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::RGB96LE => AV::RGB96LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::Y216BE => AV::Y216BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::Y216LE => AV::Y216LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::XV48BE => AV::XV48BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::XV48LE => AV::XV48LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GBRPF16BE => AV::GBRPF16BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GBRPF16LE => AV::GBRPF16LE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GBRAPF16BE => AV::GBRAPF16BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GBRAPF16LE => AV::GBRAPF16LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GRAYF16BE => AV::GRAYF16BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GRAYF16LE => AV::GRAYF16LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::AMF_SURFACE => AV::AMF_SURFACE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GRAY32BE => AV::GRAY32BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GRAY32LE => AV::GRAY32LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::YAF32BE => AV::YAF32BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::YAF32LE => AV::YAF32LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::YAF16BE => AV::YAF16BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::YAF16LE => AV::YAF16LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GBRAP32BE => AV::GBRAP32BE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GBRAP32LE => AV::GBRAP32LE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::YUV444P10MSBBE => AV::YUV444P10MSBBE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::YUV444P10MSBLE => AV::YUV444P10MSBLE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::YUV444P12MSBBE => AV::YUV444P12MSBBE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::YUV444P12MSBLE => AV::YUV444P12MSBLE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GBRP10MSBBE => AV::GBRP10MSBBE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GBRP10MSBLE => AV::GBRP10MSBLE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GBRP12MSBBE => AV::GBRP12MSBBE,
            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::GBRP12MSBLE => AV::GBRP12MSBLE,

            #[cfg(feature = "ffmpeg_8_0")]
            Pixel::OHCODEC => AV::OHCODEC,

            #[cfg(feature = "rpi")]
            Pixel::RPI => AV::RPI,
            #[cfg(feature = "rpi")]
            Pixel::SAND128 => AV::SAND128,
            #[cfg(feature = "rpi")]
            Pixel::SAND64_10 => AV::SAND64_10,
            #[cfg(feature = "rpi")]
            Pixel::SAND64_16 => AV::SAND64_16,
            #[cfg(feature = "rpi")]
            Pixel::RPI4_8 => AV::RPI4_8,
            #[cfg(feature = "rpi")]
            Pixel::RPI4_10 => AV::RPI4_10,
        }
    }
}

#[derive(Debug)]
pub enum ParsePixelError {
    NulError(NulError),
    UnknownFormat,
}

impl fmt::Display for ParsePixelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParsePixelError::NulError(ref e) => e.fmt(f),
            ParsePixelError::UnknownFormat => write!(f, "unknown pixel format"),
        }
    }
}

impl error::Error for ParsePixelError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ParsePixelError::NulError(ref e) => Some(e),
            ParsePixelError::UnknownFormat => None,
        }
    }
}

impl From<NulError> for ParsePixelError {
    fn from(x: NulError) -> ParsePixelError {
        ParsePixelError::NulError(x)
    }
}

impl FromStr for Pixel {
    type Err = ParsePixelError;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Pixel, ParsePixelError> {
        let cstring = CString::new(s)?;
        let format = unsafe { av_get_pix_fmt(cstring.as_ptr()) }.into();

        if format == Pixel::None {
            Err(ParsePixelError::UnknownFormat)
        } else {
            Ok(format)
        }
    }
}

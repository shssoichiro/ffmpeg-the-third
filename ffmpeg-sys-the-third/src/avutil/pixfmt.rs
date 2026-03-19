use crate::{AVPixelFormat, AVPixelFormat as AVP};

#[cfg(target_endian = "little")]
impl AVPixelFormat {
    pub const RGB32: AVPixelFormat = AVP::BGRA;
    pub const RGB32_1: AVPixelFormat = AVP::ABGR;
    pub const BGR32: AVPixelFormat = AVP::RGBA;
    pub const BGR32_1: AVPixelFormat = AVP::ARGB;
    pub const _0RGB32: AVPixelFormat = AVP::BGR0;
    pub const _0BGR32: AVPixelFormat = AVP::RGB0;

    pub const GRAY16: AVPixelFormat = AVP::GRAY16LE;
    pub const YA16: AVPixelFormat = AVP::YA16LE;
    pub const RGB48: AVPixelFormat = AVP::RGB48LE;
    pub const RGB565: AVPixelFormat = AVP::RGB565LE;
    pub const RGB555: AVPixelFormat = AVP::RGB555LE;
    pub const RGB444: AVPixelFormat = AVP::RGB444LE;
    pub const BGR48: AVPixelFormat = AVP::BGR48LE;
    pub const BGR565: AVPixelFormat = AVP::BGR565LE;
    pub const BGR555: AVPixelFormat = AVP::BGR555LE;
    pub const BGR444: AVPixelFormat = AVP::BGR444LE;

    pub const YUV420P9: AVPixelFormat = AVP::YUV420P9LE;
    pub const YUV422P9: AVPixelFormat = AVP::YUV422P9LE;
    pub const YUV444P9: AVPixelFormat = AVP::YUV444P9LE;
    pub const YUV420P10: AVPixelFormat = AVP::YUV420P10LE;
    pub const YUV422P10: AVPixelFormat = AVP::YUV422P10LE;
    pub const YUV440P10: AVPixelFormat = AVP::YUV440P10LE;
    pub const YUV444P10: AVPixelFormat = AVP::YUV444P10LE;
    pub const YUV420P12: AVPixelFormat = AVP::YUV420P12LE;
    pub const YUV422P12: AVPixelFormat = AVP::YUV422P12LE;
    pub const YUV440P12: AVPixelFormat = AVP::YUV440P12LE;
    pub const YUV444P12: AVPixelFormat = AVP::YUV444P12LE;
    pub const YUV420P14: AVPixelFormat = AVP::YUV420P14LE;
    pub const YUV422P14: AVPixelFormat = AVP::YUV422P14LE;
    pub const YUV444P14: AVPixelFormat = AVP::YUV444P14LE;
    pub const YUV420P16: AVPixelFormat = AVP::YUV420P16LE;
    pub const YUV422P16: AVPixelFormat = AVP::YUV422P16LE;
    pub const YUV444P16: AVPixelFormat = AVP::YUV444P16LE;

    pub const GBRP9: AVPixelFormat = AVP::GBRP9LE;
    pub const GBRP10: AVPixelFormat = AVP::GBRP10LE;
    pub const GBRP12: AVPixelFormat = AVP::GBRP12LE;
    pub const GBRP14: AVPixelFormat = AVP::GBRP14LE;
    pub const GBRP16: AVPixelFormat = AVP::GBRP16LE;
    pub const GBRAP16: AVPixelFormat = AVP::GBRAP16LE;

    pub const BAYER_BGGR16: AVPixelFormat = AVP::BAYER_BGGR16LE;
    pub const BAYER_RGGB16: AVPixelFormat = AVP::BAYER_RGGB16LE;
    pub const BAYER_GBRG16: AVPixelFormat = AVP::BAYER_GBRG16LE;
    pub const BAYER_GRBG16: AVPixelFormat = AVP::BAYER_GRBG16LE;

    pub const YUVA420P9: AVPixelFormat = AVP::YUVA420P9LE;
    pub const YUVA422P9: AVPixelFormat = AVP::YUVA422P9LE;
    pub const YUVA444P9: AVPixelFormat = AVP::YUVA444P9LE;
    pub const YUVA420P10: AVPixelFormat = AVP::YUVA420P10LE;
    pub const YUVA422P10: AVPixelFormat = AVP::YUVA422P10LE;
    pub const YUVA444P10: AVPixelFormat = AVP::YUVA444P10LE;
    pub const YUVA420P16: AVPixelFormat = AVP::YUVA420P16LE;
    pub const YUVA422P16: AVPixelFormat = AVP::YUVA422P16LE;
    pub const YUVA444P16: AVPixelFormat = AVP::YUVA444P16LE;

    pub const XYZ12: AVPixelFormat = AVP::XYZ12LE;
    pub const NV20: AVPixelFormat = AVP::NV20LE;
    pub const AYUV64: AVPixelFormat = AVP::AYUV64LE;
}

#[cfg(target_endian = "big")]
impl AVPixelFormat {
    pub const RGB32: AVPixelFormat = AVP::ARGB;
    pub const RGB32_1: AVPixelFormat = AVP::RGBA;
    pub const BGR32: AVPixelFormat = AVP::ABGR;
    pub const BGR32_1: AVPixelFormat = AVP::BGRA;
    pub const _0RGB32: AVPixelFormat = AVP::_0RGB;
    pub const _0BGR32: AVPixelFormat = AVP::_0BGR;

    pub const GRAY16: AVPixelFormat = AVP::GRAY16BE;
    pub const YA16: AVPixelFormat = AVP::YA16BE;
    pub const RGB48: AVPixelFormat = AVP::RGB48BE;
    pub const RGB565: AVPixelFormat = AVP::RGB565BE;
    pub const RGB555: AVPixelFormat = AVP::RGB555BE;
    pub const RGB444: AVPixelFormat = AVP::RGB444BE;
    pub const BGR48: AVPixelFormat = AVP::BGR48BE;
    pub const BGR565: AVPixelFormat = AVP::BGR565BE;
    pub const BGR555: AVPixelFormat = AVP::BGR555BE;
    pub const BGR444: AVPixelFormat = AVP::BGR444BE;

    pub const YUV420P9: AVPixelFormat = AVP::YUV420P9BE;
    pub const YUV422P9: AVPixelFormat = AVP::YUV422P9BE;
    pub const YUV444P9: AVPixelFormat = AVP::YUV444P9BE;
    pub const YUV420P10: AVPixelFormat = AVP::YUV420P10BE;
    pub const YUV422P10: AVPixelFormat = AVP::YUV422P10BE;
    pub const YUV440P10: AVPixelFormat = AVP::YUV440P10BE;
    pub const YUV444P10: AVPixelFormat = AVP::YUV444P10BE;
    pub const YUV420P12: AVPixelFormat = AVP::YUV420P12BE;
    pub const YUV422P12: AVPixelFormat = AVP::YUV422P12BE;
    pub const YUV440P12: AVPixelFormat = AVP::YUV440P12BE;
    pub const YUV444P12: AVPixelFormat = AVP::YUV444P12BE;
    pub const YUV420P14: AVPixelFormat = AVP::YUV420P14BE;
    pub const YUV422P14: AVPixelFormat = AVP::YUV422P14BE;
    pub const YUV444P14: AVPixelFormat = AVP::YUV444P14BE;
    pub const YUV420P16: AVPixelFormat = AVP::YUV420P16BE;
    pub const YUV422P16: AVPixelFormat = AVP::YUV422P16BE;
    pub const YUV444P16: AVPixelFormat = AVP::YUV444P16BE;

    pub const GBRP9: AVPixelFormat = AVP::GBRP9BE;
    pub const GBRP10: AVPixelFormat = AVP::GBRP10BE;
    pub const GBRP12: AVPixelFormat = AVP::GBRP12BE;
    pub const GBRP14: AVPixelFormat = AVP::GBRP14BE;
    pub const GBRP16: AVPixelFormat = AVP::GBRP16BE;
    pub const GBRAP16: AVPixelFormat = AVP::GBRAP16BE;

    pub const BAYER_BGGR16: AVPixelFormat = AVP::BAYER_BGGR16BE;
    pub const BAYER_RGGB16: AVPixelFormat = AVP::BAYER_RGGB16BE;
    pub const BAYER_GBRG16: AVPixelFormat = AVP::BAYER_GBRG16BE;
    pub const BAYER_GRBG16: AVPixelFormat = AVP::BAYER_GRBG16BE;

    pub const YUVA420P9: AVPixelFormat = AVP::YUVA420P9BE;
    pub const YUVA422P9: AVPixelFormat = AVP::YUVA422P9BE;
    pub const YUVA444P9: AVPixelFormat = AVP::YUVA444P9BE;
    pub const YUVA420P10: AVPixelFormat = AVP::YUVA420P10BE;
    pub const YUVA422P10: AVPixelFormat = AVP::YUVA422P10BE;
    pub const YUVA444P10: AVPixelFormat = AVP::YUVA444P10BE;
    pub const YUVA420P16: AVPixelFormat = AVP::YUVA420P16BE;
    pub const YUVA422P16: AVPixelFormat = AVP::YUVA422P16BE;
    pub const YUVA444P16: AVPixelFormat = AVP::YUVA444P16BE;

    pub const XYZ12: AVPixelFormat = AVP::XYZ12BE;
    pub const NV20: AVPixelFormat = AVP::NV20BE;
    pub const AYUV64: AVPixelFormat = AVP::AYUV64BE;
}

use crate::ffi::*;
use crate::utils;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Space {
    RGB,
    BT709,
    Unspecified,
    Reserved,
    FCC,
    BT470BG,
    SMPTE170M,
    SMPTE240M,
    YCGCO,
    BT2020NCL,
    BT2020CL,
    SMPTE2085,

    ChromaDerivedNCL,
    ChromaDerivedCL,
    ICTCP,

    #[cfg(feature = "ffmpeg_7_1")]
    IPTC2,
    #[cfg(feature = "ffmpeg_7_1")]
    YCGCORE,
    #[cfg(feature = "ffmpeg_7_1")]
    YCGCORO,
}

impl Space {
    pub const YCOCG: Space = Space::YCGCO;

    pub fn name(&self) -> Option<&'static str> {
        if *self == Space::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_space_name((*self).into());
            utils::optional_str_from_c_ptr(ptr)
        }
    }
}

impl From<AVColorSpace> for Space {
    fn from(value: AVColorSpace) -> Self {
        use AVColorSpace as AV;

        match value {
            AV::RGB => Space::RGB,
            AV::BT709 => Space::BT709,
            AV::UNSPECIFIED => Space::Unspecified,
            AV::RESERVED => Space::Reserved,
            AV::FCC => Space::FCC,
            AV::BT470BG => Space::BT470BG,
            AV::SMPTE170M => Space::SMPTE170M,
            AV::SMPTE240M => Space::SMPTE240M,
            AV::YCGCO => Space::YCGCO,
            AV::BT2020_NCL => Space::BT2020NCL,
            AV::BT2020_CL => Space::BT2020CL,
            AV::SMPTE2085 => Space::SMPTE2085,
            AV::NB => Space::Unspecified,

            AV::CHROMA_DERIVED_NCL => Space::ChromaDerivedNCL,
            AV::CHROMA_DERIVED_CL => Space::ChromaDerivedCL,
            AV::ICTCP => Space::ICTCP,

            #[cfg(feature = "ffmpeg_7_1")]
            AV::IPT_C2 => Space::IPTC2,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::YCGCO_RE => Space::YCGCORE,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::YCGCO_RO => Space::YCGCORO,

            _ => unimplemented!(),
        }
    }
}

impl From<Space> for AVColorSpace {
    fn from(value: Space) -> AVColorSpace {
        use AVColorSpace as AV;

        match value {
            Space::RGB => AV::RGB,
            Space::BT709 => AV::BT709,
            Space::Unspecified => AV::UNSPECIFIED,
            Space::Reserved => AV::RESERVED,
            Space::FCC => AV::FCC,
            Space::BT470BG => AV::BT470BG,
            Space::SMPTE170M => AV::SMPTE170M,
            Space::SMPTE240M => AV::SMPTE240M,
            Space::YCGCO => AV::YCGCO,
            Space::BT2020NCL => AV::BT2020_NCL,
            Space::BT2020CL => AV::BT2020_CL,
            Space::SMPTE2085 => AV::SMPTE2085,

            Space::ChromaDerivedNCL => AV::CHROMA_DERIVED_NCL,
            Space::ChromaDerivedCL => AV::CHROMA_DERIVED_CL,
            Space::ICTCP => AV::ICTCP,

            #[cfg(feature = "ffmpeg_7_1")]
            Space::IPTC2 => AV::IPT_C2,
            #[cfg(feature = "ffmpeg_7_1")]
            Space::YCGCORE => AV::YCGCO_RE,
            #[cfg(feature = "ffmpeg_7_1")]
            Space::YCGCORO => AV::YCGCO_RO,
        }
    }
}

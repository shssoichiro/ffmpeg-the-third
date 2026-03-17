use crate::ffi::*;
use crate::utils;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Primaries {
    Reserved0,
    BT709,
    Unspecified,
    Reserved,
    BT470M,

    BT470BG,
    SMPTE170M,
    SMPTE240M,
    Film,
    BT2020,

    SMPTE428,
    SMPTE431,
    SMPTE432,
    EBU3213,

    #[cfg(feature = "ffmpeg_8_1")]
    V_GAMUT,
}

impl Primaries {
    pub const JEDEC_P22: Primaries = Primaries::EBU3213;

    pub fn name(&self) -> Option<&'static str> {
        if *self == Primaries::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_primaries_name((*self).into());
            utils::optional_str_from_c_ptr(ptr)
        }
    }
}

impl From<AVColorPrimaries> for Primaries {
    fn from(value: AVColorPrimaries) -> Primaries {
        use AVColorPrimaries as AV;

        match value {
            AV::AVCOL_PRI_RESERVED0 => Primaries::Reserved0,
            AV::AVCOL_PRI_BT709 => Primaries::BT709,
            AV::AVCOL_PRI_UNSPECIFIED => Primaries::Unspecified,
            AV::AVCOL_PRI_RESERVED => Primaries::Reserved,
            AV::AVCOL_PRI_BT470M => Primaries::BT470M,

            AV::AVCOL_PRI_BT470BG => Primaries::BT470BG,
            AV::AVCOL_PRI_SMPTE170M => Primaries::SMPTE170M,
            AV::AVCOL_PRI_SMPTE240M => Primaries::SMPTE240M,
            AV::AVCOL_PRI_FILM => Primaries::Film,
            AV::AVCOL_PRI_BT2020 => Primaries::BT2020,
            AV::AVCOL_PRI_NB => unreachable!(),

            AV::AVCOL_PRI_SMPTE428 => Primaries::SMPTE428,
            AV::AVCOL_PRI_SMPTE431 => Primaries::SMPTE431,
            AV::AVCOL_PRI_SMPTE432 => Primaries::SMPTE432,
            AV::AVCOL_PRI_EBU3213 => Primaries::EBU3213,

            #[cfg(feature = "ffmpeg_8_1")]
            // upstream defines it this way
            AV::AVCOL_PRI_EXT_BASE => Primaries::V_GAMUT,
            #[cfg(feature = "ffmpeg_8_1")]
            AV::AVCOL_PRI_EXT_NB => unreachable!(),

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Primaries> for AVColorPrimaries {
    fn from(value: Primaries) -> AVColorPrimaries {
        use AVColorPrimaries as AV;

        match value {
            Primaries::Reserved0 => AV::AVCOL_PRI_RESERVED0,
            Primaries::BT709 => AV::AVCOL_PRI_BT709,
            Primaries::Unspecified => AV::AVCOL_PRI_UNSPECIFIED,
            Primaries::Reserved => AV::AVCOL_PRI_RESERVED,
            Primaries::BT470M => AV::AVCOL_PRI_BT470M,

            Primaries::BT470BG => AV::AVCOL_PRI_BT470BG,
            Primaries::SMPTE170M => AV::AVCOL_PRI_SMPTE170M,
            Primaries::SMPTE240M => AV::AVCOL_PRI_SMPTE240M,
            Primaries::Film => AV::AVCOL_PRI_FILM,
            Primaries::BT2020 => AV::AVCOL_PRI_BT2020,

            Primaries::SMPTE428 => AV::AVCOL_PRI_SMPTE428,
            Primaries::SMPTE431 => AV::AVCOL_PRI_SMPTE431,
            Primaries::SMPTE432 => AV::AVCOL_PRI_SMPTE432,
            Primaries::EBU3213 => AV::AVCOL_PRI_EBU3213,

            #[cfg(feature = "ffmpeg_8_1")]
            Primaries::V_GAMUT => AV::AVCOL_PRI_V_GAMUT,
        }
    }
}

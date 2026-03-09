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
    #[cfg(not(feature = "ffmpeg_4_3"))]
    JEDEC_P22,
    #[cfg(feature = "ffmpeg_4_3")]
    EBU3213,
}

impl Primaries {
    #[cfg(feature = "ffmpeg_4_3")]
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
            AV::RESERVED0 => Primaries::Reserved0,
            AV::BT709 => Primaries::BT709,
            AV::UNSPECIFIED => Primaries::Unspecified,
            AV::RESERVED => Primaries::Reserved,
            AV::BT470M => Primaries::BT470M,

            AV::BT470BG => Primaries::BT470BG,
            AV::SMPTE170M => Primaries::SMPTE170M,
            AV::SMPTE240M => Primaries::SMPTE240M,
            AV::FILM => Primaries::Film,
            AV::BT2020 => Primaries::BT2020,

            AV::SMPTE428 => Primaries::SMPTE428,
            AV::SMPTE431 => Primaries::SMPTE431,
            AV::SMPTE432 => Primaries::SMPTE432,

            #[cfg(not(feature = "ffmpeg_4_3"))]
            AV::JEDEC_P22 => Primaries::JEDEC_P22,
            #[cfg(feature = "ffmpeg_4_3")]
            AV::EBU3213 => Primaries::EBU3213,

            AV::NB => unreachable!(),

            _ => unimplemented!(),
        }
    }
}

impl From<Primaries> for AVColorPrimaries {
    fn from(value: Primaries) -> AVColorPrimaries {
        use AVColorPrimaries as AV;

        match value {
            Primaries::Reserved0 => AV::RESERVED0,
            Primaries::BT709 => AV::BT709,
            Primaries::Unspecified => AV::UNSPECIFIED,
            Primaries::Reserved => AV::RESERVED,
            Primaries::BT470M => AV::BT470M,

            Primaries::BT470BG => AV::BT470BG,
            Primaries::SMPTE170M => AV::SMPTE170M,
            Primaries::SMPTE240M => AV::SMPTE240M,
            Primaries::Film => AV::FILM,
            Primaries::BT2020 => AV::BT2020,

            Primaries::SMPTE428 => AV::SMPTE428,
            Primaries::SMPTE431 => AV::SMPTE431,
            Primaries::SMPTE432 => AV::SMPTE432,
            #[cfg(not(feature = "ffmpeg_4_3"))]
            Primaries::JEDEC_P22 => AV::JEDEC_P22,
            #[cfg(feature = "ffmpeg_4_3")]
            Primaries::EBU3213 => AV::EBU3213,
        }
    }
}

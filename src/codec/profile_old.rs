use super::Id;
use crate::ffi::*;
use libc::c_int;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Profile {
    Unknown,
    Reserved,

    AAC(AAC),
    DNXHD(DNXHD),
    DTS(DTS),
    MPEG2(MPEG2),
    H264(H264),
    VC1(VC1),
    MPEG4(MPEG4),
    JPEG2000(JPEG2000),
    HEVC(HEVC),
    VP9(VP9),
    #[cfg(feature = "ffmpeg_4_4")]
    VVC(VVC),
    AV1(AV1),
    MJPEG(MJPEG),
    SBC_MSBC,
    ProRes(ProRes),
    ARIB(ARIB),
    #[cfg(feature = "ffmpeg_4_3")]
    KLVA(KLVA),
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum AAC {
    Main,
    Low,
    SSR,
    LTP,
    HE,
    HEv2,
    LD,
    ELD,

    MPEG2Low,
    MPEG2HE,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum DNXHD {
    HD,
    HR_LB,
    HR_SQ,
    HR_HQ,
    HR_HQX,
    HR_444,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum DTS {
    Default,
    ES,
    _96_24,
    HD_HRA,
    HD_MA,
    Express,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum MPEG2 {
    _422,
    High,
    SS,
    SNRScalable,
    Main,
    Simple,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum H264 {
    Constrained,
    Intra,
    Baseline,
    ConstrainedBaseline,
    Main,
    Extended,
    High,
    High10,
    High10Intra,
    High422,
    High422Intra,
    High444,
    High444Predictive,
    High444Intra,
    CAVLC444,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum VC1 {
    Simple,
    Main,
    Complex,
    Advanced,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum MPEG4 {
    Simple,
    SimpleScalable,
    Core,
    Main,
    NBit,
    ScalableTexture,
    SimpleFaceAnimation,
    BasicAnimatedTexture,
    Hybrid,
    AdvancedRealTime,
    CoreScalable,
    AdvancedCoding,
    AdvancedCore,
    AdvancedScalableTexture,
    SimpleStudio,
    AdvancedSimple,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum JPEG2000 {
    CStreamRestriction0,
    CStreamRestriction1,
    CStreamNoRestriction,
    DCinema2K,
    DCinema4K,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum VP9 {
    _0,
    _1,
    _2,
    _3,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum HEVC {
    Main,
    Main10,
    MainStillPicture,
    Rext,
}

#[cfg(feature = "ffmpeg_4_4")]
#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum VVC {
    Main10,
    Main10_444,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum AV1 {
    Main,
    High,
    Professional,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum MJPEG {
    HuffmanBaselineDCT,
    HuffmanExtendedSequentialDCT,
    HuffmanProgressiveDCT,
    HuffmanLossless,
    JPEG_LS,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum ProRes {
    Proxy,
    LT,
    Standard,
    HQ,
    _4444,
    XQ,
}

#[cfg(feature = "ffmpeg_8_0")]
#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum ProResRAW {
    Default,
    HQ,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum ARIB {
    ProfileA,
    ProfileC,
}

#[cfg(feature = "ffmpeg_4_3")]
#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum KLVA {
    Sync,
    Async,
}

#[cfg(feature = "ffmpeg_6_1")]
#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum EVC {
    Baseline,
    Main,
}

#[cfg(feature = "ffmpeg_8_0")]
#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum APV {
    _422_10,
    _422_12,
    _444_10,
    _444_12,
    _4444_10,
    _4444_12,
    _400_10,
}

impl From<(Id, c_int)> for Profile {
    fn from((id, value): (Id, c_int)) -> Profile {
        if value == FF_PROFILE_UNKNOWN {
            return Profile::Unknown;
        }

        if value == FF_PROFILE_RESERVED {
            return Profile::Reserved;
        }

        match id {
            Id::AAC => match value {
                FF_PROFILE_AAC_MAIN => Profile::AAC(AAC::Main),
                FF_PROFILE_AAC_LOW => Profile::AAC(AAC::Low),
                FF_PROFILE_AAC_SSR => Profile::AAC(AAC::SSR),
                FF_PROFILE_AAC_LTP => Profile::AAC(AAC::LTP),
                FF_PROFILE_AAC_HE => Profile::AAC(AAC::HE),
                FF_PROFILE_AAC_HE_V2 => Profile::AAC(AAC::HEv2),
                FF_PROFILE_AAC_LD => Profile::AAC(AAC::LD),
                FF_PROFILE_AAC_ELD => Profile::AAC(AAC::ELD),

                FF_PROFILE_MPEG2_AAC_LOW => Profile::AAC(AAC::MPEG2Low),
                FF_PROFILE_MPEG2_AAC_HE => Profile::AAC(AAC::MPEG2HE),

                _ => Profile::Unknown,
            },

            Id::DNXHD => Profile::DNXHD(match value {
                FF_PROFILE_DNXHD => DNXHD::HD,
                FF_PROFILE_DNXHR_LB => DNXHD::HR_LB,
                FF_PROFILE_DNXHR_SQ => DNXHD::HR_SQ,
                FF_PROFILE_DNXHR_HQ => DNXHD::HR_HQ,
                FF_PROFILE_DNXHR_HQX => DNXHD::HR_HQX,
                FF_PROFILE_DNXHR_444 => DNXHD::HR_444,

                _ => return Profile::Unknown,
            }),

            Id::DTS => match value {
                FF_PROFILE_DTS => Profile::DTS(DTS::Default),
                FF_PROFILE_DTS_ES => Profile::DTS(DTS::ES),
                FF_PROFILE_DTS_96_24 => Profile::DTS(DTS::_96_24),
                FF_PROFILE_DTS_HD_HRA => Profile::DTS(DTS::HD_HRA),
                FF_PROFILE_DTS_HD_MA => Profile::DTS(DTS::HD_MA),
                FF_PROFILE_DTS_EXPRESS => Profile::DTS(DTS::Express),

                _ => Profile::Unknown,
            },

            Id::MPEG2VIDEO => match value {
                FF_PROFILE_MPEG2_422 => Profile::MPEG2(MPEG2::_422),
                FF_PROFILE_MPEG2_HIGH => Profile::MPEG2(MPEG2::High),
                FF_PROFILE_MPEG2_SS => Profile::MPEG2(MPEG2::SS),
                FF_PROFILE_MPEG2_SNR_SCALABLE => Profile::MPEG2(MPEG2::SNRScalable),
                FF_PROFILE_MPEG2_MAIN => Profile::MPEG2(MPEG2::Main),
                FF_PROFILE_MPEG2_SIMPLE => Profile::MPEG2(MPEG2::Simple),

                _ => Profile::Unknown,
            },

            Id::H264 => match value {
                FF_PROFILE_H264_CONSTRAINED => Profile::H264(H264::Constrained),
                FF_PROFILE_H264_INTRA => Profile::H264(H264::Intra),
                FF_PROFILE_H264_BASELINE => Profile::H264(H264::Baseline),
                FF_PROFILE_H264_CONSTRAINED_BASELINE => Profile::H264(H264::ConstrainedBaseline),
                FF_PROFILE_H264_MAIN => Profile::H264(H264::Main),
                FF_PROFILE_H264_EXTENDED => Profile::H264(H264::Extended),
                FF_PROFILE_H264_HIGH => Profile::H264(H264::High),
                FF_PROFILE_H264_HIGH_10 => Profile::H264(H264::High10),
                FF_PROFILE_H264_HIGH_10_INTRA => Profile::H264(H264::High10Intra),
                FF_PROFILE_H264_HIGH_422 => Profile::H264(H264::High422),
                FF_PROFILE_H264_HIGH_422_INTRA => Profile::H264(H264::High422Intra),
                FF_PROFILE_H264_HIGH_444 => Profile::H264(H264::High444),
                FF_PROFILE_H264_HIGH_444_PREDICTIVE => Profile::H264(H264::High444Predictive),
                FF_PROFILE_H264_HIGH_444_INTRA => Profile::H264(H264::High444Intra),
                FF_PROFILE_H264_CAVLC_444 => Profile::H264(H264::CAVLC444),

                _ => Profile::Unknown,
            },

            Id::VC1 => match value {
                FF_PROFILE_VC1_SIMPLE => Profile::VC1(VC1::Simple),
                FF_PROFILE_VC1_MAIN => Profile::VC1(VC1::Main),
                FF_PROFILE_VC1_COMPLEX => Profile::VC1(VC1::Complex),
                FF_PROFILE_VC1_ADVANCED => Profile::VC1(VC1::Advanced),

                _ => Profile::Unknown,
            },

            Id::MPEG4 => match value {
                FF_PROFILE_MPEG4_SIMPLE => Profile::MPEG4(MPEG4::Simple),
                FF_PROFILE_MPEG4_SIMPLE_SCALABLE => Profile::MPEG4(MPEG4::SimpleScalable),
                FF_PROFILE_MPEG4_CORE => Profile::MPEG4(MPEG4::Core),
                FF_PROFILE_MPEG4_MAIN => Profile::MPEG4(MPEG4::Main),
                FF_PROFILE_MPEG4_N_BIT => Profile::MPEG4(MPEG4::NBit),
                FF_PROFILE_MPEG4_SCALABLE_TEXTURE => Profile::MPEG4(MPEG4::ScalableTexture),
                FF_PROFILE_MPEG4_SIMPLE_FACE_ANIMATION => {
                    Profile::MPEG4(MPEG4::SimpleFaceAnimation)
                }
                FF_PROFILE_MPEG4_BASIC_ANIMATED_TEXTURE => {
                    Profile::MPEG4(MPEG4::BasicAnimatedTexture)
                }
                FF_PROFILE_MPEG4_HYBRID => Profile::MPEG4(MPEG4::Hybrid),
                FF_PROFILE_MPEG4_ADVANCED_REAL_TIME => Profile::MPEG4(MPEG4::AdvancedRealTime),
                FF_PROFILE_MPEG4_CORE_SCALABLE => Profile::MPEG4(MPEG4::CoreScalable),
                FF_PROFILE_MPEG4_ADVANCED_CODING => Profile::MPEG4(MPEG4::AdvancedCoding),
                FF_PROFILE_MPEG4_ADVANCED_CORE => Profile::MPEG4(MPEG4::AdvancedCore),
                FF_PROFILE_MPEG4_ADVANCED_SCALABLE_TEXTURE => {
                    Profile::MPEG4(MPEG4::AdvancedScalableTexture)
                }
                FF_PROFILE_MPEG4_SIMPLE_STUDIO => Profile::MPEG4(MPEG4::SimpleStudio),
                FF_PROFILE_MPEG4_ADVANCED_SIMPLE => Profile::MPEG4(MPEG4::AdvancedSimple),

                _ => Profile::Unknown,
            },

            Id::JPEG2000 => match value {
                FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_0 => {
                    Profile::JPEG2000(JPEG2000::CStreamRestriction0)
                }
                FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_1 => {
                    Profile::JPEG2000(JPEG2000::CStreamRestriction1)
                }
                FF_PROFILE_JPEG2000_CSTREAM_NO_RESTRICTION => {
                    Profile::JPEG2000(JPEG2000::CStreamNoRestriction)
                }
                FF_PROFILE_JPEG2000_DCINEMA_2K => Profile::JPEG2000(JPEG2000::DCinema2K),
                FF_PROFILE_JPEG2000_DCINEMA_4K => Profile::JPEG2000(JPEG2000::DCinema4K),

                _ => Profile::Unknown,
            },

            Id::VP9 => match value {
                FF_PROFILE_VP9_0 => Profile::VP9(VP9::_0),
                FF_PROFILE_VP9_1 => Profile::VP9(VP9::_1),
                FF_PROFILE_VP9_2 => Profile::VP9(VP9::_2),
                FF_PROFILE_VP9_3 => Profile::VP9(VP9::_3),

                _ => Profile::Unknown,
            },

            Id::HEVC => match value {
                FF_PROFILE_HEVC_MAIN => Profile::HEVC(HEVC::Main),
                FF_PROFILE_HEVC_MAIN_10 => Profile::HEVC(HEVC::Main10),
                FF_PROFILE_HEVC_MAIN_STILL_PICTURE => Profile::HEVC(HEVC::MainStillPicture),
                FF_PROFILE_HEVC_REXT => Profile::HEVC(HEVC::Rext),

                _ => Profile::Unknown,
            },

            #[cfg(feature = "ffmpeg_4_4")]
            Id::VVC => Profile::VVC(match value {
                FF_PROFILE_VVC_MAIN_10 => VVC::Main10,
                FF_PROFILE_VVC_MAIN_10_444 => VVC::Main10_444,

                _ => return Profile::Unknown,
            }),

            Id::AV1 => Profile::AV1(match value {
                FF_PROFILE_AV1_MAIN => AV1::Main,
                FF_PROFILE_AV1_HIGH => AV1::High,
                FF_PROFILE_AV1_PROFESSIONAL => AV1::Professional,

                _ => return Profile::Unknown,
            }),

            Id::MJPEG => Profile::MJPEG(match value {
                FF_PROFILE_MJPEG_HUFFMAN_BASELINE_DCT => MJPEG::HuffmanBaselineDCT,
                FF_PROFILE_MJPEG_HUFFMAN_EXTENDED_SEQUENTIAL_DCT => {
                    MJPEG::HuffmanExtendedSequentialDCT
                }
                FF_PROFILE_MJPEG_HUFFMAN_PROGRESSIVE_DCT => MJPEG::HuffmanProgressiveDCT,
                FF_PROFILE_MJPEG_HUFFMAN_LOSSLESS => MJPEG::HuffmanLossless,
                FF_PROFILE_MJPEG_JPEG_LS => MJPEG::JPEG_LS,

                _ => return Profile::Unknown,
            }),

            Id::PRORES => Profile::ProRes(match value {
                FF_PROFILE_PRORES_PROXY => ProRes::Proxy,
                FF_PROFILE_PRORES_LT => ProRes::LT,
                FF_PROFILE_PRORES_STANDARD => ProRes::Standard,
                FF_PROFILE_PRORES_HQ => ProRes::HQ,
                FF_PROFILE_PRORES_4444 => ProRes::_4444,
                FF_PROFILE_PRORES_XQ => ProRes::XQ,

                _ => return Profile::Unknown,
            }),

            #[cfg(feature = "ffmpeg_8_0")]
            Id::PRORES_RAW => Profile::ProResRAW(match value {
                FF_PROFILE_PRORES_RAW => ProResRAW::Default,
                FF_PROFILE_PRORES_RAW_HQ => ProResRAW::HQ,

                _ => return Profile::Unknown,
            }),

            Id::ARIB_CAPTION => Profile::ARIB(match value {
                FF_PROFILE_ARIB_PROFILE_A => ARIB::ProfileA,
                FF_PROFILE_ARIB_PROFILE_C => ARIB::ProfileC,

                _ => return Profile::Unknown,
            }),

            #[cfg(feature = "ffmpeg_4_3")]
            Id::SMPTE_KLV => Profile::KLVA(match value {
                FF_PROFILE_KLVA_SYNC => KLVA::Sync,
                FF_PROFILE_KLVA_ASYNC => KLVA::Async,

                _ => return Profile::Unknown,
            }),

            #[cfg(feature = "ffmpeg_6_1")]
            Id::EVC => Profile::EVC(match value {
                FF_PROFILE_EVC_BASELINE => EVC::Baseline,
                FF_PROFILE_EVC_MAIN => EVC::Main,

                _ => return Profile::Unknown,
            }),

            #[cfg(feature = "ffmpeg_8_0")]
            Id::APV => Profile::APV(match value {
                FF_PROFILE_APV_422_10 => APV::_422_10,
                FF_PROFILE_APV_422_12 => APV::_422_12,
                FF_PROFILE_APV_444_10 => APV::_444_10,
                FF_PROFILE_APV_444_12 => APV::_444_12,
                FF_PROFILE_APV_4444_10 => APV::_4444_10,
                FF_PROFILE_APV_4444_12 => APV::_4444_12,
                FF_PROFILE_APV_400_10 => APV::_400_10,

                _ => return Profile::Unknown,
            }),

            _ => Profile::Unknown,
        }
    }
}

impl From<Profile> for c_int {
    fn from(value: Profile) -> c_int {
        match value {
            Profile::Unknown => FF_PROFILE_UNKNOWN,
            Profile::Reserved => FF_PROFILE_RESERVED,

            Profile::AAC(AAC::Main) => FF_PROFILE_AAC_MAIN,
            Profile::AAC(AAC::Low) => FF_PROFILE_AAC_LOW,
            Profile::AAC(AAC::SSR) => FF_PROFILE_AAC_SSR,
            Profile::AAC(AAC::LTP) => FF_PROFILE_AAC_LTP,
            Profile::AAC(AAC::HE) => FF_PROFILE_AAC_HE,
            Profile::AAC(AAC::HEv2) => FF_PROFILE_AAC_HE_V2,
            Profile::AAC(AAC::LD) => FF_PROFILE_AAC_LD,
            Profile::AAC(AAC::ELD) => FF_PROFILE_AAC_ELD,

            Profile::AAC(AAC::MPEG2Low) => FF_PROFILE_MPEG2_AAC_LOW,
            Profile::AAC(AAC::MPEG2HE) => FF_PROFILE_MPEG2_AAC_HE,

            Profile::DNXHD(DNXHD::HD) => FF_PROFILE_DNXHD,
            Profile::DNXHD(DNXHD::HR_LB) => FF_PROFILE_DNXHR_LB,
            Profile::DNXHD(DNXHD::HR_SQ) => FF_PROFILE_DNXHR_SQ,
            Profile::DNXHD(DNXHD::HR_HQ) => FF_PROFILE_DNXHR_HQ,
            Profile::DNXHD(DNXHD::HR_HQX) => FF_PROFILE_DNXHR_HQX,
            Profile::DNXHD(DNXHD::HR_444) => FF_PROFILE_DNXHR_444,

            Profile::DTS(DTS::Default) => FF_PROFILE_DTS,
            Profile::DTS(DTS::ES) => FF_PROFILE_DTS_ES,
            Profile::DTS(DTS::_96_24) => FF_PROFILE_DTS_96_24,
            Profile::DTS(DTS::HD_HRA) => FF_PROFILE_DTS_HD_HRA,
            Profile::DTS(DTS::HD_MA) => FF_PROFILE_DTS_HD_MA,
            Profile::DTS(DTS::Express) => FF_PROFILE_DTS_EXPRESS,

            Profile::MPEG2(MPEG2::_422) => FF_PROFILE_MPEG2_422,
            Profile::MPEG2(MPEG2::High) => FF_PROFILE_MPEG2_HIGH,
            Profile::MPEG2(MPEG2::SS) => FF_PROFILE_MPEG2_SS,
            Profile::MPEG2(MPEG2::SNRScalable) => FF_PROFILE_MPEG2_SNR_SCALABLE,
            Profile::MPEG2(MPEG2::Main) => FF_PROFILE_MPEG2_MAIN,
            Profile::MPEG2(MPEG2::Simple) => FF_PROFILE_MPEG2_SIMPLE,

            Profile::H264(H264::Constrained) => FF_PROFILE_H264_CONSTRAINED,
            Profile::H264(H264::Intra) => FF_PROFILE_H264_INTRA,
            Profile::H264(H264::Baseline) => FF_PROFILE_H264_BASELINE,
            Profile::H264(H264::ConstrainedBaseline) => FF_PROFILE_H264_CONSTRAINED_BASELINE,
            Profile::H264(H264::Main) => FF_PROFILE_H264_MAIN,
            Profile::H264(H264::Extended) => FF_PROFILE_H264_EXTENDED,
            Profile::H264(H264::High) => FF_PROFILE_H264_HIGH,
            Profile::H264(H264::High10) => FF_PROFILE_H264_HIGH_10,
            Profile::H264(H264::High10Intra) => FF_PROFILE_H264_HIGH_10_INTRA,
            Profile::H264(H264::High422) => FF_PROFILE_H264_HIGH_422,
            Profile::H264(H264::High422Intra) => FF_PROFILE_H264_HIGH_422_INTRA,
            Profile::H264(H264::High444) => FF_PROFILE_H264_HIGH_444,
            Profile::H264(H264::High444Predictive) => FF_PROFILE_H264_HIGH_444_PREDICTIVE,
            Profile::H264(H264::High444Intra) => FF_PROFILE_H264_HIGH_444_INTRA,
            Profile::H264(H264::CAVLC444) => FF_PROFILE_H264_CAVLC_444,

            Profile::VC1(VC1::Simple) => FF_PROFILE_VC1_SIMPLE,
            Profile::VC1(VC1::Main) => FF_PROFILE_VC1_MAIN,
            Profile::VC1(VC1::Complex) => FF_PROFILE_VC1_COMPLEX,
            Profile::VC1(VC1::Advanced) => FF_PROFILE_VC1_ADVANCED,

            Profile::MPEG4(MPEG4::Simple) => FF_PROFILE_MPEG4_SIMPLE,
            Profile::MPEG4(MPEG4::SimpleScalable) => FF_PROFILE_MPEG4_SIMPLE_SCALABLE,
            Profile::MPEG4(MPEG4::Core) => FF_PROFILE_MPEG4_CORE,
            Profile::MPEG4(MPEG4::Main) => FF_PROFILE_MPEG4_MAIN,
            Profile::MPEG4(MPEG4::NBit) => FF_PROFILE_MPEG4_N_BIT,
            Profile::MPEG4(MPEG4::ScalableTexture) => FF_PROFILE_MPEG4_SCALABLE_TEXTURE,
            Profile::MPEG4(MPEG4::SimpleFaceAnimation) => FF_PROFILE_MPEG4_SIMPLE_FACE_ANIMATION,
            Profile::MPEG4(MPEG4::BasicAnimatedTexture) => FF_PROFILE_MPEG4_BASIC_ANIMATED_TEXTURE,
            Profile::MPEG4(MPEG4::Hybrid) => FF_PROFILE_MPEG4_HYBRID,
            Profile::MPEG4(MPEG4::AdvancedRealTime) => FF_PROFILE_MPEG4_ADVANCED_REAL_TIME,
            Profile::MPEG4(MPEG4::CoreScalable) => FF_PROFILE_MPEG4_CORE_SCALABLE,
            Profile::MPEG4(MPEG4::AdvancedCoding) => FF_PROFILE_MPEG4_ADVANCED_CODING,
            Profile::MPEG4(MPEG4::AdvancedCore) => FF_PROFILE_MPEG4_ADVANCED_CORE,
            Profile::MPEG4(MPEG4::AdvancedScalableTexture) => {
                FF_PROFILE_MPEG4_ADVANCED_SCALABLE_TEXTURE
            }
            Profile::MPEG4(MPEG4::SimpleStudio) => FF_PROFILE_MPEG4_SIMPLE_STUDIO,
            Profile::MPEG4(MPEG4::AdvancedSimple) => FF_PROFILE_MPEG4_ADVANCED_SIMPLE,

            Profile::JPEG2000(JPEG2000::CStreamRestriction0) => {
                FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_0
            }
            Profile::JPEG2000(JPEG2000::CStreamRestriction1) => {
                FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_1
            }
            Profile::JPEG2000(JPEG2000::CStreamNoRestriction) => {
                FF_PROFILE_JPEG2000_CSTREAM_NO_RESTRICTION
            }
            Profile::JPEG2000(JPEG2000::DCinema2K) => FF_PROFILE_JPEG2000_DCINEMA_2K,
            Profile::JPEG2000(JPEG2000::DCinema4K) => FF_PROFILE_JPEG2000_DCINEMA_4K,

            Profile::VP9(VP9::_0) => FF_PROFILE_VP9_0,
            Profile::VP9(VP9::_1) => FF_PROFILE_VP9_1,
            Profile::VP9(VP9::_2) => FF_PROFILE_VP9_2,
            Profile::VP9(VP9::_3) => FF_PROFILE_VP9_3,

            Profile::HEVC(HEVC::Main) => FF_PROFILE_HEVC_MAIN,
            Profile::HEVC(HEVC::Main10) => FF_PROFILE_HEVC_MAIN_10,
            Profile::HEVC(HEVC::MainStillPicture) => FF_PROFILE_HEVC_MAIN_STILL_PICTURE,
            Profile::HEVC(HEVC::Rext) => FF_PROFILE_HEVC_REXT,

            #[cfg(feature = "ffmpeg_4_4")]
            Profile::VVC(VVC::Main10) => FF_PROFILE_VVC_MAIN_10,
            #[cfg(feature = "ffmpeg_4_4")]
            Profile::VVC(VVC::Main10_444) => FF_PROFILE_VVC_MAIN_10_444,

            Profile::AV1(AV1::Main) => FF_PROFILE_AV1_MAIN,
            Profile::AV1(AV1::High) => FF_PROFILE_AV1_HIGH,
            Profile::AV1(AV1::Professional) => FF_PROFILE_AV1_PROFESSIONAL,

            Profile::MJPEG(MJPEG::HuffmanBaselineDCT) => FF_PROFILE_MJPEG_HUFFMAN_BASELINE_DCT,
            Profile::MJPEG(MJPEG::HuffmanExtendedSequentialDCT) => {
                FF_PROFILE_MJPEG_HUFFMAN_EXTENDED_SEQUENTIAL_DCT
            }
            Profile::MJPEG(MJPEG::HuffmanProgressiveDCT) => {
                FF_PROFILE_MJPEG_HUFFMAN_PROGRESSIVE_DCT
            }
            Profile::MJPEG(MJPEG::HuffmanLossless) => FF_PROFILE_MJPEG_HUFFMAN_LOSSLESS,
            Profile::MJPEG(MJPEG::JPEG_LS) => FF_PROFILE_MJPEG_JPEG_LS,

            Profile::SBC_MSBC => FF_PROFILE_SBC_MSBC,

            Profile::ProRes(ProRes::Proxy) => FF_PROFILE_PRORES_PROXY,
            Profile::ProRes(ProRes::LT) => FF_PROFILE_PRORES_LT,
            Profile::ProRes(ProRes::Standard) => FF_PROFILE_PRORES_STANDARD,
            Profile::ProRes(ProRes::HQ) => FF_PROFILE_PRORES_HQ,
            Profile::ProRes(ProRes::_4444) => FF_PROFILE_PRORES_4444,
            Profile::ProRes(ProRes::XQ) => FF_PROFILE_PRORES_XQ,

            #[cfg(feature = "ffmpeg_8_0")]
            Profile::ProResRAW(ProResRAW::Default) => FF_PROFILE_PRORES_RAW,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::ProResRAW(ProResRAW::HQ) => FF_PROFILE_PRORES_RAW_HQ,

            Profile::ARIB(ARIB::ProfileA) => FF_PROFILE_ARIB_PROFILE_A,
            Profile::ARIB(ARIB::ProfileC) => FF_PROFILE_ARIB_PROFILE_C,

            #[cfg(feature = "ffmpeg_4_3")]
            Profile::KLVA(KLVA::Sync) => FF_PROFILE_KLVA_SYNC,
            #[cfg(feature = "ffmpeg_4_3")]
            Profile::KLVA(KLVA::Async) => FF_PROFILE_KLVA_ASYNC,

            #[cfg(feature = "ffmpeg_6_1")]
            Profile::EVC(EVC::Baseline) => FF_PROFILE_EVC_BASELINE,
            #[cfg(feature = "ffmpeg_6_1")]
            Profile::EVC(EVC::Main) => FF_PROFILE_EVC_MAIN,

            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_422_10) => FF_PROFILE_APV_422_10,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_422_12) => FF_PROFILE_APV_422_12,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_444_10) => FF_PROFILE_APV_444_10,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_444_12) => FF_PROFILE_APV_444_12,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_4444_10) => FF_PROFILE_APV_4444_10,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_4444_12) => FF_PROFILE_APV_4444_12,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_400_10) => FF_PROFILE_APV_400_10,
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

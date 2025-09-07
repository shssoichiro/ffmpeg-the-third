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
    EAC3_DDP_Atmos,
    TrueHD_Atmos,
    MPEG2(MPEG2),
    H264(H264),
    VC1(VC1),
    MPEG4(MPEG4),
    JPEG2000(JPEG2000),
    HEVC(HEVC),
    VP9(VP9),
    VVC(VVC),
    AV1(AV1),
    MJPEG(MJPEG),
    SBC_MSBC,
    ProRes(ProRes),
    #[cfg(feature = "ffmpeg_8_0")]
    ProResRAW(ProResRAW),
    ARIB(ARIB),
    KLVA(KLVA),
    EVC(EVC),
    #[cfg(feature = "ffmpeg_8_0")]
    APV(APV),
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
    HD_MA_X,
    HD_MA_X_IMAX,
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

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum KLVA {
    Sync,
    Async,
}

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
        if value == AV_PROFILE_UNKNOWN {
            return Profile::Unknown;
        }

        if value == AV_PROFILE_RESERVED {
            return Profile::Reserved;
        }

        match id {
            Id::AAC => match value {
                AV_PROFILE_AAC_MAIN => Profile::AAC(AAC::Main),
                AV_PROFILE_AAC_LOW => Profile::AAC(AAC::Low),
                AV_PROFILE_AAC_SSR => Profile::AAC(AAC::SSR),
                AV_PROFILE_AAC_LTP => Profile::AAC(AAC::LTP),
                AV_PROFILE_AAC_HE => Profile::AAC(AAC::HE),
                AV_PROFILE_AAC_HE_V2 => Profile::AAC(AAC::HEv2),
                AV_PROFILE_AAC_LD => Profile::AAC(AAC::LD),
                AV_PROFILE_AAC_ELD => Profile::AAC(AAC::ELD),

                AV_PROFILE_MPEG2_AAC_LOW => Profile::AAC(AAC::MPEG2Low),
                AV_PROFILE_MPEG2_AAC_HE => Profile::AAC(AAC::MPEG2HE),

                _ => Profile::Unknown,
            },

            Id::DNXHD => Profile::DNXHD(match value {
                AV_PROFILE_DNXHD => DNXHD::HD,
                AV_PROFILE_DNXHR_LB => DNXHD::HR_LB,
                AV_PROFILE_DNXHR_SQ => DNXHD::HR_SQ,
                AV_PROFILE_DNXHR_HQ => DNXHD::HR_HQ,
                AV_PROFILE_DNXHR_HQX => DNXHD::HR_HQX,
                AV_PROFILE_DNXHR_444 => DNXHD::HR_444,

                _ => return Profile::Unknown,
            }),

            Id::DTS => match value {
                AV_PROFILE_DTS => Profile::DTS(DTS::Default),
                AV_PROFILE_DTS_ES => Profile::DTS(DTS::ES),
                AV_PROFILE_DTS_96_24 => Profile::DTS(DTS::_96_24),
                AV_PROFILE_DTS_HD_HRA => Profile::DTS(DTS::HD_HRA),
                AV_PROFILE_DTS_HD_MA => Profile::DTS(DTS::HD_MA),
                AV_PROFILE_DTS_EXPRESS => Profile::DTS(DTS::Express),

                _ => Profile::Unknown,
            },

            Id::MPEG2VIDEO => match value {
                AV_PROFILE_MPEG2_422 => Profile::MPEG2(MPEG2::_422),
                AV_PROFILE_MPEG2_HIGH => Profile::MPEG2(MPEG2::High),
                AV_PROFILE_MPEG2_SS => Profile::MPEG2(MPEG2::SS),
                AV_PROFILE_MPEG2_SNR_SCALABLE => Profile::MPEG2(MPEG2::SNRScalable),
                AV_PROFILE_MPEG2_MAIN => Profile::MPEG2(MPEG2::Main),
                AV_PROFILE_MPEG2_SIMPLE => Profile::MPEG2(MPEG2::Simple),

                _ => Profile::Unknown,
            },

            Id::H264 => match value {
                AV_PROFILE_H264_CONSTRAINED => Profile::H264(H264::Constrained),
                AV_PROFILE_H264_INTRA => Profile::H264(H264::Intra),
                AV_PROFILE_H264_BASELINE => Profile::H264(H264::Baseline),
                AV_PROFILE_H264_CONSTRAINED_BASELINE => Profile::H264(H264::ConstrainedBaseline),
                AV_PROFILE_H264_MAIN => Profile::H264(H264::Main),
                AV_PROFILE_H264_EXTENDED => Profile::H264(H264::Extended),
                AV_PROFILE_H264_HIGH => Profile::H264(H264::High),
                AV_PROFILE_H264_HIGH_10 => Profile::H264(H264::High10),
                AV_PROFILE_H264_HIGH_10_INTRA => Profile::H264(H264::High10Intra),
                AV_PROFILE_H264_HIGH_422 => Profile::H264(H264::High422),
                AV_PROFILE_H264_HIGH_422_INTRA => Profile::H264(H264::High422Intra),
                AV_PROFILE_H264_HIGH_444 => Profile::H264(H264::High444),
                AV_PROFILE_H264_HIGH_444_PREDICTIVE => Profile::H264(H264::High444Predictive),
                AV_PROFILE_H264_HIGH_444_INTRA => Profile::H264(H264::High444Intra),
                AV_PROFILE_H264_CAVLC_444 => Profile::H264(H264::CAVLC444),

                _ => Profile::Unknown,
            },

            Id::VC1 => match value {
                AV_PROFILE_VC1_SIMPLE => Profile::VC1(VC1::Simple),
                AV_PROFILE_VC1_MAIN => Profile::VC1(VC1::Main),
                AV_PROFILE_VC1_COMPLEX => Profile::VC1(VC1::Complex),
                AV_PROFILE_VC1_ADVANCED => Profile::VC1(VC1::Advanced),

                _ => Profile::Unknown,
            },

            Id::MPEG4 => match value {
                AV_PROFILE_MPEG4_SIMPLE => Profile::MPEG4(MPEG4::Simple),
                AV_PROFILE_MPEG4_SIMPLE_SCALABLE => Profile::MPEG4(MPEG4::SimpleScalable),
                AV_PROFILE_MPEG4_CORE => Profile::MPEG4(MPEG4::Core),
                AV_PROFILE_MPEG4_MAIN => Profile::MPEG4(MPEG4::Main),
                AV_PROFILE_MPEG4_N_BIT => Profile::MPEG4(MPEG4::NBit),
                AV_PROFILE_MPEG4_SCALABLE_TEXTURE => Profile::MPEG4(MPEG4::ScalableTexture),
                AV_PROFILE_MPEG4_SIMPLE_FACE_ANIMATION => {
                    Profile::MPEG4(MPEG4::SimpleFaceAnimation)
                }
                AV_PROFILE_MPEG4_BASIC_ANIMATED_TEXTURE => {
                    Profile::MPEG4(MPEG4::BasicAnimatedTexture)
                }
                AV_PROFILE_MPEG4_HYBRID => Profile::MPEG4(MPEG4::Hybrid),
                AV_PROFILE_MPEG4_ADVANCED_REAL_TIME => Profile::MPEG4(MPEG4::AdvancedRealTime),
                AV_PROFILE_MPEG4_CORE_SCALABLE => Profile::MPEG4(MPEG4::CoreScalable),
                AV_PROFILE_MPEG4_ADVANCED_CODING => Profile::MPEG4(MPEG4::AdvancedCoding),
                AV_PROFILE_MPEG4_ADVANCED_CORE => Profile::MPEG4(MPEG4::AdvancedCore),
                AV_PROFILE_MPEG4_ADVANCED_SCALABLE_TEXTURE => {
                    Profile::MPEG4(MPEG4::AdvancedScalableTexture)
                }
                AV_PROFILE_MPEG4_SIMPLE_STUDIO => Profile::MPEG4(MPEG4::SimpleStudio),
                AV_PROFILE_MPEG4_ADVANCED_SIMPLE => Profile::MPEG4(MPEG4::AdvancedSimple),

                _ => Profile::Unknown,
            },

            Id::JPEG2000 => match value {
                AV_PROFILE_JPEG2000_CSTREAM_RESTRICTION_0 => {
                    Profile::JPEG2000(JPEG2000::CStreamRestriction0)
                }
                AV_PROFILE_JPEG2000_CSTREAM_RESTRICTION_1 => {
                    Profile::JPEG2000(JPEG2000::CStreamRestriction1)
                }
                AV_PROFILE_JPEG2000_CSTREAM_NO_RESTRICTION => {
                    Profile::JPEG2000(JPEG2000::CStreamNoRestriction)
                }
                AV_PROFILE_JPEG2000_DCINEMA_2K => Profile::JPEG2000(JPEG2000::DCinema2K),
                AV_PROFILE_JPEG2000_DCINEMA_4K => Profile::JPEG2000(JPEG2000::DCinema4K),

                _ => Profile::Unknown,
            },

            Id::VP9 => match value {
                AV_PROFILE_VP9_0 => Profile::VP9(VP9::_0),
                AV_PROFILE_VP9_1 => Profile::VP9(VP9::_1),
                AV_PROFILE_VP9_2 => Profile::VP9(VP9::_2),
                AV_PROFILE_VP9_3 => Profile::VP9(VP9::_3),

                _ => Profile::Unknown,
            },

            Id::HEVC => match value {
                AV_PROFILE_HEVC_MAIN => Profile::HEVC(HEVC::Main),
                AV_PROFILE_HEVC_MAIN_10 => Profile::HEVC(HEVC::Main10),
                AV_PROFILE_HEVC_MAIN_STILL_PICTURE => Profile::HEVC(HEVC::MainStillPicture),
                AV_PROFILE_HEVC_REXT => Profile::HEVC(HEVC::Rext),

                _ => Profile::Unknown,
            },

            Id::VVC => Profile::VVC(match value {
                AV_PROFILE_VVC_MAIN_10 => VVC::Main10,
                AV_PROFILE_VVC_MAIN_10_444 => VVC::Main10_444,

                _ => return Profile::Unknown,
            }),

            Id::AV1 => Profile::AV1(match value {
                AV_PROFILE_AV1_MAIN => AV1::Main,
                AV_PROFILE_AV1_HIGH => AV1::High,
                AV_PROFILE_AV1_PROFESSIONAL => AV1::Professional,

                _ => return Profile::Unknown,
            }),

            Id::MJPEG => Profile::MJPEG(match value {
                AV_PROFILE_MJPEG_HUFFMAN_BASELINE_DCT => MJPEG::HuffmanBaselineDCT,
                AV_PROFILE_MJPEG_HUFFMAN_EXTENDED_SEQUENTIAL_DCT => {
                    MJPEG::HuffmanExtendedSequentialDCT
                }
                AV_PROFILE_MJPEG_HUFFMAN_PROGRESSIVE_DCT => MJPEG::HuffmanProgressiveDCT,
                AV_PROFILE_MJPEG_HUFFMAN_LOSSLESS => MJPEG::HuffmanLossless,
                AV_PROFILE_MJPEG_JPEG_LS => MJPEG::JPEG_LS,

                _ => return Profile::Unknown,
            }),

            Id::PRORES => Profile::ProRes(match value {
                AV_PROFILE_PRORES_PROXY => ProRes::Proxy,
                AV_PROFILE_PRORES_LT => ProRes::LT,
                AV_PROFILE_PRORES_STANDARD => ProRes::Standard,
                AV_PROFILE_PRORES_HQ => ProRes::HQ,
                AV_PROFILE_PRORES_4444 => ProRes::_4444,
                AV_PROFILE_PRORES_XQ => ProRes::XQ,

                _ => return Profile::Unknown,
            }),

            #[cfg(feature = "ffmpeg_8_0")]
            Id::PRORES_RAW => Profile::ProResRAW(match value {
                AV_PROFILE_PRORES_RAW => ProResRAW::Default,
                AV_PROFILE_PRORES_RAW_HQ => ProResRAW::HQ,

                _ => return Profile::Unknown,
            }),

            Id::ARIB_CAPTION => Profile::ARIB(match value {
                AV_PROFILE_ARIB_PROFILE_A => ARIB::ProfileA,
                AV_PROFILE_ARIB_PROFILE_C => ARIB::ProfileC,

                _ => return Profile::Unknown,
            }),

            Id::SMPTE_KLV => Profile::KLVA(match value {
                AV_PROFILE_KLVA_SYNC => KLVA::Sync,
                AV_PROFILE_KLVA_ASYNC => KLVA::Async,

                _ => return Profile::Unknown,
            }),

            Id::EVC => Profile::EVC(match value {
                AV_PROFILE_EVC_BASELINE => EVC::Baseline,
                AV_PROFILE_EVC_MAIN => EVC::Main,

                _ => return Profile::Unknown,
            }),

            #[cfg(feature = "ffmpeg_8_0")]
            Id::APV => Profile::APV(match value {
                AV_PROFILE_APV_422_10 => APV::_422_10,
                AV_PROFILE_APV_422_12 => APV::_422_12,
                AV_PROFILE_APV_444_10 => APV::_444_10,
                AV_PROFILE_APV_444_12 => APV::_444_12,
                AV_PROFILE_APV_4444_10 => APV::_4444_10,
                AV_PROFILE_APV_4444_12 => APV::_4444_12,
                AV_PROFILE_APV_400_10 => APV::_400_10,

                _ => return Profile::Unknown,
            }),

            _ => Profile::Unknown,
        }
    }
}

impl From<Profile> for c_int {
    fn from(value: Profile) -> c_int {
        match value {
            Profile::Unknown => AV_PROFILE_UNKNOWN,
            Profile::Reserved => AV_PROFILE_RESERVED,

            Profile::AAC(AAC::Main) => AV_PROFILE_AAC_MAIN,
            Profile::AAC(AAC::Low) => AV_PROFILE_AAC_LOW,
            Profile::AAC(AAC::SSR) => AV_PROFILE_AAC_SSR,
            Profile::AAC(AAC::LTP) => AV_PROFILE_AAC_LTP,
            Profile::AAC(AAC::HE) => AV_PROFILE_AAC_HE,
            Profile::AAC(AAC::HEv2) => AV_PROFILE_AAC_HE_V2,
            Profile::AAC(AAC::LD) => AV_PROFILE_AAC_LD,
            Profile::AAC(AAC::ELD) => AV_PROFILE_AAC_ELD,

            Profile::AAC(AAC::MPEG2Low) => AV_PROFILE_MPEG2_AAC_LOW,
            Profile::AAC(AAC::MPEG2HE) => AV_PROFILE_MPEG2_AAC_HE,

            Profile::DNXHD(DNXHD::HD) => AV_PROFILE_DNXHD,
            Profile::DNXHD(DNXHD::HR_LB) => AV_PROFILE_DNXHR_LB,
            Profile::DNXHD(DNXHD::HR_SQ) => AV_PROFILE_DNXHR_SQ,
            Profile::DNXHD(DNXHD::HR_HQ) => AV_PROFILE_DNXHR_HQ,
            Profile::DNXHD(DNXHD::HR_HQX) => AV_PROFILE_DNXHR_HQX,
            Profile::DNXHD(DNXHD::HR_444) => AV_PROFILE_DNXHR_444,

            Profile::DTS(DTS::Default) => AV_PROFILE_DTS,
            Profile::DTS(DTS::ES) => AV_PROFILE_DTS_ES,
            Profile::DTS(DTS::_96_24) => AV_PROFILE_DTS_96_24,
            Profile::DTS(DTS::HD_HRA) => AV_PROFILE_DTS_HD_HRA,
            Profile::DTS(DTS::HD_MA) => AV_PROFILE_DTS_HD_MA,
            Profile::DTS(DTS::Express) => AV_PROFILE_DTS_EXPRESS,
            Profile::DTS(DTS::HD_MA_X) => AV_PROFILE_DTS_HD_MA_X,
            Profile::DTS(DTS::HD_MA_X_IMAX) => AV_PROFILE_DTS_HD_MA_X_IMAX,

            Profile::EAC3_DDP_Atmos => AV_PROFILE_EAC3_DDP_ATMOS,

            Profile::TrueHD_Atmos => AV_PROFILE_TRUEHD_ATMOS,

            Profile::MPEG2(MPEG2::_422) => AV_PROFILE_MPEG2_422,
            Profile::MPEG2(MPEG2::High) => AV_PROFILE_MPEG2_HIGH,
            Profile::MPEG2(MPEG2::SS) => AV_PROFILE_MPEG2_SS,
            Profile::MPEG2(MPEG2::SNRScalable) => AV_PROFILE_MPEG2_SNR_SCALABLE,
            Profile::MPEG2(MPEG2::Main) => AV_PROFILE_MPEG2_MAIN,
            Profile::MPEG2(MPEG2::Simple) => AV_PROFILE_MPEG2_SIMPLE,

            Profile::H264(H264::Constrained) => AV_PROFILE_H264_CONSTRAINED,
            Profile::H264(H264::Intra) => AV_PROFILE_H264_INTRA,
            Profile::H264(H264::Baseline) => AV_PROFILE_H264_BASELINE,
            Profile::H264(H264::ConstrainedBaseline) => AV_PROFILE_H264_CONSTRAINED_BASELINE,
            Profile::H264(H264::Main) => AV_PROFILE_H264_MAIN,
            Profile::H264(H264::Extended) => AV_PROFILE_H264_EXTENDED,
            Profile::H264(H264::High) => AV_PROFILE_H264_HIGH,
            Profile::H264(H264::High10) => AV_PROFILE_H264_HIGH_10,
            Profile::H264(H264::High10Intra) => AV_PROFILE_H264_HIGH_10_INTRA,
            Profile::H264(H264::High422) => AV_PROFILE_H264_HIGH_422,
            Profile::H264(H264::High422Intra) => AV_PROFILE_H264_HIGH_422_INTRA,
            Profile::H264(H264::High444) => AV_PROFILE_H264_HIGH_444,
            Profile::H264(H264::High444Predictive) => AV_PROFILE_H264_HIGH_444_PREDICTIVE,
            Profile::H264(H264::High444Intra) => AV_PROFILE_H264_HIGH_444_INTRA,
            Profile::H264(H264::CAVLC444) => AV_PROFILE_H264_CAVLC_444,

            Profile::VC1(VC1::Simple) => AV_PROFILE_VC1_SIMPLE,
            Profile::VC1(VC1::Main) => AV_PROFILE_VC1_MAIN,
            Profile::VC1(VC1::Complex) => AV_PROFILE_VC1_COMPLEX,
            Profile::VC1(VC1::Advanced) => AV_PROFILE_VC1_ADVANCED,

            Profile::MPEG4(MPEG4::Simple) => AV_PROFILE_MPEG4_SIMPLE,
            Profile::MPEG4(MPEG4::SimpleScalable) => AV_PROFILE_MPEG4_SIMPLE_SCALABLE,
            Profile::MPEG4(MPEG4::Core) => AV_PROFILE_MPEG4_CORE,
            Profile::MPEG4(MPEG4::Main) => AV_PROFILE_MPEG4_MAIN,
            Profile::MPEG4(MPEG4::NBit) => AV_PROFILE_MPEG4_N_BIT,
            Profile::MPEG4(MPEG4::ScalableTexture) => AV_PROFILE_MPEG4_SCALABLE_TEXTURE,
            Profile::MPEG4(MPEG4::SimpleFaceAnimation) => AV_PROFILE_MPEG4_SIMPLE_FACE_ANIMATION,
            Profile::MPEG4(MPEG4::BasicAnimatedTexture) => AV_PROFILE_MPEG4_BASIC_ANIMATED_TEXTURE,
            Profile::MPEG4(MPEG4::Hybrid) => AV_PROFILE_MPEG4_HYBRID,
            Profile::MPEG4(MPEG4::AdvancedRealTime) => AV_PROFILE_MPEG4_ADVANCED_REAL_TIME,
            Profile::MPEG4(MPEG4::CoreScalable) => AV_PROFILE_MPEG4_CORE_SCALABLE,
            Profile::MPEG4(MPEG4::AdvancedCoding) => AV_PROFILE_MPEG4_ADVANCED_CODING,
            Profile::MPEG4(MPEG4::AdvancedCore) => AV_PROFILE_MPEG4_ADVANCED_CORE,
            Profile::MPEG4(MPEG4::AdvancedScalableTexture) => {
                AV_PROFILE_MPEG4_ADVANCED_SCALABLE_TEXTURE
            }
            Profile::MPEG4(MPEG4::SimpleStudio) => AV_PROFILE_MPEG4_SIMPLE_STUDIO,
            Profile::MPEG4(MPEG4::AdvancedSimple) => AV_PROFILE_MPEG4_ADVANCED_SIMPLE,

            Profile::JPEG2000(JPEG2000::CStreamRestriction0) => {
                AV_PROFILE_JPEG2000_CSTREAM_RESTRICTION_0
            }
            Profile::JPEG2000(JPEG2000::CStreamRestriction1) => {
                AV_PROFILE_JPEG2000_CSTREAM_RESTRICTION_1
            }
            Profile::JPEG2000(JPEG2000::CStreamNoRestriction) => {
                AV_PROFILE_JPEG2000_CSTREAM_NO_RESTRICTION
            }
            Profile::JPEG2000(JPEG2000::DCinema2K) => AV_PROFILE_JPEG2000_DCINEMA_2K,
            Profile::JPEG2000(JPEG2000::DCinema4K) => AV_PROFILE_JPEG2000_DCINEMA_4K,

            Profile::VP9(VP9::_0) => AV_PROFILE_VP9_0,
            Profile::VP9(VP9::_1) => AV_PROFILE_VP9_1,
            Profile::VP9(VP9::_2) => AV_PROFILE_VP9_2,
            Profile::VP9(VP9::_3) => AV_PROFILE_VP9_3,

            Profile::HEVC(HEVC::Main) => AV_PROFILE_HEVC_MAIN,
            Profile::HEVC(HEVC::Main10) => AV_PROFILE_HEVC_MAIN_10,
            Profile::HEVC(HEVC::MainStillPicture) => AV_PROFILE_HEVC_MAIN_STILL_PICTURE,
            Profile::HEVC(HEVC::Rext) => AV_PROFILE_HEVC_REXT,

            Profile::VVC(VVC::Main10) => AV_PROFILE_VVC_MAIN_10,
            Profile::VVC(VVC::Main10_444) => AV_PROFILE_VVC_MAIN_10_444,

            Profile::AV1(AV1::Main) => AV_PROFILE_AV1_MAIN,
            Profile::AV1(AV1::High) => AV_PROFILE_AV1_HIGH,
            Profile::AV1(AV1::Professional) => AV_PROFILE_AV1_PROFESSIONAL,

            Profile::MJPEG(MJPEG::HuffmanBaselineDCT) => AV_PROFILE_MJPEG_HUFFMAN_BASELINE_DCT,
            Profile::MJPEG(MJPEG::HuffmanExtendedSequentialDCT) => {
                AV_PROFILE_MJPEG_HUFFMAN_EXTENDED_SEQUENTIAL_DCT
            }
            Profile::MJPEG(MJPEG::HuffmanProgressiveDCT) => {
                AV_PROFILE_MJPEG_HUFFMAN_PROGRESSIVE_DCT
            }
            Profile::MJPEG(MJPEG::HuffmanLossless) => AV_PROFILE_MJPEG_HUFFMAN_LOSSLESS,
            Profile::MJPEG(MJPEG::JPEG_LS) => AV_PROFILE_MJPEG_JPEG_LS,

            Profile::SBC_MSBC => AV_PROFILE_SBC_MSBC,

            Profile::ProRes(ProRes::Proxy) => AV_PROFILE_PRORES_PROXY,
            Profile::ProRes(ProRes::LT) => AV_PROFILE_PRORES_LT,
            Profile::ProRes(ProRes::Standard) => AV_PROFILE_PRORES_STANDARD,
            Profile::ProRes(ProRes::HQ) => AV_PROFILE_PRORES_HQ,
            Profile::ProRes(ProRes::_4444) => AV_PROFILE_PRORES_4444,
            Profile::ProRes(ProRes::XQ) => AV_PROFILE_PRORES_XQ,

            #[cfg(feature = "ffmpeg_8_0")]
            Profile::ProResRAW(ProResRAW::Default) => AV_PROFILE_PRORES_RAW,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::ProResRAW(ProResRAW::HQ) => AV_PROFILE_PRORES_RAW_HQ,

            Profile::ARIB(ARIB::ProfileA) => AV_PROFILE_ARIB_PROFILE_A,
            Profile::ARIB(ARIB::ProfileC) => AV_PROFILE_ARIB_PROFILE_C,

            Profile::KLVA(KLVA::Sync) => AV_PROFILE_KLVA_SYNC,
            Profile::KLVA(KLVA::Async) => AV_PROFILE_KLVA_ASYNC,

            Profile::EVC(EVC::Baseline) => AV_PROFILE_EVC_BASELINE,
            Profile::EVC(EVC::Main) => AV_PROFILE_EVC_MAIN,

            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_422_10) => AV_PROFILE_APV_422_10,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_422_12) => AV_PROFILE_APV_422_12,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_444_10) => AV_PROFILE_APV_444_10,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_444_12) => AV_PROFILE_APV_444_12,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_4444_10) => AV_PROFILE_APV_4444_10,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_4444_12) => AV_PROFILE_APV_4444_12,
            #[cfg(feature = "ffmpeg_8_0")]
            Profile::APV(APV::_400_10) => AV_PROFILE_APV_400_10,
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
            if (*self.ptr).profile == AV_PROFILE_UNKNOWN {
                return None;
            }

            let profile = Profile::from((self.id, (*self.ptr).profile));
            self.ptr = self.ptr.offset(1);

            Some(profile)
        }
    }
}

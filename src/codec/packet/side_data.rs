use std::marker::PhantomData;
use std::slice;

use super::Packet;
use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    Palette,
    NewExtraData,
    ParamChange,
    H263MbInfo,
    ReplayGain,
    DisplayMatrix,
    Stereo3d,
    AudioServiceType,
    QualityStats,
    FallbackTrack,
    CBPProperties,
    SkipSamples,
    JpDualMono,
    StringsMetadata,
    SubtitlePosition,
    MatroskaBlockAdditional,
    WebVTTIdentifier,
    WebVTTSettings,
    MetadataUpdate,
    MPEGTSStreamID,
    MasteringDisplayMetadata,
    DataSpherical,
    DataNb,

    ContentLightLevel,
    A53CC,

    EncryptionInitInfo,
    EncryptionInfo,

    AFD,

    #[cfg(feature = "ffmpeg_4_3")]
    PRFT,
    #[cfg(feature = "ffmpeg_4_3")]
    ICC_PROFILE,
    #[cfg(feature = "ffmpeg_4_3")]
    DOVI_CONF,

    #[cfg(feature = "ffmpeg_4_4")]
    S12M_TIMECODE,

    #[cfg(feature = "ffmpeg_5_0")]
    DYNAMIC_HDR10_PLUS,

    #[cfg(feature = "ffmpeg_7_0")]
    IAMF_MIX_GAIN_PARAM,
    #[cfg(feature = "ffmpeg_7_0")]
    IAMF_DEMIXING_INFO_PARAM,
    #[cfg(feature = "ffmpeg_7_0")]
    IAMF_RECON_GAIN_INFO_PARAM,
    #[cfg(feature = "ffmpeg_7_0")]
    AMBIENT_VIEWING_ENVIRONMENT,

    #[cfg(feature = "ffmpeg_7_1")]
    FrameCropping,
    #[cfg(feature = "ffmpeg_7_1")]
    LCEVC,

    #[cfg(feature = "ffmpeg_8_0")]
    _3DReferenceDisplays,
    #[cfg(feature = "ffmpeg_8_0")]
    RTCP_SR,
}

impl From<AVPacketSideDataType> for Type {
    fn from(value: AVPacketSideDataType) -> Self {
        use AVPacketSideDataType as AV;

        match value {
            AV::PALETTE => Type::Palette,
            AV::NEW_EXTRADATA => Type::NewExtraData,
            AV::PARAM_CHANGE => Type::ParamChange,
            AV::H263_MB_INFO => Type::H263MbInfo,
            AV::REPLAYGAIN => Type::ReplayGain,
            AV::DISPLAYMATRIX => Type::DisplayMatrix,
            AV::STEREO3D => Type::Stereo3d,
            AV::AUDIO_SERVICE_TYPE => Type::AudioServiceType,
            AV::QUALITY_STATS => Type::QualityStats,
            AV::FALLBACK_TRACK => Type::FallbackTrack,
            AV::CPB_PROPERTIES => Type::CBPProperties,
            AV::SKIP_SAMPLES => Type::SkipSamples,
            AV::JP_DUALMONO => Type::JpDualMono,
            AV::STRINGS_METADATA => Type::StringsMetadata,
            AV::SUBTITLE_POSITION => Type::SubtitlePosition,
            AV::MATROSKA_BLOCKADDITIONAL => Type::MatroskaBlockAdditional,
            AV::WEBVTT_IDENTIFIER => Type::WebVTTIdentifier,
            AV::WEBVTT_SETTINGS => Type::WebVTTSettings,
            AV::METADATA_UPDATE => Type::MetadataUpdate,
            AV::MPEGTS_STREAM_ID => Type::MPEGTSStreamID,
            AV::MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
            AV::SPHERICAL => Type::DataSpherical,
            AV::NB => Type::DataNb,

            AV::CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
            AV::A53_CC => Type::A53CC,

            AV::ENCRYPTION_INIT_INFO => Type::EncryptionInitInfo,
            AV::ENCRYPTION_INFO => Type::EncryptionInfo,

            AV::AFD => Type::AFD,

            #[cfg(feature = "ffmpeg_4_3")]
            AV::PRFT => Type::PRFT,
            #[cfg(feature = "ffmpeg_4_3")]
            AV::ICC_PROFILE => Type::ICC_PROFILE,
            #[cfg(feature = "ffmpeg_4_3")]
            AV::DOVI_CONF => Type::DOVI_CONF,

            #[cfg(feature = "ffmpeg_4_4")]
            AV::S12M_TIMECODE => Type::S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_5_0")]
            AV::DYNAMIC_HDR10_PLUS => Type::DYNAMIC_HDR10_PLUS,

            #[cfg(feature = "ffmpeg_7_0")]
            AV::IAMF_MIX_GAIN_PARAM => Type::IAMF_MIX_GAIN_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            AV::IAMF_DEMIXING_INFO_PARAM => Type::IAMF_DEMIXING_INFO_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            AV::IAMF_RECON_GAIN_INFO_PARAM => Type::IAMF_RECON_GAIN_INFO_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            AV::AMBIENT_VIEWING_ENVIRONMENT => Type::AMBIENT_VIEWING_ENVIRONMENT,

            #[cfg(feature = "ffmpeg_7_1")]
            AV::FRAME_CROPPING => Type::FrameCropping,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::LCEVC => Type::LCEVC,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::_3D_REFERENCE_DISPLAYS => Type::_3DReferenceDisplays,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::RTCP_SR => Type::RTCP_SR,

            _ => unimplemented!(),
        }
    }
}

impl From<Type> for AVPacketSideDataType {
    fn from(value: Type) -> AVPacketSideDataType {
        use AVPacketSideDataType as AV;

        match value {
            Type::Palette => AV::PALETTE,
            Type::NewExtraData => AV::NEW_EXTRADATA,
            Type::ParamChange => AV::PARAM_CHANGE,
            Type::H263MbInfo => AV::H263_MB_INFO,
            Type::ReplayGain => AV::REPLAYGAIN,
            Type::DisplayMatrix => AV::DISPLAYMATRIX,
            Type::Stereo3d => AV::STEREO3D,
            Type::AudioServiceType => AV::AUDIO_SERVICE_TYPE,
            Type::QualityStats => AV::QUALITY_STATS,
            Type::FallbackTrack => AV::FALLBACK_TRACK,
            Type::CBPProperties => AV::CPB_PROPERTIES,
            Type::SkipSamples => AV::SKIP_SAMPLES,
            Type::JpDualMono => AV::JP_DUALMONO,
            Type::StringsMetadata => AV::STRINGS_METADATA,
            Type::SubtitlePosition => AV::SUBTITLE_POSITION,
            Type::MatroskaBlockAdditional => AV::MATROSKA_BLOCKADDITIONAL,
            Type::WebVTTIdentifier => AV::WEBVTT_IDENTIFIER,
            Type::WebVTTSettings => AV::WEBVTT_SETTINGS,
            Type::MetadataUpdate => AV::METADATA_UPDATE,
            Type::MPEGTSStreamID => AV::MPEGTS_STREAM_ID,
            Type::MasteringDisplayMetadata => AV::MASTERING_DISPLAY_METADATA,
            Type::DataSpherical => AV::SPHERICAL,
            Type::DataNb => AV::NB,

            Type::ContentLightLevel => AV::CONTENT_LIGHT_LEVEL,
            Type::A53CC => AV::A53_CC,

            Type::EncryptionInitInfo => AV::ENCRYPTION_INIT_INFO,
            Type::EncryptionInfo => AV::ENCRYPTION_INFO,

            Type::AFD => AV::AFD,

            #[cfg(feature = "ffmpeg_4_3")]
            Type::PRFT => AV::PRFT,
            #[cfg(feature = "ffmpeg_4_3")]
            Type::ICC_PROFILE => AV::ICC_PROFILE,
            #[cfg(feature = "ffmpeg_4_3")]
            Type::DOVI_CONF => AV::DOVI_CONF,

            #[cfg(feature = "ffmpeg_4_4")]
            Type::S12M_TIMECODE => AV::S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_5_0")]
            Type::DYNAMIC_HDR10_PLUS => AV::DYNAMIC_HDR10_PLUS,

            #[cfg(feature = "ffmpeg_7_0")]
            Type::IAMF_MIX_GAIN_PARAM => AV::IAMF_MIX_GAIN_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            Type::IAMF_DEMIXING_INFO_PARAM => AV::IAMF_DEMIXING_INFO_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            Type::IAMF_RECON_GAIN_INFO_PARAM => AV::IAMF_RECON_GAIN_INFO_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            Type::AMBIENT_VIEWING_ENVIRONMENT => AV::AMBIENT_VIEWING_ENVIRONMENT,

            #[cfg(feature = "ffmpeg_7_1")]
            Type::FrameCropping => AV::FRAME_CROPPING,
            #[cfg(feature = "ffmpeg_7_1")]
            Type::LCEVC => AV::LCEVC,

            #[cfg(feature = "ffmpeg_8_0")]
            Type::_3DReferenceDisplays => AV::_3D_REFERENCE_DISPLAYS,
            #[cfg(feature = "ffmpeg_8_0")]
            Type::RTCP_SR => AV::RTCP_SR,
        }
    }
}

pub struct SideData<'a> {
    ptr: *mut AVPacketSideData,

    _marker: PhantomData<&'a Packet>,
}

impl<'a> SideData<'a> {
    pub unsafe fn wrap(ptr: *mut AVPacketSideData) -> Self {
        SideData {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVPacketSideData {
        self.ptr as *const _
    }
}

impl<'a> SideData<'a> {
    pub fn kind(&self) -> Type {
        unsafe { Type::from((*self.as_ptr()).type_) }
    }

    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize) }
    }
}

use std::marker::PhantomData;
use std::slice;

use super::Frame;
use crate::ffi::*;
use crate::utils;
use crate::DictionaryRef;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    PanScan,
    A53CC,
    Stereo3D,
    MatrixEncoding,
    DownMixInfo,
    ReplayGain,
    DisplayMatrix,
    AFD,
    MotionVectors,
    SkipSamples,
    AudioServiceType,
    MasteringDisplayMetadata,
    GOPTimecode,
    Spherical,

    ContentLightLevel,
    IccProfile,

    #[cfg(not(feature = "ffmpeg_5_0"))]
    QPTableProperties,
    #[cfg(not(feature = "ffmpeg_5_0"))]
    QPTableData,

    S12M_TIMECODE,

    DYNAMIC_HDR_PLUS,
    REGIONS_OF_INTEREST,

    #[cfg(feature = "ffmpeg_4_3")]
    VIDEO_ENC_PARAMS,

    #[cfg(feature = "ffmpeg_4_4")]
    SEI_UNREGISTERED,
    #[cfg(feature = "ffmpeg_4_4")]
    FILM_GRAIN_PARAMS,

    #[cfg(feature = "ffmpeg_5_0")]
    DETECTION_BBOXES,
    #[cfg(feature = "ffmpeg_5_0")]
    DOVI_RPU_BUFFER,
    #[cfg(feature = "ffmpeg_5_0")]
    DOVI_METADATA,

    #[cfg(feature = "ffmpeg_5_1")]
    DYNAMIC_HDR_VIVID,

    #[cfg(feature = "ffmpeg_6_0")]
    AMBIENT_VIEWING_ENVIRONMENT,

    #[cfg(feature = "ffmpeg_6_1")]
    VIDEO_HINT,

    #[cfg(feature = "ffmpeg_7_1")]
    LCEVC,
    #[cfg(feature = "ffmpeg_7_1")]
    ViewId,

    #[cfg(feature = "ffmpeg_8_0")]
    _3DReferenceDisplays,
}

impl Type {
    #[inline]
    pub fn name(&self) -> &'static str {
        unsafe { utils::str_from_c_ptr(av_frame_side_data_name((*self).into())) }
    }
}

impl From<AVFrameSideDataType> for Type {
    #[inline(always)]
    fn from(value: AVFrameSideDataType) -> Self {
        use AVFrameSideDataType as AV;

        match value {
            AV::PANSCAN => Type::PanScan,
            AV::A53_CC => Type::A53CC,
            AV::STEREO3D => Type::Stereo3D,
            AV::MATRIXENCODING => Type::MatrixEncoding,
            AV::DOWNMIX_INFO => Type::DownMixInfo,
            AV::REPLAYGAIN => Type::ReplayGain,
            AV::DISPLAYMATRIX => Type::DisplayMatrix,
            AV::AFD => Type::AFD,
            AV::MOTION_VECTORS => Type::MotionVectors,
            AV::SKIP_SAMPLES => Type::SkipSamples,
            AV::AUDIO_SERVICE_TYPE => Type::AudioServiceType,
            AV::MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
            AV::GOP_TIMECODE => Type::GOPTimecode,
            AV::SPHERICAL => Type::Spherical,

            AV::CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
            AV::ICC_PROFILE => Type::IccProfile,

            #[cfg(not(feature = "ffmpeg_5_0"))]
            AV::QP_TABLE_PROPERTIES => Type::QPTableProperties,
            #[cfg(not(feature = "ffmpeg_5_0"))]
            AV::QP_TABLE_DATA => Type::QPTableData,
            AV::S12M_TIMECODE => Type::S12M_TIMECODE,

            AV::DYNAMIC_HDR_PLUS => Type::DYNAMIC_HDR_PLUS,
            AV::REGIONS_OF_INTEREST => Type::REGIONS_OF_INTEREST,

            #[cfg(feature = "ffmpeg_4_3")]
            AV::VIDEO_ENC_PARAMS => Type::VIDEO_ENC_PARAMS,

            #[cfg(feature = "ffmpeg_4_4")]
            AV::SEI_UNREGISTERED => Type::SEI_UNREGISTERED,
            #[cfg(feature = "ffmpeg_4_4")]
            AV::FILM_GRAIN_PARAMS => Type::FILM_GRAIN_PARAMS,

            #[cfg(feature = "ffmpeg_5_0")]
            AV::DETECTION_BBOXES => Type::DETECTION_BBOXES,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::DOVI_RPU_BUFFER => Type::DOVI_RPU_BUFFER,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::DOVI_METADATA => Type::DOVI_METADATA,

            #[cfg(feature = "ffmpeg_5_1")]
            AV::DYNAMIC_HDR_VIVID => Type::DYNAMIC_HDR_VIVID,

            #[cfg(feature = "ffmpeg_6_0")]
            AV::AMBIENT_VIEWING_ENVIRONMENT => Type::AMBIENT_VIEWING_ENVIRONMENT,

            #[cfg(feature = "ffmpeg_6_1")]
            AV::VIDEO_HINT => Type::VIDEO_HINT,

            #[cfg(feature = "ffmpeg_7_1")]
            AV::LCEVC => Type::LCEVC,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::VIEW_ID => Type::ViewId,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::_3D_REFERENCE_DISPLAYS => Type::_3DReferenceDisplays,

            _ => unimplemented!(),
        }
    }
}

impl From<Type> for AVFrameSideDataType {
    #[inline(always)]
    fn from(value: Type) -> AVFrameSideDataType {
        use AVFrameSideDataType as AV;

        match value {
            Type::PanScan => AV::PANSCAN,
            Type::A53CC => AV::A53_CC,
            Type::Stereo3D => AV::STEREO3D,
            Type::MatrixEncoding => AV::MATRIXENCODING,
            Type::DownMixInfo => AV::DOWNMIX_INFO,
            Type::ReplayGain => AV::REPLAYGAIN,
            Type::DisplayMatrix => AV::DISPLAYMATRIX,
            Type::AFD => AV::AFD,
            Type::MotionVectors => AV::MOTION_VECTORS,
            Type::SkipSamples => AV::SKIP_SAMPLES,
            Type::AudioServiceType => AV::AUDIO_SERVICE_TYPE,
            Type::MasteringDisplayMetadata => AV::MASTERING_DISPLAY_METADATA,
            Type::GOPTimecode => AV::GOP_TIMECODE,
            Type::Spherical => AV::SPHERICAL,

            Type::ContentLightLevel => AV::CONTENT_LIGHT_LEVEL,
            Type::IccProfile => AV::ICC_PROFILE,

            #[cfg(not(feature = "ffmpeg_5_0"))]
            Type::QPTableProperties => AV::QP_TABLE_PROPERTIES,
            #[cfg(not(feature = "ffmpeg_5_0"))]
            Type::QPTableData => AV::QP_TABLE_DATA,
            Type::S12M_TIMECODE => AV::S12M_TIMECODE,

            Type::DYNAMIC_HDR_PLUS => AV::DYNAMIC_HDR_PLUS,
            Type::REGIONS_OF_INTEREST => AV::REGIONS_OF_INTEREST,

            #[cfg(feature = "ffmpeg_4_3")]
            Type::VIDEO_ENC_PARAMS => AV::VIDEO_ENC_PARAMS,

            #[cfg(feature = "ffmpeg_4_4")]
            Type::SEI_UNREGISTERED => AV::SEI_UNREGISTERED,
            #[cfg(feature = "ffmpeg_4_4")]
            Type::FILM_GRAIN_PARAMS => AV::FILM_GRAIN_PARAMS,

            #[cfg(feature = "ffmpeg_5_0")]
            Type::DETECTION_BBOXES => AV::DETECTION_BBOXES,
            #[cfg(feature = "ffmpeg_5_0")]
            Type::DOVI_RPU_BUFFER => AV::DOVI_RPU_BUFFER,
            #[cfg(feature = "ffmpeg_5_0")]
            Type::DOVI_METADATA => AV::DOVI_METADATA,

            #[cfg(feature = "ffmpeg_5_1")]
            Type::DYNAMIC_HDR_VIVID => AV::DYNAMIC_HDR_VIVID,

            #[cfg(feature = "ffmpeg_6_0")]
            Type::AMBIENT_VIEWING_ENVIRONMENT => AV::AMBIENT_VIEWING_ENVIRONMENT,

            #[cfg(feature = "ffmpeg_6_1")]
            Type::VIDEO_HINT => AV::VIDEO_HINT,

            #[cfg(feature = "ffmpeg_7_1")]
            Type::LCEVC => AV::LCEVC,
            #[cfg(feature = "ffmpeg_7_1")]
            Type::ViewId => AV::VIEW_ID,

            #[cfg(feature = "ffmpeg_8_0")]
            Type::_3DReferenceDisplays => AV::_3D_REFERENCE_DISPLAYS,
        }
    }
}

pub struct SideData<'a> {
    ptr: *mut AVFrameSideData,

    _marker: PhantomData<&'a Frame>,
}

impl<'a> SideData<'a> {
    #[inline(always)]
    pub unsafe fn wrap(ptr: *mut AVFrameSideData) -> Self {
        SideData {
            ptr,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const AVFrameSideData {
        self.ptr as *const _
    }

    #[inline(always)]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFrameSideData {
        self.ptr
    }
}

impl<'a> SideData<'a> {
    #[inline]
    pub fn kind(&self) -> Type {
        unsafe { Type::from((*self.as_ptr()).type_) }
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize) }
    }

    #[inline]
    pub fn metadata(&self) -> DictionaryRef<'_> {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }
}

use crate::ffi::*;
use crate::utils;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum TransferCharacteristic {
    Reserved0,
    BT709,
    Unspecified,
    Reserved,
    GAMMA22,
    GAMMA28,
    SMPTE170M,
    SMPTE240M,
    Linear,
    Log,
    LogSqrt,
    IEC61966_2_4,
    BT1361_ECG,
    IEC61966_2_1,
    BT2020_10,
    BT2020_12,
    SMPTE2084,
    SMPTE428,
    ARIB_STD_B67,

    #[cfg(feature = "ffmpeg_8_1")]
    V_LOG,
}

impl TransferCharacteristic {
    pub fn name(&self) -> Option<&'static str> {
        if *self == TransferCharacteristic::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_transfer_name((*self).into());
            utils::optional_str_from_c_ptr(ptr)
        }
    }
}

impl From<AVColorTransferCharacteristic> for TransferCharacteristic {
    fn from(value: AVColorTransferCharacteristic) -> TransferCharacteristic {
        use AVColorTransferCharacteristic as AV;

        match value {
            AV::AVCOL_TRC_RESERVED0 => TransferCharacteristic::Reserved0,
            AV::AVCOL_TRC_BT709 => TransferCharacteristic::BT709,
            AV::AVCOL_TRC_UNSPECIFIED => TransferCharacteristic::Unspecified,
            AV::AVCOL_TRC_RESERVED => TransferCharacteristic::Reserved,
            AV::AVCOL_TRC_GAMMA22 => TransferCharacteristic::GAMMA22,
            AV::AVCOL_TRC_GAMMA28 => TransferCharacteristic::GAMMA28,
            AV::AVCOL_TRC_SMPTE170M => TransferCharacteristic::SMPTE170M,
            AV::AVCOL_TRC_SMPTE240M => TransferCharacteristic::SMPTE240M,
            AV::AVCOL_TRC_LINEAR => TransferCharacteristic::Linear,
            AV::AVCOL_TRC_LOG => TransferCharacteristic::Log,
            AV::AVCOL_TRC_LOG_SQRT => TransferCharacteristic::LogSqrt,
            AV::AVCOL_TRC_IEC61966_2_4 => TransferCharacteristic::IEC61966_2_4,
            AV::AVCOL_TRC_BT1361_ECG => TransferCharacteristic::BT1361_ECG,
            AV::AVCOL_TRC_IEC61966_2_1 => TransferCharacteristic::IEC61966_2_1,
            AV::AVCOL_TRC_BT2020_10 => TransferCharacteristic::BT2020_10,
            AV::AVCOL_TRC_BT2020_12 => TransferCharacteristic::BT2020_12,
            AV::AVCOL_TRC_NB => TransferCharacteristic::Reserved0,
            AV::AVCOL_TRC_SMPTE2084 => TransferCharacteristic::SMPTE2084,
            AV::AVCOL_TRC_SMPTE428 => TransferCharacteristic::SMPTE428,
            AV::AVCOL_TRC_ARIB_STD_B67 => TransferCharacteristic::ARIB_STD_B67,

            #[cfg(feature = "ffmpeg_8_1")]
            // upstream defines it this way
            AV::AVCOL_TRC_EXT_BASE => TransferCharacteristic::V_LOG,
            #[cfg(feature = "ffmpeg_8_1")]
            AV::AVCOL_TRC_EXT_NB => unreachable!(),

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<TransferCharacteristic> for AVColorTransferCharacteristic {
    fn from(value: TransferCharacteristic) -> AVColorTransferCharacteristic {
        use AVColorTransferCharacteristic as AV;

        match value {
            TransferCharacteristic::Reserved0 => AV::AVCOL_TRC_RESERVED0,
            TransferCharacteristic::BT709 => AV::AVCOL_TRC_BT709,
            TransferCharacteristic::Unspecified => AV::AVCOL_TRC_UNSPECIFIED,
            TransferCharacteristic::Reserved => AV::AVCOL_TRC_RESERVED,
            TransferCharacteristic::GAMMA22 => AV::AVCOL_TRC_GAMMA22,
            TransferCharacteristic::GAMMA28 => AV::AVCOL_TRC_GAMMA28,
            TransferCharacteristic::SMPTE170M => AV::AVCOL_TRC_SMPTE170M,
            TransferCharacteristic::SMPTE240M => AV::AVCOL_TRC_SMPTE240M,
            TransferCharacteristic::Linear => AV::AVCOL_TRC_LINEAR,
            TransferCharacteristic::Log => AV::AVCOL_TRC_LOG,
            TransferCharacteristic::LogSqrt => AV::AVCOL_TRC_LOG_SQRT,
            TransferCharacteristic::IEC61966_2_4 => AV::AVCOL_TRC_IEC61966_2_4,
            TransferCharacteristic::BT1361_ECG => AV::AVCOL_TRC_BT1361_ECG,
            TransferCharacteristic::IEC61966_2_1 => AV::AVCOL_TRC_IEC61966_2_1,
            TransferCharacteristic::BT2020_10 => AV::AVCOL_TRC_BT2020_10,
            TransferCharacteristic::BT2020_12 => AV::AVCOL_TRC_BT2020_12,
            TransferCharacteristic::SMPTE2084 => AV::AVCOL_TRC_SMPTE2084,
            TransferCharacteristic::SMPTE428 => AV::AVCOL_TRC_SMPTE428,
            TransferCharacteristic::ARIB_STD_B67 => AV::AVCOL_TRC_ARIB_STD_B67,

            #[cfg(feature = "ffmpeg_8_1")]
            TransferCharacteristic::V_LOG => AV::AVCOL_TRC_V_LOG,
        }
    }
}

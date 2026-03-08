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
            AV::RESERVED0 => TransferCharacteristic::Reserved0,
            AV::BT709 => TransferCharacteristic::BT709,
            AV::UNSPECIFIED => TransferCharacteristic::Unspecified,
            AV::RESERVED => TransferCharacteristic::Reserved,
            AV::GAMMA22 => TransferCharacteristic::GAMMA22,
            AV::GAMMA28 => TransferCharacteristic::GAMMA28,
            AV::SMPTE170M => TransferCharacteristic::SMPTE170M,
            AV::SMPTE240M => TransferCharacteristic::SMPTE240M,
            AV::LINEAR => TransferCharacteristic::Linear,
            AV::LOG => TransferCharacteristic::Log,
            AV::LOG_SQRT => TransferCharacteristic::LogSqrt,
            AV::IEC61966_2_4 => TransferCharacteristic::IEC61966_2_4,
            AV::BT1361_ECG => TransferCharacteristic::BT1361_ECG,
            AV::IEC61966_2_1 => TransferCharacteristic::IEC61966_2_1,
            AV::BT2020_10 => TransferCharacteristic::BT2020_10,
            AV::BT2020_12 => TransferCharacteristic::BT2020_12,
            AV::NB => TransferCharacteristic::Reserved0,
            AV::SMPTE2084 => TransferCharacteristic::SMPTE2084,
            AV::SMPTE428 => TransferCharacteristic::SMPTE428,
            AV::ARIB_STD_B67 => TransferCharacteristic::ARIB_STD_B67,

            _ => unimplemented!(),
        }
    }
}

impl From<TransferCharacteristic> for AVColorTransferCharacteristic {
    fn from(value: TransferCharacteristic) -> AVColorTransferCharacteristic {
        use AVColorTransferCharacteristic as AV;

        match value {
            TransferCharacteristic::Reserved0 => AV::RESERVED0,
            TransferCharacteristic::BT709 => AV::BT709,
            TransferCharacteristic::Unspecified => AV::UNSPECIFIED,
            TransferCharacteristic::Reserved => AV::RESERVED,
            TransferCharacteristic::GAMMA22 => AV::GAMMA22,
            TransferCharacteristic::GAMMA28 => AV::GAMMA28,
            TransferCharacteristic::SMPTE170M => AV::SMPTE170M,
            TransferCharacteristic::SMPTE240M => AV::SMPTE240M,
            TransferCharacteristic::Linear => AV::LINEAR,
            TransferCharacteristic::Log => AV::LOG,
            TransferCharacteristic::LogSqrt => AV::LOG_SQRT,
            TransferCharacteristic::IEC61966_2_4 => AV::IEC61966_2_4,
            TransferCharacteristic::BT1361_ECG => AV::BT1361_ECG,
            TransferCharacteristic::IEC61966_2_1 => AV::IEC61966_2_1,
            TransferCharacteristic::BT2020_10 => AV::BT2020_10,
            TransferCharacteristic::BT2020_12 => AV::BT2020_12,
            TransferCharacteristic::SMPTE2084 => AV::SMPTE2084,
            TransferCharacteristic::SMPTE428 => AV::SMPTE428,
            TransferCharacteristic::ARIB_STD_B67 => AV::ARIB_STD_B67,
        }
    }
}

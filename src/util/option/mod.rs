mod traits;
pub use self::traits::{Gettable, Iterable, Settable, Target};

use crate::ffi::*;
use libc::c_uint;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
    pub struct Type: c_uint {
        const FLAGS             = AVOptionType::AV_OPT_TYPE_FLAGS.0;
        const INT               = AVOptionType::AV_OPT_TYPE_INT.0;
        const INT64             = AVOptionType::AV_OPT_TYPE_INT64.0;
        const DOUBLE            = AVOptionType::AV_OPT_TYPE_DOUBLE.0;
        const FLOAT             = AVOptionType::AV_OPT_TYPE_FLOAT.0;
        const STRING            = AVOptionType::AV_OPT_TYPE_STRING.0;
        const RATIONAL          = AVOptionType::AV_OPT_TYPE_RATIONAL.0;
        const BINARY            = AVOptionType::AV_OPT_TYPE_BINARY.0;
        const DICTIONARY        = AVOptionType::AV_OPT_TYPE_DICT.0;
        const CONSTANT          = AVOptionType::AV_OPT_TYPE_CONST.0;

        const IMAGE_SIZE        = AVOptionType::AV_OPT_TYPE_IMAGE_SIZE.0;
        const PIXEL_FORMAT      = AVOptionType::AV_OPT_TYPE_PIXEL_FMT.0;
        const SAMPLE_FORMAT     = AVOptionType::AV_OPT_TYPE_SAMPLE_FMT.0;
        const VIDEO_RATE        = AVOptionType::AV_OPT_TYPE_VIDEO_RATE.0;
        const DURATION          = AVOptionType::AV_OPT_TYPE_DURATION.0;
        const COLOR             = AVOptionType::AV_OPT_TYPE_COLOR.0;
        #[cfg(not(feature = "ffmpeg_7_0"))]
        const CHANNEL_LAYOUT    = AVOptionType::AV_OPT_TYPE_CHANNEL_LAYOUT.0;
        #[cfg(feature = "ffmpeg_5_1")]
        const CHLAYOUT          = AVOptionType::AV_OPT_TYPE_CHLAYOUT.0;
        const C_ULONG           = AVOptionType::AV_OPT_TYPE_UINT64.0;
        const BOOL              = AVOptionType::AV_OPT_TYPE_BOOL.0;

        #[cfg(feature = "ffmpeg_7_0")]
        const FLAG_ARRAY        = AVOptionType::AV_OPT_TYPE_FLAG_ARRAY.0;
    }
}

impl From<AVOptionType> for Type {
    fn from(value: AVOptionType) -> Self {
        Self::from_bits_retain(value.0)
    }
}

impl From<Type> for AVOptionType {
    fn from(value: Type) -> AVOptionType {
        AVOptionType(value.bits())
    }
}

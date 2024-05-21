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
        const FLAGS             = AVOptionType::AV_OPT_TYPE_FLAGS.0 as c_uint;
        const INT               = AVOptionType::AV_OPT_TYPE_INT.0 as c_uint;
        const INT64             = AVOptionType::AV_OPT_TYPE_INT64.0 as c_uint;
        const DOUBLE            = AVOptionType::AV_OPT_TYPE_DOUBLE.0 as c_uint;
        const FLOAT             = AVOptionType::AV_OPT_TYPE_FLOAT.0 as c_uint;
        const STRING            = AVOptionType::AV_OPT_TYPE_STRING.0 as c_uint;
        const RATIONAL          = AVOptionType::AV_OPT_TYPE_RATIONAL.0 as c_uint;
        /// `offset` must point to a pointer immediately followed by an int
        /// for the length.
        const BINARY            = AVOptionType::AV_OPT_TYPE_BINARY.0 as c_uint;
        const DICTIONARY        = AVOptionType::AV_OPT_TYPE_DICT.0 as c_uint;
        const CONSTANT          = AVOptionType::AV_OPT_TYPE_CONST.0 as c_uint;
        /// `offset` must point to two consecutive ints
        const IMAGE_SIZE        = AVOptionType::AV_OPT_TYPE_IMAGE_SIZE.0 as c_uint;
        const PIXEL_FORMAT      = AVOptionType::AV_OPT_TYPE_PIXEL_FMT.0 as c_uint;
        const SAMPLE_FORMAT     = AVOptionType::AV_OPT_TYPE_SAMPLE_FMT.0 as c_uint;
        /// `offset` must point to AVRational
        const VIDEO_RATE        = AVOptionType::AV_OPT_TYPE_VIDEO_RATE.0 as c_uint;
        const DURATION          = AVOptionType::AV_OPT_TYPE_DURATION.0 as c_uint;
        const COLOR             = AVOptionType::AV_OPT_TYPE_COLOR.0 as c_uint;
        #[cfg(not(feature = "ffmpeg_7_0"))]
        const CHANNEL_LAYOUT    = AVOptionType::AV_OPT_TYPE_CHANNEL_LAYOUT.0 as c_uint;
        #[cfg(feature = "ffmpeg_5_1")]
        const CHLAYOUT          = AVOptionType::AV_OPT_TYPE_CHLAYOUT.0 as c_uint;
        const C_ULONG           = AVOptionType::AV_OPT_TYPE_UINT64.0 as c_uint;
        const BOOL              = AVOptionType::AV_OPT_TYPE_BOOL.0 as c_uint;

        /// May be combined with another regular option type to declare an
        /// array option.
        ///
        /// For array options, `AVOption.offset` should refer to a pointer
        /// corresponding to the option type. The pointer should be immediately
        /// followed by an unsigned int that will store the number of elements
        /// in the array.
        #[cfg(feature = "ffmpeg_7_0")]
        const FLAG_ARRAY        = AVOptionType::AV_OPT_TYPE_FLAG_ARRAY.0 as c_uint;
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<AVOptionType> for Type {
    fn from(value: AVOptionType) -> Self {
        Self::from_bits_retain(value.0 as c_uint)
    }
}

impl From<Type> for AVOptionType {
    fn from(value: Type) -> AVOptionType {
        // cast to whichever type the C enum uses
        AVOptionType(value.bits() as _)
    }
}

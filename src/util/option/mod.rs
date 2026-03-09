mod traits;
pub use self::traits::{Gettable, Iterable, Settable};

use crate::ffi::*;
use libc::c_uint;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
    pub struct Type: c_uint {
        const FLAGS             = AVOptionType::FLAGS.0 as _;
        const INT               = AVOptionType::INT.0 as _;
        const INT64             = AVOptionType::INT64.0 as _;
        const DOUBLE            = AVOptionType::DOUBLE.0 as _;
        const FLOAT             = AVOptionType::FLOAT.0 as _;
        const STRING            = AVOptionType::STRING.0 as _;
        const RATIONAL          = AVOptionType::RATIONAL.0 as _;
        /// `offset` must point to a pointer immediately followed by an int
        /// for the length.
        const BINARY            = AVOptionType::BINARY.0 as _;
        const DICTIONARY        = AVOptionType::DICT.0 as _;
        const CONSTANT          = AVOptionType::CONST.0 as _;
        /// `offset` must point to two consecutive ints
        const IMAGE_SIZE        = AVOptionType::IMAGE_SIZE.0 as _;
        const PIXEL_FORMAT      = AVOptionType::PIXEL_FMT.0 as _;
        const SAMPLE_FORMAT     = AVOptionType::SAMPLE_FMT.0 as _;
        /// `offset` must point to AVRational
        const VIDEO_RATE        = AVOptionType::VIDEO_RATE.0 as _;
        const DURATION          = AVOptionType::DURATION.0 as _;
        const COLOR             = AVOptionType::COLOR.0 as _;
        #[cfg(not(feature = "ffmpeg_7_0"))]
        const CHANNEL_LAYOUT    = AVOptionType::CHANNEL_LAYOUT.0 as _;
        #[cfg(feature = "ffmpeg_5_1")]
        const CHLAYOUT          = AVOptionType::CHLAYOUT.0 as _;
        const C_ULONG           = AVOptionType::UINT64.0 as _;
        const BOOL              = AVOptionType::BOOL.0 as _;

        /// May be combined with another regular option type to declare an
        /// array option.
        ///
        /// For array options, `AVOption.offset` should refer to a pointer
        /// corresponding to the option type. The pointer should be immediately
        /// followed by an unsigned int that will store the number of elements
        /// in the array.
        #[cfg(feature = "ffmpeg_7_0")]
        const FLAG_ARRAY        = AVOptionType::FLAG_ARRAY.0 as _;
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

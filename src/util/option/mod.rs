mod traits;
pub use self::traits::{Gettable, Iterable, Settable, Target};

use crate::ffi::AVOptionType::*;
use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    Flags,
    Int,
    Int64,
    Double,
    Float,
    String,
    Rational,
    Binary,
    Dictionary,
    Constant,

    ImageSize,
    PixelFormat,
    SampleFormat,
    VideoRate,
    Duration,
    Color,
    #[cfg(not(feature = "ffmpeg_7_0"))]
    ChannelLayout,
    #[cfg(feature = "ffmpeg_5_1")]
    ChLayout,
    c_ulong,
    bool,

    // FIXME: This is not supported yet. `enum Type` should become a bitflags struct.
    // FIXME: AVOptionType is also not technically an enum because it may contain
    //        values that are outside the defined enum variants!
    #[cfg(feature = "ffmpeg_7_0")]
    ArrayFlag = 0x10000,
}

impl From<AVOptionType> for Type {
    fn from(value: AVOptionType) -> Self {
        match value {
            AV_OPT_TYPE_FLAGS => Type::Flags,
            AV_OPT_TYPE_INT => Type::Int,
            AV_OPT_TYPE_INT64 => Type::Int64,
            AV_OPT_TYPE_DOUBLE => Type::Double,
            AV_OPT_TYPE_FLOAT => Type::Float,
            AV_OPT_TYPE_STRING => Type::String,
            AV_OPT_TYPE_RATIONAL => Type::Rational,
            AV_OPT_TYPE_BINARY => Type::Binary,
            AV_OPT_TYPE_DICT => Type::Dictionary,
            AV_OPT_TYPE_CONST => Type::Constant,
            AV_OPT_TYPE_UINT64 => Type::c_ulong,
            AV_OPT_TYPE_BOOL => Type::bool,

            AV_OPT_TYPE_IMAGE_SIZE => Type::ImageSize,
            AV_OPT_TYPE_PIXEL_FMT => Type::PixelFormat,
            AV_OPT_TYPE_SAMPLE_FMT => Type::SampleFormat,
            AV_OPT_TYPE_VIDEO_RATE => Type::VideoRate,
            AV_OPT_TYPE_DURATION => Type::Duration,
            AV_OPT_TYPE_COLOR => Type::Color,
            #[cfg(not(feature = "ffmpeg_7_0"))]
            AV_OPT_TYPE_CHANNEL_LAYOUT => Type::ChannelLayout,
            #[cfg(feature = "ffmpeg_5_1")]
            AV_OPT_TYPE_CHLAYOUT => Type::ChLayout,

            #[cfg(feature = "ffmpeg_7_0")]
            AV_OPT_TYPE_FLAG_ARRAY => Type::ArrayFlag,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Type> for AVOptionType {
    fn from(value: Type) -> AVOptionType {
        match value {
            Type::Flags => AV_OPT_TYPE_FLAGS,
            Type::Int => AV_OPT_TYPE_INT,
            Type::Int64 => AV_OPT_TYPE_INT64,
            Type::Double => AV_OPT_TYPE_DOUBLE,
            Type::Float => AV_OPT_TYPE_FLOAT,
            Type::String => AV_OPT_TYPE_STRING,
            Type::Rational => AV_OPT_TYPE_RATIONAL,
            Type::Binary => AV_OPT_TYPE_BINARY,
            Type::Dictionary => AV_OPT_TYPE_DICT,
            Type::Constant => AV_OPT_TYPE_CONST,
            Type::c_ulong => AV_OPT_TYPE_UINT64,
            Type::bool => AV_OPT_TYPE_BOOL,

            Type::ImageSize => AV_OPT_TYPE_IMAGE_SIZE,
            Type::PixelFormat => AV_OPT_TYPE_PIXEL_FMT,
            Type::SampleFormat => AV_OPT_TYPE_SAMPLE_FMT,
            Type::VideoRate => AV_OPT_TYPE_VIDEO_RATE,
            Type::Duration => AV_OPT_TYPE_DURATION,
            Type::Color => AV_OPT_TYPE_COLOR,
            #[cfg(not(feature = "ffmpeg_7_0"))]
            Type::ChannelLayout => AV_OPT_TYPE_CHANNEL_LAYOUT,
            #[cfg(feature = "ffmpeg_5_1")]
            Type::ChLayout => AV_OPT_TYPE_CHLAYOUT,

            #[cfg(feature = "ffmpeg_7_0")]
            Type::ArrayFlag => AV_OPT_TYPE_FLAG_ARRAY,
        }
    }
}

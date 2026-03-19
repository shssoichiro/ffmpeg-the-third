use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    Unknown,
    Video,
    Audio,
    Data,
    Subtitle,
    Attachment,
}

impl From<AVMediaType> for Type {
    #[inline(always)]
    fn from(value: AVMediaType) -> Self {
        use AVMediaType as AV;

        match value {
            AV::UNKNOWN => Type::Unknown,
            AV::VIDEO => Type::Video,
            AV::AUDIO => Type::Audio,
            AV::DATA => Type::Data,
            AV::SUBTITLE => Type::Subtitle,
            AV::ATTACHMENT => Type::Attachment,

            AV::NB => unreachable!(),

            _ => unimplemented!(),
        }
    }
}

impl From<Type> for AVMediaType {
    #[inline(always)]
    fn from(value: Type) -> AVMediaType {
        use AVMediaType as AV;

        match value {
            Type::Unknown => AV::UNKNOWN,
            Type::Video => AV::VIDEO,
            Type::Audio => AV::AUDIO,
            Type::Data => AV::DATA,
            Type::Subtitle => AV::SUBTITLE,
            Type::Attachment => AV::ATTACHMENT,
        }
    }
}

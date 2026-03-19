use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    None,
    I,
    P,
    B,
    S,
    SI,
    SP,
    BI,
}

impl From<AVPictureType> for Type {
    #[inline(always)]
    fn from(value: AVPictureType) -> Type {
        use AVPictureType as AV;

        match value {
            AV::NONE => Type::None,
            AV::I => Type::I,
            AV::P => Type::P,
            AV::B => Type::B,
            AV::S => Type::S,
            AV::SI => Type::SI,
            AV::SP => Type::SP,
            AV::BI => Type::BI,

            _ => unimplemented!(),
        }
    }
}

impl From<Type> for AVPictureType {
    #[inline(always)]
    fn from(value: Type) -> AVPictureType {
        use AVPictureType as AV;

        match value {
            Type::None => AV::NONE,
            Type::I => AV::I,
            Type::P => AV::P,
            Type::B => AV::B,
            Type::S => AV::S,
            Type::SI => AV::SI,
            Type::SP => AV::SP,
            Type::BI => AV::BI,
        }
    }
}

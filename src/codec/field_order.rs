use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum FieldOrder {
    Unknown,
    Progressive,
    TT,
    BB,
    TB,
    BT,
}

impl From<AVFieldOrder> for FieldOrder {
    fn from(value: AVFieldOrder) -> Self {
        use AVFieldOrder as AV;

        match value {
            AV::UNKNOWN => FieldOrder::Unknown,
            AV::PROGRESSIVE => FieldOrder::Progressive,
            AV::TT => FieldOrder::TT,
            AV::BB => FieldOrder::BB,
            AV::TB => FieldOrder::TB,
            AV::BT => FieldOrder::BT,

            _ => unimplemented!(),
        }
    }
}

impl From<FieldOrder> for AVFieldOrder {
    fn from(value: FieldOrder) -> AVFieldOrder {
        use AVFieldOrder as AV;

        match value {
            FieldOrder::Unknown => AV::UNKNOWN,
            FieldOrder::Progressive => AV::PROGRESSIVE,
            FieldOrder::TT => AV::TT,
            FieldOrder::BB => AV::BB,
            FieldOrder::TB => AV::TB,
            FieldOrder::BT => AV::BT,
        }
    }
}

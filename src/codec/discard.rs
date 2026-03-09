use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Discard {
    None,
    Default,
    NonReference,
    Bidirectional,
    NonIntra,
    NonKey,
    All,
}

impl From<AVDiscard> for Discard {
    fn from(value: AVDiscard) -> Self {
        use AVDiscard as AV;

        match value {
            AV::NONE => Discard::None,
            AV::DEFAULT => Discard::Default,
            AV::NONREF => Discard::NonReference,
            AV::BIDIR => Discard::Bidirectional,
            AV::NONINTRA => Discard::NonIntra,
            AV::NONKEY => Discard::NonKey,
            AV::ALL => Discard::All,

            _ => unimplemented!(),
        }
    }
}

impl From<Discard> for AVDiscard {
    fn from(value: Discard) -> AVDiscard {
        use AVDiscard as AV;

        match value {
            Discard::None => AV::NONE,
            Discard::Default => AV::DEFAULT,
            Discard::NonReference => AV::NONREF,
            Discard::Bidirectional => AV::BIDIR,
            Discard::NonIntra => AV::NONINTRA,
            Discard::NonKey => AV::NONKEY,
            Discard::All => AV::ALL,
        }
    }
}

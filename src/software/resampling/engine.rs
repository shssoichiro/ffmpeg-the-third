use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Engine {
    Software,
    SoundExchange,
}

impl From<SwrEngine> for Engine {
    fn from(value: SwrEngine) -> Engine {
        use SwrEngine as AV;

        match value {
            AV::SWR => Engine::Software,
            AV::SOXR => Engine::SoundExchange,

            AV::NB => unreachable!(),

            _ => unimplemented!(),
        }
    }
}

impl From<Engine> for SwrEngine {
    fn from(value: Engine) -> SwrEngine {
        use SwrEngine as AV;

        match value {
            Engine::Software => AV::SWR,
            Engine::SoundExchange => AV::SOXR,
        }
    }
}

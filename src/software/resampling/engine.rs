use crate::ffi::*;
use crate::sys::SwrEngine::*;
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
        match value {
            SWR_ENGINE_SWR => Engine::Software,
            SWR_ENGINE_SOXR => Engine::SoundExchange,
            SWR_ENGINE_NB => Engine::Software,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Engine> for SwrEngine {
    fn from(value: Engine) -> SwrEngine {
        match value {
            Engine::Software => SWR_ENGINE_SWR,
            Engine::SoundExchange => SWR_ENGINE_SOXR,
        }
    }
}

use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Rounding {
    Zero,
    Infinity,
    Down,
    Up,
    NearInfinity,
    PassMinMax,
}

impl From<AVRounding> for Rounding {
    #[inline(always)]
    fn from(value: AVRounding) -> Self {
        use AVRounding as AV;

        match value {
            AV::ZERO => Rounding::Zero,
            AV::INF => Rounding::Infinity,
            AV::DOWN => Rounding::Down,
            AV::UP => Rounding::Up,
            AV::NEAR_INF => Rounding::NearInfinity,
            AV::PASS_MINMAX => Rounding::PassMinMax,

            _ => unimplemented!(),
        }
    }
}

impl From<Rounding> for AVRounding {
    #[inline(always)]
    fn from(value: Rounding) -> AVRounding {
        use AVRounding as AV;

        match value {
            Rounding::Zero => AV::ZERO,
            Rounding::Infinity => AV::INF,
            Rounding::Down => AV::DOWN,
            Rounding::Up => AV::UP,
            Rounding::NearInfinity => AV::NEAR_INF,
            Rounding::PassMinMax => AV::PASS_MINMAX,
        }
    }
}

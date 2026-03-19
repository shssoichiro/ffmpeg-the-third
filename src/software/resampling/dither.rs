use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Dither {
    None,
    Rectangular,
    Triangular,
    TriangularHighPass,

    NoiseShapingLipshitz,
    NoiseShapingFWeighted,
    NoiseShapingModifiedEWeighted,
    NoiseShapingImprovedEWeighted,
    NoiseShapingShibata,
    NoiseShapingLowShibata,
    NoiseShapingHighShibata,
}

impl From<SwrDitherType> for Dither {
    fn from(value: SwrDitherType) -> Dither {
        use SwrDitherType as AV;

        match value {
            AV::NONE => Dither::None,
            AV::RECTANGULAR => Dither::Rectangular,
            AV::TRIANGULAR => Dither::Triangular,
            AV::TRIANGULAR_HIGHPASS => Dither::TriangularHighPass,

            AV::NS => Dither::None,
            AV::NS_LIPSHITZ => Dither::NoiseShapingLipshitz,
            AV::NS_F_WEIGHTED => Dither::NoiseShapingFWeighted,
            AV::NS_MODIFIED_E_WEIGHTED => Dither::NoiseShapingModifiedEWeighted,
            AV::NS_IMPROVED_E_WEIGHTED => Dither::NoiseShapingImprovedEWeighted,
            AV::NS_SHIBATA => Dither::NoiseShapingShibata,
            AV::NS_LOW_SHIBATA => Dither::NoiseShapingLowShibata,
            AV::NS_HIGH_SHIBATA => Dither::NoiseShapingHighShibata,

            AV::NB => unreachable!(),

            _ => unimplemented!(),
        }
    }
}

impl From<Dither> for SwrDitherType {
    fn from(value: Dither) -> SwrDitherType {
        use SwrDitherType as AV;

        match value {
            Dither::None => AV::NONE,
            Dither::Rectangular => AV::RECTANGULAR,
            Dither::Triangular => AV::TRIANGULAR,
            Dither::TriangularHighPass => AV::TRIANGULAR_HIGHPASS,

            Dither::NoiseShapingLipshitz => AV::NS_LIPSHITZ,
            Dither::NoiseShapingFWeighted => AV::NS_F_WEIGHTED,
            Dither::NoiseShapingModifiedEWeighted => AV::NS_MODIFIED_E_WEIGHTED,
            Dither::NoiseShapingImprovedEWeighted => AV::NS_IMPROVED_E_WEIGHTED,
            Dither::NoiseShapingShibata => AV::NS_SHIBATA,
            Dither::NoiseShapingLowShibata => AV::NS_LOW_SHIBATA,
            Dither::NoiseShapingHighShibata => AV::NS_HIGH_SHIBATA,
        }
    }
}

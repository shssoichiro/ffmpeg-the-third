use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Filter {
    Cubic,
    BlackmanNuttall,
    Kaiser,
}

impl From<SwrFilterType> for Filter {
    fn from(value: SwrFilterType) -> Filter {
        use SwrFilterType as AV;

        match value {
            AV::CUBIC => Filter::Cubic,
            AV::BLACKMAN_NUTTALL => Filter::BlackmanNuttall,
            AV::KAISER => Filter::Kaiser,

            _ => unimplemented!(),
        }
    }
}

impl From<Filter> for SwrFilterType {
    fn from(value: Filter) -> SwrFilterType {
        use SwrFilterType as AV;

        match value {
            Filter::Cubic => AV::CUBIC,
            Filter::BlackmanNuttall => AV::BLACKMAN_NUTTALL,
            Filter::Kaiser => AV::KAISER,
        }
    }
}

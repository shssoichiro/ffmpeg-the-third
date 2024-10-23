use crate::ffi::AVColorRange::*;
use crate::ffi::*;
use crate::utils;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Range {
    Unspecified,
    MPEG,
    JPEG,
}

impl Range {
    pub fn name(&self) -> Option<&'static str> {
        if *self == Range::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_range_name((*self).into());
            utils::optional_str_from_c_ptr(ptr)
        }
    }
}

impl From<AVColorRange> for Range {
    fn from(value: AVColorRange) -> Self {
        match value {
            AVCOL_RANGE_UNSPECIFIED => Range::Unspecified,
            AVCOL_RANGE_MPEG => Range::MPEG,
            AVCOL_RANGE_JPEG => Range::JPEG,
            AVCOL_RANGE_NB => Range::Unspecified,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Range> for AVColorRange {
    fn from(value: Range) -> AVColorRange {
        match value {
            Range::Unspecified => AVCOL_RANGE_UNSPECIFIED,
            Range::MPEG => AVCOL_RANGE_MPEG,
            Range::JPEG => AVCOL_RANGE_JPEG,
        }
    }
}

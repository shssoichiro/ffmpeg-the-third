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
        use AVColorRange as AV;

        match value {
            AV::UNSPECIFIED => Range::Unspecified,
            AV::MPEG => Range::MPEG,
            AV::JPEG => Range::JPEG,

            AV::NB => unreachable!(),
            _ => unimplemented!(),
        }
    }
}

impl From<Range> for AVColorRange {
    fn from(value: Range) -> AVColorRange {
        use AVColorRange as AV;

        match value {
            Range::Unspecified => AV::UNSPECIFIED,
            Range::MPEG => AV::MPEG,
            Range::JPEG => AV::JPEG,
        }
    }
}

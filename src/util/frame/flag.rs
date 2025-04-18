use crate::ffi::*;
use libc::c_int;

bitflags::bitflags! {
    pub struct Flags: c_int {
        const CORRUPT = AV_FRAME_FLAG_CORRUPT;
    }
}

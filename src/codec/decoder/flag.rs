use crate::ffi::*;
use libc::c_uint;

bitflags::bitflags! {
    pub struct Flags: c_uint {
        const SYNCHRONOUS = AV_CODEC_RECEIVE_FRAME_FLAG_SYNCHRONOUS as c_uint;
    }
}

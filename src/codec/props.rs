use crate::ffi::*;
use libc::c_int;

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct CodecProperties: c_int {
        const INTRA_ONLY = AV_CODEC_PROP_INTRA_ONLY;
        const LOSSY      = AV_CODEC_PROP_LOSSY;
        const LOSSLESS   = AV_CODEC_PROP_LOSSLESS;
        const REORDER    = AV_CODEC_PROP_REORDER;
        #[cfg(feature = "ffmpeg_6_1")]
        const FIELDS     = AV_CODEC_PROP_FIELDS;

        const BITMAP_SUB = AV_CODEC_PROP_BITMAP_SUB;
        const TEXT_SUB   = AV_CODEC_PROP_TEXT_SUB;
    }
}

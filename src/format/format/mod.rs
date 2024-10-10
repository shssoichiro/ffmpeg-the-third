pub mod flag;
pub use self::flag::Flags;

mod input;
pub use self::input::Input;

mod output;
pub use self::output::Output;

mod iter;
pub use self::iter::{DemuxerIter, MuxerIter};

pub fn list_demuxers() -> DemuxerIter {
    DemuxerIter::new()
}

pub fn list_muxers() -> MuxerIter {
    MuxerIter::new()
}

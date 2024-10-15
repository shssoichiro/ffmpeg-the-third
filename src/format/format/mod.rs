pub mod flag;
pub use self::flag::Flags;

mod input;
pub use self::input::Input;

mod output;
pub use self::output::Output;

#[cfg(feature = "ffmpeg_4_0")]
mod new_iter;
#[cfg(feature = "ffmpeg_4_0")]
pub use self::new_iter::{DemuxerIter, MuxerIter};

#[cfg(feature = "ffmpeg_4_0")]
pub fn list_demuxers() -> DemuxerIter {
    DemuxerIter::new()
}

#[cfg(feature = "ffmpeg_4_0")]
pub fn list_muxers() -> MuxerIter {
    MuxerIter::new()
}

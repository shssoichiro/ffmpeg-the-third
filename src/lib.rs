#![allow(non_camel_case_types)]
#![allow(clippy::module_inception)]
// FFI Types may differ across platforms, making casts necessary
#![allow(clippy::unnecessary_cast)]
// This lint sometimes suggests worse code. See rust-lang/rust-clippy#13514
#![allow(clippy::needless_lifetimes)]
// TODO: Add safety docs and remove this #[allow]
#![allow(clippy::missing_safety_doc)]

pub use ffmpeg_sys_the_third as sys;
pub use ffmpeg_sys_the_third as ffi;

pub mod util;
pub use crate::util::channel_layout::{
    self, Channel, ChannelCustom, ChannelLayout, ChannelLayoutIter, ChannelLayoutMask, ChannelOrder,
};
pub use crate::util::{
    chroma, color, dictionary,
    dictionary::Mut as DictionaryMut,
    dictionary::Owned as Dictionary,
    dictionary::Ref as DictionaryRef,
    error::{self, Error},
    frame::{self, Frame},
    log,
    mathematics::{self, rescale, Rescale, Rounding},
    media, option, picture,
    rational::{self, Rational},
    time,
};

#[cfg(feature = "format")]
pub mod format;
#[cfg(feature = "format")]
pub use crate::format::{
    chapter::{Chapter, ChapterMut},
    stream::{Stream, StreamMut},
};

#[cfg(feature = "codec")]
pub mod codec;
#[cfg(feature = "codec")]
pub use crate::codec::{
    audio_service::AudioService,
    codec::Codec,
    decoder,
    discard::Discard,
    encoder,
    field_order::FieldOrder,
    packet::{self, Packet},
    subtitle::{self, Subtitle},
    threading,
};

#[cfg(feature = "device")]
pub mod device;

#[cfg(feature = "filter")]
pub mod filter;
#[cfg(feature = "filter")]
pub use crate::filter::Filter;

pub mod software;

mod as_ptr;
pub use as_ptr::{AsMutPtr, AsPtr};

pub(crate) mod iters;
pub(crate) mod macros;
pub(crate) mod utils;

#[cfg(not(feature = "format"))]
fn init_format() {}

#[cfg(feature = "device")]
fn init_device() {
    device::register_all();
}

#[cfg(not(feature = "device"))]
fn init_device() {}

#[cfg(not(feature = "filter"))]
fn init_filter() {}

pub fn init() -> Result<(), Error> {
    init_device();

    Ok(())
}

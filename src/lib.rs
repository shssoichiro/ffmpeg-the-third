#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
// FFI Types may differ across platforms, making casts necessary
#![allow(clippy::unnecessary_cast)]

#[macro_use]
extern crate bitflags;
pub extern crate ffmpeg_sys_the_third as sys;
#[cfg(feature = "image")]
extern crate image;
extern crate libc;
#[cfg(feature = "serialize")]
extern crate serde;

pub use sys as ffi;

#[macro_use]
pub mod util;
pub use util::channel_layout::{self, ChannelLayout};
pub use util::chroma;
pub use util::color;
pub use util::dictionary;
pub use util::dictionary::Mut as DictionaryMut;
pub use util::dictionary::Owned as Dictionary;
pub use util::dictionary::Ref as DictionaryRef;
pub use util::error::{self, Error};
pub use util::frame::{self, Frame};
pub use util::log;
pub use util::mathematics::{self, rescale, Rescale, Rounding};
pub use util::media;
pub use util::option;
pub use util::picture;
pub use util::rational::{self, Rational};
pub use util::time;

#[cfg(feature = "format")]
pub mod format;
#[cfg(feature = "format")]
pub use format::chapter::{Chapter, ChapterMut};
#[cfg(feature = "format")]
pub use format::format::Format;
#[cfg(feature = "format")]
pub use format::stream::{Stream, StreamMut};

#[cfg(feature = "codec")]
pub mod codec;
#[cfg(feature = "codec")]
pub use codec::audio_service::AudioService;
#[cfg(feature = "codec")]
pub use codec::codec::Codec;
#[cfg(feature = "codec")]
pub use codec::discard::Discard;
#[cfg(feature = "codec")]
pub use codec::field_order::FieldOrder;
#[cfg(feature = "codec")]
pub use codec::packet::{self, Packet};
#[cfg(all(feature = "codec", not(feature = "ffmpeg_5_0")))]
pub use codec::picture::Picture;
#[cfg(feature = "codec")]
pub use codec::subtitle::{self, Subtitle};
#[cfg(feature = "codec")]
pub use codec::threading;
#[cfg(feature = "codec")]
pub use codec::{decoder, encoder};

#[cfg(feature = "device")]
pub mod device;

#[cfg(feature = "filter")]
pub mod filter;
#[cfg(feature = "filter")]
pub use filter::Filter;

pub mod software;

fn init_error() {
    util::error::register_all();
}

#[cfg(all(feature = "format", not(feature = "ffmpeg_5_0")))]
fn init_format() {
    format::register_all();
}

#[cfg(not(feature = "format"))]
fn init_format() {}

#[cfg(feature = "device")]
fn init_device() {
    device::register_all();
}

#[cfg(not(feature = "device"))]
fn init_device() {}

#[cfg(all(feature = "filter", not(feature = "ffmpeg_5_0")))]
fn init_filter() {
    filter::register_all();
}

#[cfg(not(feature = "filter"))]
fn init_filter() {}

pub fn init() -> Result<(), Error> {
    init_error();
    #[cfg(not(feature = "ffmpeg_5_0"))]
    init_format();
    init_device();
    #[cfg(not(feature = "ffmpeg_5_0"))]
    init_filter();

    Ok(())
}

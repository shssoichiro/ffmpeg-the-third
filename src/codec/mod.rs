pub mod flag;
pub use self::flag::Flags;

pub mod id;
pub use self::id::Id;

pub mod packet;

pub mod subtitle;

#[cfg(not(feature = "ffmpeg_5_0"))]
pub mod picture;

pub mod descriptor;
pub use self::descriptor::CodecDescriptor;

pub mod discard;

pub mod config;

pub mod context;
pub use self::context::Context;

pub mod capabilities;
pub use self::capabilities::Capabilities;

pub mod codec;
pub use self::codec::{Audio, Codec, Video};

pub mod parameters;
pub use self::parameters::Parameters;

pub mod audio_service;
pub mod field_order;

pub mod compliance;
pub use self::compliance::Compliance;

pub mod debug;
pub use self::debug::Debug;

pub mod profile;
pub use self::profile::Profile;

pub mod props;
pub use self::props::CodecProperties;

pub mod threading;

pub mod decoder;
pub mod encoder;
pub mod traits;

use crate::ffi::*;
use crate::utils;

pub fn version() -> u32 {
    unsafe { avcodec_version() }
}

pub fn configuration() -> &'static str {
    unsafe { utils::str_from_c_ptr(avcodec_configuration()) }
}

pub fn license() -> &'static str {
    unsafe { utils::str_from_c_ptr(avcodec_license()) }
}

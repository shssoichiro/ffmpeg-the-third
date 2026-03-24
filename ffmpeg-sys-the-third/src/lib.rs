include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod avutil;
pub use crate::avutil::*;

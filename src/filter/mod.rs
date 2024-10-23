pub mod flag;
pub use self::flag::Flags;

pub mod pad;
pub use self::pad::Pad;

pub mod filter;
pub use self::filter::Filter;

pub mod context;
pub use self::context::{Context, Sink, Source};

pub mod graph;
pub use self::graph::Graph;

use std::ffi::CString;

use crate::ffi::*;
use crate::utils;
#[cfg(not(feature = "ffmpeg_5_0"))]
use crate::Error;

#[cfg(not(feature = "ffmpeg_5_0"))]
pub fn register_all() {
    unsafe {
        avfilter_register_all();
    }
}

#[cfg(not(feature = "ffmpeg_5_0"))]
pub fn register(filter: &Filter) -> Result<(), Error> {
    unsafe {
        match avfilter_register(filter.as_ptr() as *mut _) {
            0 => Ok(()),
            _ => Err(Error::InvalidData),
        }
    }
}

pub fn version() -> u32 {
    unsafe { avfilter_version() }
}

pub fn configuration() -> &'static str {
    unsafe { utils::str_from_c_ptr(avfilter_configuration()) }
}

pub fn license() -> &'static str {
    unsafe { utils::str_from_c_ptr(avfilter_license()) }
}

pub fn find(name: &str) -> Option<Filter> {
    unsafe {
        let name = CString::new(name).unwrap();
        let ptr = avfilter_get_by_name(name.as_ptr());

        if ptr.is_null() {
            None
        } else {
            Some(Filter::wrap(ptr as *mut _))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paditer() {
        #[cfg(not(feature = "ffmpeg_5_0"))]
        register_all();
        assert_eq!(
            find("overlay")
                .unwrap()
                .inputs()
                .unwrap()
                .map(|input| input.name().unwrap().to_string())
                .collect::<Vec<_>>(),
            vec!("main", "overlay")
        );
    }
}

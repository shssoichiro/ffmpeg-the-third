use std::ptr::null_mut;

use crate::ffi::*;
use crate::format::format::{Input, Output};
use crate::format::Format;
use libc::c_void;

pub struct DemuxerIter {
    ptr: *mut c_void,
}

impl DemuxerIter {
    pub fn new() -> Self {
        Self { ptr: null_mut() }
    }
}

impl Default for DemuxerIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for DemuxerIter {
    type Item = Format;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let next = av_demuxer_iterate(&mut self.ptr);
            if next.is_null() {
                None
            } else {
                Some(Format::Input(Input::wrap(next as _)))
            }
        }
    }
}

pub struct MuxerIter {
    ptr: *mut c_void,
}

impl MuxerIter {
    pub fn new() -> Self {
        Self { ptr: null_mut() }
    }
}

impl Default for MuxerIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for MuxerIter {
    type Item = Format;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let next = av_muxer_iterate(&mut self.ptr);
            if next.is_null() {
                None
            } else {
                Some(Format::Output(Output::wrap(next as _)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn muxer_iter() {
        for f in MuxerIter::new() {
            println!("{}:", f.name());
            println!("\t{}", f.description());
            println!("\t{:?}", f.extensions());
            println!("\t{:?}", f.mime_types());
        }
    }

    #[test]
    fn demuxer_iter() {
        for f in DemuxerIter::new() {
            println!("{}:", f.name());
            println!("\t{}", f.description());
            println!("\t{:?}", f.extensions());
            println!("\t{:?}", f.mime_types());
        }
    }
}

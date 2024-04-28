use std::ptr::{addr_of_mut, null_mut};

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

impl Iterator for DemuxerIter {
    type Item = Format;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let next = av_demuxer_iterate(addr_of_mut!(self.ptr));
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

impl Iterator for MuxerIter {
    type Item = Format;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let next = av_muxer_iterate(addr_of_mut!(self.ptr));
            if next.is_null() {
                None
            } else {
                Some(Format::Output(Output::wrap(next as _)))
            }
        }
    }
}

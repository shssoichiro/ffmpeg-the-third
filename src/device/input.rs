use std::ptr;

use crate::ffi::*;
use crate::format;

pub struct AudioIter(*const AVInputFormat);

impl Iterator for AudioIter {
    type Item = format::Input;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let ptr = av_input_audio_device_next(self.0);

            if let Some(input) = format::Input::from_raw(ptr) {
                self.0 = ptr;
                Some(input)
            } else {
                None
            }
        }
    }
}

pub fn audio() -> AudioIter {
    AudioIter(ptr::null())
}

pub struct VideoIter(*const AVInputFormat);

impl Iterator for VideoIter {
    type Item = format::Input;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let ptr = av_input_video_device_next(self.0);

            if let Some(input) = format::Input::from_raw(ptr) {
                self.0 = ptr;
                Some(input)
            } else {
                None
            }
        }
    }
}

pub fn video() -> VideoIter {
    VideoIter(ptr::null())
}

use std::ptr;

use crate::ffi::*;
use crate::format;

pub struct AudioIter(*const AVInputFormat);

impl Iterator for AudioIter {
    type Item = format::Input;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let inner = self.0;

            // Pre-5.0 FFmpeg uses a non-const pointer here
            #[cfg(not(feature = "ffmpeg_5_0"))]
            let inner = inner as *mut _;

            let ptr = av_input_audio_device_next(inner);

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
            let inner = self.0;

            // Pre-5.0 FFmpeg uses a non-const pointer here
            #[cfg(not(feature = "ffmpeg_5_0"))]
            let inner = inner as *mut _;

            let ptr = av_input_video_device_next(inner);

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

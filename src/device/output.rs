use std::ptr;

use crate::ffi::*;
use crate::format;

pub struct AudioIter(*const AVOutputFormat);

impl Iterator for AudioIter {
    type Item = format::Output;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let inner = self.0;

            // Pre-5.0 FFmpeg uses a non-const pointer here
            #[cfg(not(feature = "ffmpeg_5_0"))]
            let inner = inner as *mut _;

            let ptr = av_output_audio_device_next(inner);

            if let Some(output) = format::Output::from_raw(ptr) {
                self.0 = ptr;
                Some(output)
            } else {
                None
            }
        }
    }
}

pub fn audio() -> AudioIter {
    AudioIter(ptr::null_mut())
}

pub struct VideoIter(*const AVOutputFormat);

impl Iterator for VideoIter {
    type Item = format::Output;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let inner = self.0;

            // Pre-5.0 FFmpeg uses a non-const pointer here
            #[cfg(not(feature = "ffmpeg_5_0"))]
            let inner = inner as *mut _;

            let ptr = av_output_video_device_next(inner);

            if let Some(output) = format::Output::from_raw(ptr) {
                self.0 = ptr;
                Some(output)
            } else {
                None
            }
        }
    }
}

pub fn video() -> VideoIter {
    VideoIter(ptr::null_mut())
}

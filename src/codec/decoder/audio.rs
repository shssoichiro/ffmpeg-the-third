use std::ops::{Deref, DerefMut};

use super::Opened;
use crate::codec::Context;
use crate::util::format;
use crate::AudioService;

use crate::ChannelLayout;

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;

pub struct Audio(pub Opened);

impl Audio {
    pub fn rate(&self) -> u32 {
        unsafe { (*self.as_ptr()).sample_rate as u32 }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channels(&self) -> u16 {
        unsafe { (*self.as_ptr()).channels as u16 }
    }

    pub fn format(&self) -> format::Sample {
        unsafe { format::Sample::from((*self.as_ptr()).sample_fmt) }
    }

    pub fn request_format(&mut self, value: format::Sample) {
        unsafe {
            (*self.as_mut_ptr()).request_sample_fmt = value.into();
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn frames(&self) -> usize {
        unsafe { (*self.as_ptr()).frame_number as usize }
    }

    pub fn align(&self) -> usize {
        unsafe { (*self.as_ptr()).block_align as usize }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channel_layout(&self) -> ChannelLayoutMask {
        unsafe { ChannelLayoutMask::from_bits_truncate((*self.as_ptr()).channel_layout) }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn set_channel_layout(&mut self, value: ChannelLayoutMask) {
        unsafe {
            (*self.as_mut_ptr()).channel_layout = value.bits();
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn request_channel_layout(&mut self, value: ChannelLayoutMask) {
        unsafe {
            (*self.as_mut_ptr()).request_channel_layout = value.bits();
        }
    }

    pub fn ch_layout(&self) -> ChannelLayout<'_> {
        unsafe { ChannelLayout::from(&self.as_ptr().as_ref().unwrap().ch_layout) }
    }

    pub fn set_ch_layout(&mut self, value: ChannelLayout) {
        unsafe {
            self.as_mut_ptr().as_mut().unwrap().ch_layout = value.into_owned();
        }
    }

    pub fn audio_service(&mut self) -> AudioService {
        unsafe { AudioService::from((*self.as_mut_ptr()).audio_service_type) }
    }

    pub fn max_bit_rate(&self) -> usize {
        unsafe { (*self.as_ptr()).rc_max_rate as usize }
    }

    pub fn frame_size(&self) -> u32 {
        unsafe { (*self.as_ptr()).frame_size as u32 }
    }
}

impl Deref for Audio {
    type Target = Opened;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Audio {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Audio {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}

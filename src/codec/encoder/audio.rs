use std::ops::{Deref, DerefMut};
use std::ptr;

use crate::ffi::*;

use super::Encoder as Super;
use crate::codec::{traits, Context};
use crate::util::format;
use crate::{Dictionary, Error};

#[cfg(feature = "ffmpeg_5_1")]
use crate::ChannelLayout;

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;

pub struct Audio(pub Super);

impl Audio {
    pub fn open(mut self) -> Result<Encoder, Error> {
        unsafe {
            match avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut()) {
                0 => Ok(Encoder(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn open_as<T, E: traits::Encoder<T>>(mut self, codec: E) -> Result<Encoder, Error> {
        unsafe {
            if let Some(codec) = codec.encoder() {
                match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), ptr::null_mut()) {
                    0 => Ok(Encoder(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::EncoderNotFound)
            }
        }
    }

    pub fn open_with(mut self, options: Dictionary) -> Result<Encoder, Error> {
        unsafe {
            let mut opts = options.disown();
            let res = avcodec_open2(self.as_mut_ptr(), ptr::null(), &mut opts);

            Dictionary::own(opts);

            match res {
                0 => Ok(Encoder(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn open_as_with<T, E: traits::Encoder<T>>(
        mut self,
        codec: E,
        options: Dictionary,
    ) -> Result<Encoder, Error> {
        unsafe {
            if let Some(codec) = codec.encoder() {
                let mut opts = options.disown();
                let res = avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), &mut opts);

                Dictionary::own(opts);

                match res {
                    0 => Ok(Encoder(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::EncoderNotFound)
            }
        }
    }

    pub fn set_rate(&mut self, rate: i32) {
        unsafe {
            (*self.as_mut_ptr()).sample_rate = rate;
        }
    }

    pub fn rate(&self) -> u32 {
        unsafe { (*self.as_ptr()).sample_rate as u32 }
    }

    pub fn set_format(&mut self, value: format::Sample) {
        unsafe {
            (*self.as_mut_ptr()).sample_fmt = value.into();
        }
    }

    pub fn format(&self) -> format::Sample {
        unsafe { format::Sample::from((*self.as_ptr()).sample_fmt) }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn set_channel_layout(&mut self, value: ChannelLayoutMask) {
        unsafe {
            (*self.as_mut_ptr()).channel_layout = value.bits();
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channel_layout(&self) -> ChannelLayoutMask {
        unsafe { ChannelLayoutMask::from_bits_truncate((*self.as_ptr()).channel_layout) }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn set_channels(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).channels = value;
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channels(&self) -> u16 {
        unsafe { (*self.as_ptr()).channels as u16 }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn ch_layout(&self) -> ChannelLayout<'_> {
        unsafe { ChannelLayout::from(&self.as_ptr().as_ref().unwrap().ch_layout) }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn set_ch_layout(&mut self, value: ChannelLayout) {
        unsafe {
            self.as_mut_ptr().as_mut().unwrap().ch_layout = value.into_owned();
        }
    }
}

impl Deref for Audio {
    type Target = Super;

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

pub struct Encoder(pub Audio);

impl Encoder {
    pub fn frame_size(&self) -> u32 {
        unsafe { (*self.as_ptr()).frame_size as u32 }
    }
}

impl Deref for Encoder {
    type Target = Audio;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Encoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Encoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}

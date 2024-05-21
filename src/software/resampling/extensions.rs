use super::Context;
use crate::util::format;
use crate::{decoder, frame, Error};

#[cfg(feature = "ffmpeg_5_1")]
use crate::ChannelLayout;

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;

impl frame::Audio {
    #[cfg(not(feature = "ffmpeg_7_0"))]
    #[inline]
    pub fn resampler(
        &self,
        format: format::Sample,
        channel_layout: ChannelLayoutMask,
        rate: u32,
    ) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.channel_layout(),
            unsafe { (*self.as_ptr()).sample_rate as u32 },
            format,
            channel_layout,
            rate,
        )
    }

    #[cfg(feature = "ffmpeg_5_1")]
    #[inline]
    pub fn resampler2(
        &self,
        format: format::Sample,
        ch_layout: ChannelLayout,
        rate: u32,
    ) -> Result<Context, Error> {
        Context::get2(
            self.format(),
            self.ch_layout(),
            unsafe { (*self.as_ptr()).sample_rate as u32 },
            format,
            ch_layout,
            rate,
        )
    }
}

impl decoder::Audio {
    #[cfg(not(feature = "ffmpeg_7_0"))]
    #[inline]
    pub fn resampler(
        &self,
        format: format::Sample,
        channel_layout: ChannelLayoutMask,
        rate: u32,
    ) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.channel_layout(),
            self.rate(),
            format,
            channel_layout,
            rate,
        )
    }

    #[cfg(feature = "ffmpeg_5_1")]
    #[inline]
    pub fn resampler2(
        &self,
        format: format::Sample,
        ch_layout: ChannelLayout,
        rate: u32,
    ) -> Result<Context, Error> {
        Context::get2(
            self.format(),
            self.ch_layout(),
            self.rate(),
            format,
            ch_layout,
            rate,
        )
    }
}

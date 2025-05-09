use crate::ffi::*;
use libc::c_ulonglong;

bitflags::bitflags! {
    #[derive(Eq, PartialEq, Copy, Clone, Debug)]
    pub struct ChannelLayoutMask: c_ulonglong {
        const FRONT_LEFT            = AV_CH_FRONT_LEFT;
        const FRONT_RIGHT           = AV_CH_FRONT_RIGHT;
        const FRONT_CENTER          = AV_CH_FRONT_CENTER;
        const LOW_FREQUENCY         = AV_CH_LOW_FREQUENCY;
        const BACK_LEFT             = AV_CH_BACK_LEFT;
        const BACK_RIGHT            = AV_CH_BACK_RIGHT;
        const FRONT_LEFT_OF_CENTER  = AV_CH_FRONT_LEFT_OF_CENTER;
        const FRONT_RIGHT_OF_CENTER = AV_CH_FRONT_RIGHT_OF_CENTER;
        const BACK_CENTER           = AV_CH_BACK_CENTER;
        const SIDE_LEFT             = AV_CH_SIDE_LEFT;
        const SIDE_RIGHT            = AV_CH_SIDE_RIGHT;
        const TOP_CENTER            = AV_CH_TOP_CENTER;
        const TOP_FRONT_LEFT        = AV_CH_TOP_FRONT_LEFT;
        const TOP_FRONT_CENTER      = AV_CH_TOP_FRONT_CENTER;
        const TOP_FRONT_RIGHT       = AV_CH_TOP_FRONT_RIGHT;
        const TOP_BACK_LEFT         = AV_CH_TOP_BACK_LEFT;
        const TOP_BACK_CENTER       = AV_CH_TOP_BACK_CENTER;
        const TOP_BACK_RIGHT        = AV_CH_TOP_BACK_RIGHT;
        const STEREO_LEFT           = AV_CH_STEREO_LEFT;
        const STEREO_RIGHT          = AV_CH_STEREO_RIGHT;
        const WIDE_LEFT             = AV_CH_WIDE_LEFT;
        const WIDE_RIGHT            = AV_CH_WIDE_RIGHT;
        const SURROUND_DIRECT_LEFT  = AV_CH_SURROUND_DIRECT_LEFT;
        const SURROUND_DIRECT_RIGHT = AV_CH_SURROUND_DIRECT_RIGHT;
        const LOW_FREQUENCY_2       = AV_CH_LOW_FREQUENCY_2;
        #[cfg(not(feature = "ffmpeg_7_0"))]
        const NATIVE                = AV_CH_LAYOUT_NATIVE;

        const MONO                = AV_CH_LAYOUT_MONO;
        const STEREO              = AV_CH_LAYOUT_STEREO;
        const _2POINT1            = AV_CH_LAYOUT_2POINT1;
        const _2_1                = AV_CH_LAYOUT_2_1;
        const SURROUND            = AV_CH_LAYOUT_SURROUND;
        const _3POINT1            = AV_CH_LAYOUT_3POINT1;
        const _4POINT0            = AV_CH_LAYOUT_4POINT0;
        const _4POINT1            = AV_CH_LAYOUT_4POINT1;
        const _2_2                = AV_CH_LAYOUT_2_2;
        const QUAD                = AV_CH_LAYOUT_QUAD;
        const _5POINT0            = AV_CH_LAYOUT_5POINT0;
        const _5POINT1            = AV_CH_LAYOUT_5POINT1;
        const _5POINT0_BACK       = AV_CH_LAYOUT_5POINT0_BACK;
        const _5POINT1_BACK       = AV_CH_LAYOUT_5POINT1_BACK;
        const _6POINT0            = AV_CH_LAYOUT_6POINT0;
        const _6POINT0_FRONT      = AV_CH_LAYOUT_6POINT0_FRONT;
        const HEXAGONAL           = AV_CH_LAYOUT_HEXAGONAL;
        #[cfg(feature = "ffmpeg_5_1")]
        const _3POINT1POINT2      = AV_CH_LAYOUT_3POINT1POINT2;
        const _6POINT1            = AV_CH_LAYOUT_6POINT1;
        const _6POINT1_BACK       = AV_CH_LAYOUT_6POINT1_BACK;
        const _6POINT1_FRONT      = AV_CH_LAYOUT_6POINT1_FRONT;
        const _7POINT0            = AV_CH_LAYOUT_7POINT0;
        const _7POINT0_FRONT      = AV_CH_LAYOUT_7POINT0_FRONT;
        const _7POINT1            = AV_CH_LAYOUT_7POINT1;
        const _7POINT1_WIDE       = AV_CH_LAYOUT_7POINT1_WIDE;
        const _7POINT1_WIDE_BACK  = AV_CH_LAYOUT_7POINT1_WIDE_BACK;
        #[cfg(feature = "ffmpeg_5_1")]
        const _5POINT1POINT2_BACK = AV_CH_LAYOUT_5POINT1POINT2_BACK;
        const OCTAGONAL           = AV_CH_LAYOUT_OCTAGONAL;
        #[cfg(feature = "ffmpeg_5_1")]
        const CUBE                = AV_CH_LAYOUT_CUBE;
        #[cfg(feature = "ffmpeg_5_1")]
        const _5POINT1POINT4_BACK = AV_CH_LAYOUT_5POINT1POINT4_BACK;
        #[cfg(feature = "ffmpeg_5_1")]
        const _7POINT1POINT2      = AV_CH_LAYOUT_7POINT1POINT2;
        #[cfg(feature = "ffmpeg_5_1")]
        const _7POINT1POINT4_BACK = AV_CH_LAYOUT_7POINT1POINT4_BACK;
        #[cfg(feature = "ffmpeg_7_0")]
        const _7POINT2POINT3      = AV_CH_LAYOUT_7POINT2POINT3;
        #[cfg(feature = "ffmpeg_7_0")]
        const _9POINT1POINT4_BACK = AV_CH_LAYOUT_9POINT1POINT4_BACK;
        const HEXADECAGONAL       = AV_CH_LAYOUT_HEXADECAGONAL;
        const STEREO_DOWNMIX      = AV_CH_LAYOUT_STEREO_DOWNMIX;
        #[cfg(feature = "ffmpeg_5_1")]
        const _22POINT2           = AV_CH_LAYOUT_22POINT2;
    }
}

#[cfg(not(feature = "ffmpeg_7_0"))]
impl ChannelLayoutMask {
    #[inline]
    pub fn channels(&self) -> i32 {
        unsafe { av_get_channel_layout_nb_channels(self.bits()) }
    }

    pub fn default(number: i32) -> ChannelLayoutMask {
        unsafe {
            ChannelLayoutMask::from_bits_truncate(
                av_get_default_channel_layout(number) as c_ulonglong
            )
        }
    }
}

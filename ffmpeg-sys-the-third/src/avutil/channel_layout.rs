use libc::c_int;

use crate::AVChannel::*;
use crate::*;
use crate::{AVChannelLayout, AVChannelOrder};

use std::fmt;
use std::mem::{align_of, size_of};
use std::ptr::null_mut;

impl AVChannelLayout {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            order: AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC,
            nb_channels: 0,
            u: AVChannelLayout__bindgen_ty_1 { mask: 0 },
            opaque: null_mut(),
        }
    }
}

impl Clone for AVChannelLayout {
    fn clone(&self) -> Self {
        let mut cloned = Self::empty();
        cloned.clone_from(self);

        cloned
    }

    fn clone_from(&mut self, source: &Self) {
        #[cold]
        fn clone_failed(channels: c_int) -> ! {
            use std::alloc::{handle_alloc_error, Layout};

            let alloc_size = channels as usize * size_of::<AVChannelCustom>();
            let layout =
                Layout::from_size_align(alloc_size, align_of::<AVChannelCustom>()).unwrap();
            handle_alloc_error(layout)
        }

        let ret = unsafe { av_channel_layout_copy(self as _, source as _) };

        if ret < 0 {
            clone_failed(self.nb_channels);
        }
    }
}

impl Drop for AVChannelLayout {
    fn drop(&mut self) {
        unsafe { av_channel_layout_uninit(self as _) }
    }
}

impl PartialEq for AVChannelLayout {
    fn eq(&self, other: &Self) -> bool {
        unsafe { av_channel_layout_compare(self as _, other as _) == 0 }
    }
}

impl fmt::Debug for AVChannelLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("AVChannelLayout");
        dbg.field("order", &self.order)
            .field("nb_channels", &self.nb_channels);

        unsafe {
            match self.order {
                AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC => {} // no other valid fields
                AVChannelOrder::AV_CHANNEL_ORDER_NATIVE
                | AVChannelOrder::AV_CHANNEL_ORDER_AMBISONIC => {
                    dbg.field("mask", &format_args!("0x{:X}", self.u.mask));
                }
                AVChannelOrder::AV_CHANNEL_ORDER_CUSTOM => {
                    dbg.field(
                        "map",
                        &std::slice::from_raw_parts(self.u.map, self.nb_channels as usize),
                    );
                } // Starting with FFmpeg 7.0:
                  // Not part of public API, but we have to exhaustively match
                  // AVChannelOrder::FF_CHANNEL_ORDER_NB => {}
            }
        }

        dbg.field("opaque", &self.opaque).finish()
    }
}

impl fmt::Debug for AVChannelCustom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("AVChannelCustom")
                .field("id", &self.id)
                .field("name", &std::ffi::CStr::from_ptr(self.name.as_ptr()))
                .field("opaque", &self.opaque)
                .finish()
        }
    }
}

// Here until https://github.com/rust-lang/rust-bindgen/issues/2192 /
// https://github.com/rust-lang/rust-bindgen/issues/258 is fixed.

// The constants here should be kept up to date with libavutil/channel_layout.h.

// Audio channel masks
pub const AV_CH_FRONT_LEFT: u64 = 1 << (AV_CHAN_FRONT_LEFT as i32);
pub const AV_CH_FRONT_RIGHT: u64 = 1 << (AV_CHAN_FRONT_RIGHT as i32);
pub const AV_CH_FRONT_CENTER: u64 = 1 << (AV_CHAN_FRONT_CENTER as i32);
pub const AV_CH_LOW_FREQUENCY: u64 = 1 << (AV_CHAN_LOW_FREQUENCY as i32);
pub const AV_CH_BACK_LEFT: u64 = 1 << (AV_CHAN_BACK_LEFT as i32);
pub const AV_CH_BACK_RIGHT: u64 = 1 << (AV_CHAN_BACK_RIGHT as i32);
pub const AV_CH_FRONT_LEFT_OF_CENTER: u64 = 1 << (AV_CHAN_FRONT_LEFT_OF_CENTER as i32);
pub const AV_CH_FRONT_RIGHT_OF_CENTER: u64 = 1 << (AV_CHAN_FRONT_RIGHT_OF_CENTER as i32);
pub const AV_CH_BACK_CENTER: u64 = 1 << (AV_CHAN_BACK_CENTER as i32);
pub const AV_CH_SIDE_LEFT: u64 = 1 << (AV_CHAN_SIDE_LEFT as i32);
pub const AV_CH_SIDE_RIGHT: u64 = 1 << (AV_CHAN_SIDE_RIGHT as i32);
pub const AV_CH_TOP_CENTER: u64 = 1 << (AV_CHAN_TOP_CENTER as i32);
pub const AV_CH_TOP_FRONT_LEFT: u64 = 1 << (AV_CHAN_TOP_FRONT_LEFT as i32);
pub const AV_CH_TOP_FRONT_CENTER: u64 = 1 << (AV_CHAN_TOP_FRONT_CENTER as i32);
pub const AV_CH_TOP_FRONT_RIGHT: u64 = 1 << (AV_CHAN_TOP_FRONT_RIGHT as i32);
pub const AV_CH_TOP_BACK_LEFT: u64 = 1 << (AV_CHAN_TOP_BACK_LEFT as i32);
pub const AV_CH_TOP_BACK_CENTER: u64 = 1 << (AV_CHAN_TOP_BACK_CENTER as i32);
pub const AV_CH_TOP_BACK_RIGHT: u64 = 1 << (AV_CHAN_TOP_BACK_RIGHT as i32);
pub const AV_CH_STEREO_LEFT: u64 = 1 << (AV_CHAN_STEREO_LEFT as i32);
pub const AV_CH_STEREO_RIGHT: u64 = 1 << (AV_CHAN_STEREO_RIGHT as i32);
pub const AV_CH_WIDE_LEFT: u64 = 1 << (AV_CHAN_WIDE_LEFT as i32);
pub const AV_CH_WIDE_RIGHT: u64 = 1 << (AV_CHAN_WIDE_RIGHT as i32);
pub const AV_CH_SURROUND_DIRECT_LEFT: u64 = 1 << (AV_CHAN_SURROUND_DIRECT_LEFT as i32);
pub const AV_CH_SURROUND_DIRECT_RIGHT: u64 = 1 << (AV_CHAN_SURROUND_DIRECT_RIGHT as i32);
pub const AV_CH_LOW_FREQUENCY_2: u64 = 1 << (AV_CHAN_LOW_FREQUENCY_2 as i32);
pub const AV_CH_TOP_SIDE_LEFT: u64 = 1 << (AV_CHAN_TOP_SIDE_LEFT as i32);
pub const AV_CH_TOP_SIDE_RIGHT: u64 = 1 << (AV_CHAN_TOP_SIDE_RIGHT as i32);
pub const AV_CH_BOTTOM_FRONT_CENTER: u64 = 1 << (AV_CHAN_BOTTOM_FRONT_CENTER as i32);
pub const AV_CH_BOTTOM_FRONT_LEFT: u64 = 1 << (AV_CHAN_BOTTOM_FRONT_LEFT as i32);
pub const AV_CH_BOTTOM_FRONT_RIGHT: u64 = 1 << (AV_CHAN_BOTTOM_FRONT_RIGHT as i32);

// Audio channel layouts
pub const AV_CH_LAYOUT_MONO: u64 = AV_CH_FRONT_CENTER;
pub const AV_CH_LAYOUT_STEREO: u64 = AV_CH_FRONT_LEFT | AV_CH_FRONT_RIGHT;
pub const AV_CH_LAYOUT_2POINT1: u64 = AV_CH_LAYOUT_STEREO | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_2_1: u64 = AV_CH_LAYOUT_STEREO | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_SURROUND: u64 = AV_CH_LAYOUT_STEREO | AV_CH_FRONT_CENTER;
pub const AV_CH_LAYOUT_3POINT1: u64 = AV_CH_LAYOUT_SURROUND | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_4POINT0: u64 = AV_CH_LAYOUT_SURROUND | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_4POINT1: u64 = AV_CH_LAYOUT_4POINT0 | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_2_2: u64 = AV_CH_LAYOUT_STEREO | AV_CH_SIDE_LEFT | AV_CH_SIDE_RIGHT;
pub const AV_CH_LAYOUT_QUAD: u64 = AV_CH_LAYOUT_STEREO | AV_CH_BACK_LEFT | AV_CH_BACK_RIGHT;
pub const AV_CH_LAYOUT_5POINT0: u64 = AV_CH_LAYOUT_SURROUND | AV_CH_SIDE_LEFT | AV_CH_SIDE_RIGHT;
pub const AV_CH_LAYOUT_5POINT1: u64 = AV_CH_LAYOUT_5POINT0 | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_5POINT0_BACK: u64 =
    AV_CH_LAYOUT_SURROUND | AV_CH_BACK_LEFT | AV_CH_BACK_RIGHT;
pub const AV_CH_LAYOUT_5POINT1_BACK: u64 = AV_CH_LAYOUT_5POINT0_BACK | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_6POINT0: u64 = AV_CH_LAYOUT_5POINT0 | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_6POINT0_FRONT: u64 =
    AV_CH_LAYOUT_2_2 | AV_CH_FRONT_LEFT_OF_CENTER | AV_CH_FRONT_RIGHT_OF_CENTER;
pub const AV_CH_LAYOUT_HEXAGONAL: u64 = AV_CH_LAYOUT_5POINT0_BACK | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_3POINT1POINT2: u64 =
    AV_CH_LAYOUT_3POINT1 | AV_CH_TOP_FRONT_LEFT | AV_CH_TOP_FRONT_RIGHT;
pub const AV_CH_LAYOUT_6POINT1: u64 = AV_CH_LAYOUT_5POINT1 | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_6POINT1_BACK: u64 = AV_CH_LAYOUT_5POINT1_BACK | AV_CH_BACK_CENTER;
pub const AV_CH_LAYOUT_6POINT1_FRONT: u64 = AV_CH_LAYOUT_6POINT0_FRONT | AV_CH_LOW_FREQUENCY;
pub const AV_CH_LAYOUT_7POINT0: u64 = AV_CH_LAYOUT_5POINT0 | AV_CH_BACK_LEFT | AV_CH_BACK_RIGHT;
pub const AV_CH_LAYOUT_7POINT0_FRONT: u64 =
    AV_CH_LAYOUT_5POINT0 | AV_CH_FRONT_LEFT_OF_CENTER | AV_CH_FRONT_RIGHT_OF_CENTER;
pub const AV_CH_LAYOUT_7POINT1: u64 = AV_CH_LAYOUT_5POINT1 | AV_CH_BACK_LEFT | AV_CH_BACK_RIGHT;
pub const AV_CH_LAYOUT_7POINT1_WIDE: u64 =
    AV_CH_LAYOUT_5POINT1 | AV_CH_FRONT_LEFT_OF_CENTER | AV_CH_FRONT_RIGHT_OF_CENTER;
pub const AV_CH_LAYOUT_7POINT1_WIDE_BACK: u64 =
    AV_CH_LAYOUT_5POINT1_BACK | AV_CH_FRONT_LEFT_OF_CENTER | AV_CH_FRONT_RIGHT_OF_CENTER;
pub const AV_CH_LAYOUT_5POINT1POINT2_BACK: u64 =
    AV_CH_LAYOUT_5POINT1_BACK | AV_CH_TOP_FRONT_LEFT | AV_CH_TOP_FRONT_RIGHT;
pub const AV_CH_LAYOUT_OCTAGONAL: u64 =
    AV_CH_LAYOUT_5POINT0 | AV_CH_BACK_LEFT | AV_CH_BACK_CENTER | AV_CH_BACK_RIGHT;
pub const AV_CH_LAYOUT_CUBE: u64 = AV_CH_LAYOUT_QUAD
    | AV_CH_TOP_FRONT_LEFT
    | AV_CH_TOP_FRONT_RIGHT
    | AV_CH_TOP_BACK_LEFT
    | AV_CH_TOP_BACK_RIGHT;
pub const AV_CH_LAYOUT_5POINT1POINT4_BACK: u64 =
    AV_CH_LAYOUT_5POINT1POINT2_BACK | AV_CH_TOP_BACK_LEFT | AV_CH_TOP_BACK_RIGHT;
pub const AV_CH_LAYOUT_7POINT1POINT2: u64 =
    AV_CH_LAYOUT_7POINT1 | AV_CH_TOP_FRONT_LEFT | AV_CH_TOP_FRONT_RIGHT;
pub const AV_CH_LAYOUT_7POINT1POINT4_BACK: u64 =
    AV_CH_LAYOUT_7POINT1POINT2 | AV_CH_TOP_BACK_LEFT | AV_CH_TOP_BACK_RIGHT;
pub const AV_CH_LAYOUT_HEXADECAGONAL: u64 = AV_CH_LAYOUT_OCTAGONAL
    | AV_CH_WIDE_LEFT
    | AV_CH_WIDE_RIGHT
    | AV_CH_TOP_BACK_LEFT
    | AV_CH_TOP_BACK_RIGHT
    | AV_CH_TOP_BACK_CENTER
    | AV_CH_TOP_FRONT_CENTER
    | AV_CH_TOP_FRONT_LEFT
    | AV_CH_TOP_FRONT_RIGHT;
pub const AV_CH_LAYOUT_STEREO_DOWNMIX: u64 = AV_CH_STEREO_LEFT | AV_CH_STEREO_RIGHT;
pub const AV_CH_LAYOUT_22POINT2: u64 = AV_CH_LAYOUT_5POINT1_BACK
    | AV_CH_FRONT_LEFT_OF_CENTER
    | AV_CH_FRONT_RIGHT_OF_CENTER
    | AV_CH_BACK_CENTER
    | AV_CH_LOW_FREQUENCY_2
    | AV_CH_SIDE_LEFT
    | AV_CH_SIDE_RIGHT
    | AV_CH_TOP_FRONT_LEFT
    | AV_CH_TOP_FRONT_RIGHT
    | AV_CH_TOP_FRONT_CENTER
    | AV_CH_TOP_CENTER
    | AV_CH_TOP_BACK_LEFT
    | AV_CH_TOP_BACK_RIGHT
    | AV_CH_TOP_SIDE_LEFT
    | AV_CH_TOP_SIDE_RIGHT
    | AV_CH_TOP_BACK_CENTER
    | AV_CH_BOTTOM_FRONT_CENTER
    | AV_CH_BOTTOM_FRONT_LEFT
    | AV_CH_BOTTOM_FRONT_RIGHT;

pub const AV_CH_LAYOUT_7POINT1_TOP_BACK: u64 = AV_CH_LAYOUT_5POINT1POINT2_BACK;

// Audio channel layouts as AVChannelLayout
pub const fn AV_CHANNEL_LAYOUT_MASK(nb_channels: c_int, channel_mask: u64) -> AVChannelLayout {
    AVChannelLayout {
        order: AVChannelOrder::AV_CHANNEL_ORDER_NATIVE,
        nb_channels,
        u: crate::AVChannelLayout__bindgen_ty_1 { mask: channel_mask },
        opaque: std::ptr::null_mut(),
    }
}

pub const AV_CHANNEL_LAYOUT_MONO: AVChannelLayout = AV_CHANNEL_LAYOUT_MASK(1, AV_CH_LAYOUT_MONO);
pub const AV_CHANNEL_LAYOUT_STEREO: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(2, AV_CH_LAYOUT_STEREO);
pub const AV_CHANNEL_LAYOUT_2POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(3, AV_CH_LAYOUT_2POINT1);
pub const AV_CHANNEL_LAYOUT_2_1: AVChannelLayout = AV_CHANNEL_LAYOUT_MASK(3, AV_CH_LAYOUT_2_1);
pub const AV_CHANNEL_LAYOUT_SURROUND: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(3, AV_CH_LAYOUT_SURROUND);
pub const AV_CHANNEL_LAYOUT_3POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(4, AV_CH_LAYOUT_3POINT1);
pub const AV_CHANNEL_LAYOUT_4POINT0: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(4, AV_CH_LAYOUT_4POINT0);
pub const AV_CHANNEL_LAYOUT_4POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(5, AV_CH_LAYOUT_4POINT1);
pub const AV_CHANNEL_LAYOUT_2_2: AVChannelLayout = AV_CHANNEL_LAYOUT_MASK(4, AV_CH_LAYOUT_2_2);
pub const AV_CHANNEL_LAYOUT_QUAD: AVChannelLayout = AV_CHANNEL_LAYOUT_MASK(4, AV_CH_LAYOUT_QUAD);
pub const AV_CHANNEL_LAYOUT_5POINT0: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(5, AV_CH_LAYOUT_5POINT0);
pub const AV_CHANNEL_LAYOUT_5POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_5POINT1);
pub const AV_CHANNEL_LAYOUT_5POINT0_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(5, AV_CH_LAYOUT_5POINT0_BACK);
pub const AV_CHANNEL_LAYOUT_5POINT1_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_5POINT1_BACK);
pub const AV_CHANNEL_LAYOUT_6POINT0: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_6POINT0);
pub const AV_CHANNEL_LAYOUT_6POINT0_FRONT: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_6POINT0_FRONT);
pub const AV_CHANNEL_LAYOUT_3POINT1POINT2: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_3POINT1POINT2);
pub const AV_CHANNEL_LAYOUT_HEXAGONAL: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_HEXAGONAL);
pub const AV_CHANNEL_LAYOUT_6POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(7, AV_CH_LAYOUT_6POINT1);
pub const AV_CHANNEL_LAYOUT_6POINT1_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(7, AV_CH_LAYOUT_6POINT1_BACK);
pub const AV_CHANNEL_LAYOUT_6POINT1_FRONT: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(7, AV_CH_LAYOUT_6POINT1_FRONT);
pub const AV_CHANNEL_LAYOUT_7POINT0: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(7, AV_CH_LAYOUT_7POINT0);
pub const AV_CHANNEL_LAYOUT_7POINT0_FRONT: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(7, AV_CH_LAYOUT_7POINT0_FRONT);
pub const AV_CHANNEL_LAYOUT_7POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_7POINT1);
pub const AV_CHANNEL_LAYOUT_7POINT1_WIDE: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_7POINT1_WIDE);
pub const AV_CHANNEL_LAYOUT_7POINT1_WIDE_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_7POINT1_WIDE_BACK);
pub const AV_CHANNEL_LAYOUT_5POINT1POINT2_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_5POINT1POINT2_BACK);
pub const AV_CHANNEL_LAYOUT_OCTAGONAL: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_OCTAGONAL);
pub const AV_CHANNEL_LAYOUT_CUBE: AVChannelLayout = AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_CUBE);
pub const AV_CHANNEL_LAYOUT_5POINT1POINT4_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(10, AV_CH_LAYOUT_5POINT1POINT4_BACK);
pub const AV_CHANNEL_LAYOUT_7POINT1POINT2: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(10, AV_CH_LAYOUT_7POINT1POINT2);
pub const AV_CHANNEL_LAYOUT_7POINT1POINT4_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(12, AV_CH_LAYOUT_7POINT1POINT4_BACK);
pub const AV_CHANNEL_LAYOUT_HEXADECAGONAL: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(16, AV_CH_LAYOUT_HEXADECAGONAL);
pub const AV_CHANNEL_LAYOUT_STEREO_DOWNMIX: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(2, AV_CH_LAYOUT_STEREO_DOWNMIX);
pub const AV_CHANNEL_LAYOUT_22POINT2: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(24, AV_CH_LAYOUT_22POINT2);

pub const AV_CHANNEL_LAYOUT_7POINT1_TOP_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_5POINT1POINT2_BACK;

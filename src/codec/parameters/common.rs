use crate::macros::impl_for_many;

use super::{Parameters, ParametersMut, ParametersRef};
use crate::chroma::Location;
use crate::codec::Id;
use crate::color;
use crate::media;
use crate::{FieldOrder, Rational};

#[cfg(feature = "ffmpeg_5_1")]
use crate::ChannelLayout;

impl_for_many! {
    impl for Parameters, ParametersRef<'p>, ParametersMut<'p> {
        pub fn medium(&self) -> media::Type {
            unsafe { (*self.as_ptr()).codec_type.into() }
        }

        pub fn id(&self) -> Id {
            unsafe { (*self.as_ptr()).codec_id.into() }
        }

        // TODO: codec_tag
        // TODO: extradata
        // TODO: coded_side_data
        // TODO: format (needs From<c_int> for format::Pixel and format::Sample)

        pub fn bit_rate(&self) -> i64 {
            unsafe { (*self.as_ptr()).bit_rate }
        }

        pub fn bits_per_coded_sample(&self) -> u32 {
            unsafe { (*self.as_ptr()).bits_per_coded_sample as u32 }
        }

        pub fn bits_per_raw_sample(&self) -> u32 {
            unsafe { (*self.as_ptr()).bits_per_raw_sample as u32 }
        }

        pub fn profile(&self) -> i32 {
            unsafe { (*self.as_ptr()).profile as i32 }
        }

        pub fn level(&self) -> i32 {
            unsafe { (*self.as_ptr()).level as i32 }
        }

        /// Video only
        pub fn width(&self) -> u32 {
            unsafe { (*self.as_ptr()).width as u32 }
        }

        /// Video only
        pub fn height(&self) -> u32 {
            unsafe { (*self.as_ptr()).height as u32 }
        }

        /// Video only
        pub fn sample_aspect_ratio(&self) -> Rational {
            unsafe { (*self.as_ptr()).sample_aspect_ratio.into() }
        }

        /// Video only
        #[cfg(feature = "ffmpeg_6_1")]
        pub fn framerate(&self) -> Rational {
            unsafe { (*self.as_ptr()).framerate.into() }
        }

        /// Video only
        pub fn field_order(&self) -> FieldOrder {
            unsafe { (*self.as_ptr()).field_order.into() }
        }

        /// Video only
        pub fn color_range(&self) -> color::Range {
            unsafe { (*self.as_ptr()).color_range.into() }
        }

        /// Video only
        pub fn color_primaries(&self) -> color::Primaries {
            unsafe { (*self.as_ptr()).color_primaries.into() }
        }

        /// Video only
        pub fn color_transfer_characteristic(&self) -> color::TransferCharacteristic {
            unsafe { (*self.as_ptr()).color_trc.into() }
        }

        /// Video only
        pub fn color_space(&self) -> color::Space {
            unsafe { (*self.as_ptr()).color_space.into() }
        }

        /// Video only
        pub fn chroma_location(&self) -> Location {
            unsafe { (*self.as_ptr()).chroma_location.into() }
        }

        /// Video only
        pub fn video_delay(&self) -> i32 {
            unsafe { (*self.as_ptr()).video_delay as i32 }
        }

        /// Audio only
        #[cfg(feature = "ffmpeg_5_1")]
        pub fn ch_layout(&self) -> ChannelLayout<'_> {
            unsafe { ChannelLayout::from(&(*self.as_ptr()).ch_layout) }
        }

        /// Audio only
        pub fn sample_rate(&self) -> u32 {
            unsafe { (*self.as_ptr()).sample_rate as u32 }
        }

        /// Audio only
        pub fn block_align(&self) -> u32 {
            unsafe { (*self.as_ptr()).block_align as u32 }
        }

        /// Audio only
        pub fn frame_size(&self) -> u32 {
            unsafe { (*self.as_ptr()).frame_size as u32 }
        }

        /// Audio only
        pub fn initial_padding(&self) -> u32 {
            unsafe { (*self.as_ptr()).initial_padding as u32 }
        }

        /// Audio only
        pub fn trailing_padding(&self) -> u32 {
            unsafe { (*self.as_ptr()).trailing_padding as u32 }
        }

        /// Audio only
        pub fn seek_preroll(&self) -> u32 {
            unsafe { (*self.as_ptr()).seek_preroll as u32 }
        }
    }
}

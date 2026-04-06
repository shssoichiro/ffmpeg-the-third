use super::{Disposition, Stream, StreamMut};
use crate::macros::impl_for_many;

use crate::codec;
use crate::AsPtr;
use crate::{DictionaryRef, Discard, Rational};

#[cfg(not(feature = "ffmpeg_8_0"))]
use super::side_data::SideDataIter;

impl_for_many! {
    impl for Stream<'a>, StreamMut<'a> {
        pub fn id(&self) -> i32 {
            unsafe { (*self.as_ptr()).id }
        }

        pub fn parameters(&self) -> codec::ParametersRef<'_> {
            unsafe {
                codec::ParametersRef::from_raw((*self.as_ptr()).codecpar).expect("codecpar is non-null")
            }
        }

        pub fn index(&self) -> usize {
            unsafe { (*self.as_ptr()).index as usize }
        }

        pub fn time_base(&self) -> Rational {
            unsafe { Rational::from((*self.as_ptr()).time_base) }
        }

        pub fn start_time(&self) -> i64 {
            unsafe { (*self.as_ptr()).start_time }
        }

        pub fn duration(&self) -> i64 {
            unsafe { (*self.as_ptr()).duration }
        }

        pub fn frames(&self) -> i64 {
            unsafe { (*self.as_ptr()).nb_frames }
        }

        pub fn disposition(&self) -> Disposition {
            unsafe { Disposition::from_bits_truncate((*self.as_ptr()).disposition) }
        }

        pub fn discard(&self) -> Discard {
            unsafe { Discard::from((*self.as_ptr()).discard) }
        }

        #[cfg(not(feature = "ffmpeg_8_0"))]
        pub fn side_data(&self) -> SideDataIter<'_, Self> {
            SideDataIter::new(self)
        }

        pub fn rate(&self) -> Rational {
            unsafe { Rational::from((*self.as_ptr()).r_frame_rate) }
        }

        pub fn avg_frame_rate(&self) -> Rational {
            unsafe { Rational::from((*self.as_ptr()).avg_frame_rate) }
        }

        pub fn metadata(&self) -> DictionaryRef<'_> {
            unsafe { DictionaryRef::from_raw((*self.as_ptr()).metadata) }
        }

        pub fn sample_aspect_ratio(&self) -> Rational {
            unsafe { Rational::from((*self.as_ptr()).sample_aspect_ratio) }
        }
    }
}

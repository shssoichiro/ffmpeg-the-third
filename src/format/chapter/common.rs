use super::{Chapter, ChapterMut};
use crate::macros::impl_for_many;

use crate::AsPtr;
use crate::{DictionaryRef, Rational};

impl_for_many! {
    impl for Chapter<'a>, ChapterMut<'a> {
        pub fn id(&self) -> i64 {
            unsafe { (*self.as_ptr()).id as i64 }
        }

        pub fn time_base(&self) -> Rational {
            unsafe { Rational::from((*self.as_ptr()).time_base) }
        }

        pub fn start(&self) -> i64 {
            unsafe { (*self.as_ptr()).start }
        }

        pub fn end(&self) -> i64 {
            unsafe { (*self.as_ptr()).end }
        }

        pub fn metadata(&self) -> DictionaryRef<'_> {
            unsafe { DictionaryRef::from_raw((*self.as_ptr()).metadata) }
        }
    }
}

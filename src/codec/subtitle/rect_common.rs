use crate::macros::impl_for_many;

use super::{Flags, RectMut, RectRef, Type};
use crate::utils;
use crate::AsPtr;

impl_for_many! {
    impl for RectRef<'s>, RectMut<'s> {
        pub fn kind(&self) -> Type {
            unsafe { (*self.as_ptr()).type_.into() }
        }

        pub fn flags(&self) -> Flags {
            let flags = unsafe { (*self.as_ptr()).flags };
            Flags::from_bits_retain(flags)
        }

        pub fn x(&self) -> usize {
            unsafe { (*self.as_ptr()).x as usize }
        }

        pub fn y(&self) -> usize {
            unsafe { (*self.as_ptr()).y as usize }
        }

        pub fn width(&self) -> u32 {
            unsafe { (*self.as_ptr()).w as u32 }
        }

        pub fn height(&self) -> u32 {
            unsafe { (*self.as_ptr()).h as u32 }
        }

        pub fn colors(&self) -> usize {
            unsafe { (*self.as_ptr()).nb_colors as usize }
        }

        pub fn text(&self) -> Option<&str> {
            unsafe { utils::optional_str_from_c_ptr((*self.as_ptr()).text) }
        }

        pub fn ass(&self) -> Option<&str> {
            unsafe { utils::optional_str_from_c_ptr((*self.as_ptr()).ass) }
        }
    }
}

pub mod flag;
pub use self::flag::Flags;

mod rect;
mod rect_common;
pub use self::rect::RectRef;
mod rect_mut;
pub use self::rect_mut::RectMut;

use std::iter::FusedIterator;
use std::mem;
use std::ptr::NonNull;

use crate::ffi::*;
use libc::size_t;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    None,
    Bitmap,
    Text,
    Ass,
}

impl From<AVSubtitleType> for Type {
    fn from(value: AVSubtitleType) -> Type {
        use AVSubtitleType as AV;

        match value {
            AV::NONE => Type::None,
            AV::BITMAP => Type::Bitmap,
            AV::TEXT => Type::Text,
            AV::ASS => Type::Ass,

            _ => unimplemented!(),
        }
    }
}

impl From<Type> for AVSubtitleType {
    fn from(value: Type) -> AVSubtitleType {
        use AVSubtitleType as AV;

        match value {
            Type::None => AV::NONE,
            Type::Bitmap => AV::BITMAP,
            Type::Text => AV::TEXT,
            Type::Ass => AV::ASS,
        }
    }
}

pub struct Subtitle(AVSubtitle);

impl Subtitle {
    pub unsafe fn as_ptr(&self) -> *const AVSubtitle {
        &self.0
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVSubtitle {
        &mut self.0
    }
}

impl Subtitle {
    pub fn new() -> Self {
        unsafe { Subtitle(mem::zeroed()) }
    }

    pub fn pts(&self) -> Option<i64> {
        match self.0.pts {
            AV_NOPTS_VALUE => None,
            pts => Some(pts),
        }
    }

    pub fn set_pts(&mut self, value: Option<i64>) {
        self.0.pts = value.unwrap_or(AV_NOPTS_VALUE);
    }

    pub fn start(&self) -> u32 {
        self.0.start_display_time as u32
    }

    pub fn set_start(&mut self, value: u32) {
        self.0.start_display_time = value;
    }

    pub fn end(&self) -> u32 {
        self.0.end_display_time as u32
    }

    pub fn set_end(&mut self, value: u32) {
        self.0.end_display_time = value;
    }

    pub fn rects(&self) -> RectIter<'_> {
        unsafe {
            let ptrs = if (*self.as_ptr()).rects.is_null() {
                &[]
            } else {
                std::slice::from_raw_parts(
                    (*self.as_ptr()).rects,
                    (*self.as_ptr()).num_rects as usize,
                )
            };

            RectIter::from_av_rects(ptrs)
        }
    }

    pub fn rects_mut(&mut self) -> RectMutIter<'_> {
        unsafe {
            let ptrs = if (*self.as_ptr()).rects.is_null() {
                &mut []
            } else {
                std::slice::from_raw_parts_mut(
                    (*self.as_ptr()).rects,
                    (*self.as_ptr()).num_rects as usize,
                )
            };

            RectMutIter::from_av_rects(ptrs)
        }
    }

    pub fn add_rect(&mut self, kind: Type) -> Option<RectMut<'_>> {
        unsafe {
            let new_sz = 1 + self.0.num_rects as usize;
            let new_ptr = av_realloc(
                self.0.rects as *mut _,
                size_of::<*const AVSubtitleRect>() * new_sz,
            ) as *mut *mut AVSubtitleRect;

            if new_ptr.is_null() {
                return None;
            }

            self.0.rects = new_ptr;
            self.0.num_rects = new_sz as u32;

            let rect = av_mallocz(size_of::<AVSubtitleRect>() as size_t) as *mut AVSubtitleRect;
            let mut rect = NonNull::new(rect)?;

            rect.as_mut().type_ = kind.into();
            *self.0.rects.add(new_sz - 1) = rect.as_ptr();

            Some(RectMut::from_ptr(rect))
        }
    }
}

#[derive(Debug, Clone)]
pub struct RectIter<'s> {
    raw_iter: std::slice::Iter<'s, *mut AVSubtitleRect>,
}

impl<'s> RectIter<'s> {
    pub fn from_av_rects(rects: &'s [*mut AVSubtitleRect]) -> Self {
        Self {
            raw_iter: rects.iter(),
        }
    }
}

impl<'s> Iterator for RectIter<'s> {
    type Item = RectRef<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            // SAFETY: Lifetime is bounded by Self::Item (= 's)
            self.raw_iter
                .next()
                .map(|&ptr| RectRef::from_ptr(NonNull::new(ptr).expect("ptr is non-null")))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw_iter.size_hint()
    }
}

impl<'s> DoubleEndedIterator for RectIter<'s> {
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {
            // SAFETY: Lifetime is bounded by Self::Item (= 's)
            self.raw_iter
                .next_back()
                .map(|&ptr| RectRef::from_ptr(NonNull::new(ptr).expect("ptr is non-null")))
        }
    }
}

impl<'s> ExactSizeIterator for RectIter<'s> {}
impl<'s> FusedIterator for RectIter<'s> {}

#[derive(Debug)]
pub struct RectMutIter<'s> {
    raw_iter: std::slice::IterMut<'s, *mut AVSubtitleRect>,
}

impl<'s> RectMutIter<'s> {
    pub fn from_av_rects(rects: &'s mut [*mut AVSubtitleRect]) -> Self {
        Self {
            raw_iter: rects.iter_mut(),
        }
    }
}

impl<'s> Iterator for RectMutIter<'s> {
    type Item = RectMut<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            // SAFETY: Lifetime is bounded by Self::Item (= 's)
            self.raw_iter
                .next()
                .map(|&mut ptr| RectMut::from_ptr(NonNull::new(ptr).expect("ptr is non-null")))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw_iter.size_hint()
    }
}

impl<'s> DoubleEndedIterator for RectMutIter<'s> {
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {
            // SAFETY: Lifetime is bounded by Self::Item (= 's)
            self.raw_iter
                .next_back()
                .map(|&mut ptr| RectMut::from_ptr(NonNull::new(ptr).expect("ptr is non-null")))
        }
    }
}

impl<'s> ExactSizeIterator for RectMutIter<'s> {}
impl<'s> FusedIterator for RectMutIter<'s> {}

impl Default for Subtitle {
    fn default() -> Self {
        Self::new()
    }
}

use super::{Input, Output};
use crate::iters::impl_slice_iter;
use crate::macros::impl_for_many;
use crate::utils;

use libc::c_int;
use std::marker::PhantomData;
use std::ptr;

use crate::ffi::*;
use crate::{media, Chapter, ChapterMut, Codec, DictionaryRef, Stream, StreamMut};

impl_for_many! {
    impl for Input, Output {
        pub fn nb_streams(&self) -> u32 {
            unsafe { (*self.as_ptr()).nb_streams }
        }

        pub fn best_stream(&mut self) -> Best<'_> {
            unsafe { Best::from_raw(self.as_mut_ptr()) }
        }

        pub fn stream(&self, index: usize) -> Option<Stream<'_>> {
            unsafe {
                Stream::from_ctx_and_idx(self, index)
            }
        }

        pub fn stream_mut(&mut self, index: usize) -> Option<StreamMut<'_>> {
            unsafe {
                StreamMut::from_ctx_and_idx(self, index)
            }
        }

        pub fn streams(&self) -> StreamIter<'_> {
            let ptrs = unsafe {
                utils::c_slice_or_empty(
                    (*self.as_ptr()).streams,
                    (*self.as_ptr()).nb_streams as usize,
                )
            };
            StreamIter::from_slice(ptrs)
        }

        pub fn streams_mut(&mut self) -> StreamIterMut<'_> {
            let ptrs = unsafe {
                utils::c_slice_or_empty(
                    (*self.as_ptr()).streams,
                    (*self.as_ptr()).nb_streams as usize,
                )
            };
            StreamIterMut::from_slice(ptrs)
        }

        pub fn bit_rate(&self) -> i64 {
            unsafe { (*self.as_ptr()).bit_rate }
        }

        pub fn duration(&self) -> i64 {
            unsafe { (*self.as_ptr()).duration }
        }

        pub fn nb_chapters(&self) -> u32 {
            unsafe { (*self.as_ptr()).nb_chapters }
        }

        pub fn chapter(&self, index: usize) -> Option<Chapter<'_>>
        {
            unsafe {
                Chapter::from_ctx_and_idx(self, index)
            }
        }

        pub fn chapter_mut(&mut self, index: usize) -> Option<ChapterMut<'_>>
        {
            unsafe {
                ChapterMut::from_ctx_and_idx(self, index)
            }
        }

        pub fn chapters(&self) -> ChapterIter<'_> {
            let ptrs = unsafe {
                utils::c_slice_or_empty(
                    (*self.as_ptr()).chapters,
                    (*self.as_ptr()).nb_chapters as usize,
                )
            };
            ChapterIter::from_slice(ptrs)
        }

        pub fn chapters_mut(&mut self) -> ChapterIterMut<'_> {
            let ptrs = unsafe {
                utils::c_mut_slice_or_empty(
                    (*self.as_mut_ptr()).chapters,
                    (*self.as_mut_ptr()).nb_chapters as usize,
                )
            };
            ChapterIterMut::from_slice(ptrs)
        }

        pub fn metadata(&self) -> DictionaryRef<'_> {
            unsafe { DictionaryRef::from_raw((*self.as_ptr()).metadata) }
        }
    }
}

pub struct Best<'a> {
    ctx: *mut AVFormatContext,
    _marker: PhantomData<&'a mut AVFormatContext>,

    wanted: i32,
    related: i32,
}

impl<'a> Best<'a> {
    /// # Safety
    /// `ctx` must be non-null and valid. Ensure that the returned lifetime
    /// is correctly bounded.
    pub unsafe fn from_raw(ctx: *mut AVFormatContext) -> Self {
        Self {
            ctx,
            _marker: PhantomData,

            wanted: -1,
            related: -1,
        }
    }

    pub fn wanted(mut self, stream: &Stream) -> Self {
        self.wanted = stream.index() as i32;
        self
    }

    pub fn related(mut self, stream: &Stream) -> Self {
        self.related = stream.index() as i32;
        self
    }
}

impl<'a> Best<'a> {
    pub fn find(self, kind: media::Type) -> Option<Stream<'a>> {
        unsafe {
            let index = av_find_best_stream(
                self.ctx,
                kind.into(),
                self.wanted as c_int,
                self.related as c_int,
                ptr::null_mut(),
                0,
            );

            match usize::try_from(index) {
                Ok(index) => Some(
                    Stream::from_raw(*(*self.ctx).streams.add(index as usize))
                        .expect("best stream exists"),
                ),
                Err(_) => None,
            }
        }
    }

    pub fn find_with_decoder(self, kind: media::Type) -> Option<(Stream<'a>, Codec)> {
        unsafe {
            let mut decoder = ptr::null();
            let index = av_find_best_stream(
                self.ctx,
                kind.into(),
                self.wanted as c_int,
                self.related as c_int,
                &mut decoder,
                0,
            );

            match usize::try_from(index) {
                Ok(index) => Some((
                    Stream::from_raw(*(*self.ctx).streams.add(index as usize))
                        .expect("best stream exists"),
                    Codec::from_raw(decoder).expect("decoder_ret is non-null"),
                )),
                Err(_) => None,
            }
        }
    }
}

impl_slice_iter!(StreamIter, Stream, AVStream);
impl_slice_iter!(StreamIterMut, StreamMut, AVStream);

impl_slice_iter!(ChapterIter, Chapter, AVChapter);
impl_slice_iter!(ChapterIterMut, ChapterMut, AVChapter);

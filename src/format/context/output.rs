use std::ffi::CString;
use std::ptr;

use crate::codec::traits;
use crate::ffi::*;
use crate::{format, ChapterMut, DictionaryMut, Error, Rational, StreamMut};
use crate::{AsMutPtr, AsPtr};

pub struct Output {
    ptr: *mut AVFormatContext,
}

unsafe impl Send for Output {}

impl Output {
    pub unsafe fn wrap(ptr: *mut AVFormatContext) -> Self {
        Output { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }
}

impl Output {
    pub fn format(&self) -> format::Output {
        unsafe { format::Output::from_raw((*self.as_ptr()).oformat).expect("oformat is non-null") }
    }

    pub fn write_header(&mut self) -> Result<(), Error> {
        unsafe {
            match avformat_write_header(self.as_mut_ptr(), ptr::null_mut()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn write_header_with<Dict>(&mut self, mut options: Dict) -> Result<Dict, Error>
    where
        Dict: AsMutPtr<*mut AVDictionary>,
    {
        unsafe {
            let res = avformat_write_header(self.as_mut_ptr(), options.as_mut_ptr());

            match res {
                0 => Ok(options),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn write_trailer(&mut self) -> Result<(), Error> {
        unsafe {
            match av_write_trailer(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn add_stream<T, E: traits::Encoder<T>>(
        &mut self,
        codec: E,
    ) -> Result<StreamMut<'_>, Error> {
        unsafe {
            let codec = codec.encoder();
            let codec = codec.map_or(ptr::null(), |c| c.as_ptr());
            let ptr = avformat_new_stream(self.as_mut_ptr(), codec);

            StreamMut::from_raw(ptr).ok_or(Error::Unknown)
        }
    }

    pub fn add_chapter<R: Into<Rational>, S: AsRef<str>>(
        &mut self,
        id: i64,
        time_base: R,
        start: i64,
        end: i64,
        title: S,
    ) -> Result<ChapterMut<'_>, Error> {
        // avpriv_new_chapter is private (libavformat/internal.h)

        if start > end {
            return Err(Error::InvalidData);
        }

        let mut existing = None;
        for (idx, chapter) in self.chapters().enumerate() {
            if chapter.id() == id {
                existing = Some(idx);
                break;
            }
        }

        let index = match existing {
            Some(index) => index,
            None => unsafe {
                let ptr = av_mallocz(size_of::<AVChapter>());
                if ptr.is_null() {
                    return Err(Error::Bug);
                }

                let mut nb_chapters = (*self.as_ptr()).nb_chapters as i32;

                // chapters array will be freed by `avformat_free_context`
                av_dynarray_add(
                    &mut (*self.as_mut_ptr()).chapters as *mut _ as *mut libc::c_void,
                    &mut nb_chapters,
                    ptr,
                );

                if nb_chapters > 0 {
                    (*self.as_mut_ptr()).nb_chapters = nb_chapters as u32;
                    let index = (*self.as_ptr()).nb_chapters - 1;
                    index as usize
                } else {
                    // failed to add the chapter
                    av_freep(ptr);
                    return Err(Error::Bug);
                }
            },
        };

        let mut chapter = self.chapter_mut(index).ok_or(Error::Bug)?;

        chapter.set_id(id);
        chapter.set_time_base(time_base);
        chapter.set_start(start);
        chapter.set_end(end);
        chapter.metadata_mut().set("title", title);

        Ok(chapter)
    }

    pub fn metadata_mut(&mut self) -> DictionaryMut<'_> {
        unsafe { DictionaryMut::from_raw(&mut (*self.as_mut_ptr()).metadata) }
    }
}

impl AsPtr<AVFormatContext> for Output {
    fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr
    }
}

impl AsMutPtr<AVFormatContext> for Output {
    fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }
}

pub fn dump(ctx: &Output, index: i32, url: Option<&str>) {
    let url = url.map(|u| CString::new(u).unwrap());

    unsafe {
        av_dump_format(
            ctx.as_ptr() as *mut _,
            index,
            url.unwrap_or_else(|| CString::new("").unwrap()).as_ptr(),
            1,
        );
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        unsafe {
            avio_close((*self.ptr).pb);
            avformat_free_context(self.ptr);
        }
    }
}

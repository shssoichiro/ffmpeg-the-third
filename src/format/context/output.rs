use std::ffi::{CString, OsStr};
use std::ptr::{self, NonNull};

use crate::codec::traits;
use crate::ffi::*;
use crate::format::Flags;
use crate::{format, ChapterMut, Dictionary, DictionaryMut, Error, Rational, StreamMut};
use crate::{AsMutPtr, AsPtr};

pub struct Output {
    ptr: NonNull<AVFormatContext>,
}

unsafe impl Send for Output {}

impl Output {
    pub unsafe fn from_raw(ptr: *mut AVFormatContext) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    pub fn from_format(format: format::Output) -> Result<Self, Error> {
        OutputFormat::Format(format).create_output()
    }

    pub fn from_format_name(format_name: impl AsRef<str>) -> Result<Self, Error> {
        let format_name = CString::new(format_name.as_ref()).unwrap();
        OutputFormat::Name(format_name).create_output()
    }

    pub fn from_filename(filename: impl AsRef<OsStr>) -> Result<Self, Error> {
        let filename = CString::new(filename.as_ref().as_encoded_bytes()).unwrap();
        OutputFormat::Filename(filename).create_output()
    }

    pub fn open_file(&mut self, filename: impl AsRef<OsStr>) -> Result<(), Error> {
        self.open_file_with(filename, Dictionary::new())
    }

    pub fn open_file_with<P, D>(&mut self, filename: P, mut options: D) -> Result<(), Error>
    where
        P: AsRef<OsStr>,
        D: AsMutPtr<*mut AVDictionary>,
    {
        if self.format().flags().contains(Flags::NO_FILE) {
            // The demuxer handles IO itself
            return Err(Error::InvalidData);
        }

        let res = unsafe {
            let filename = CString::new(filename.as_ref().as_encoded_bytes()).unwrap();

            avio_open2(
                &mut (*self.as_mut_ptr()).pb,
                filename.as_ptr(),
                AVIO_FLAG_WRITE,
                ptr::null(),
                options.as_mut_ptr(),
            )
        };

        if res >= 0 {
            Ok(())
        } else {
            Err(Error::from(res))
        }
    }

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
        self.ptr.as_ptr()
    }
}

impl AsMutPtr<AVFormatContext> for Output {
    fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr.as_ptr()
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
            avio_close((*self.as_mut_ptr()).pb);
            avformat_free_context(self.as_mut_ptr());
        }
    }
}

#[derive(Debug)]
enum OutputFormat {
    Format(format::Output),
    Name(CString),
    Filename(CString),
}

impl OutputFormat {
    pub fn create_output(self) -> Result<Output, Error> {
        let mut ctx = ptr::null_mut();

        unsafe {
            let oformat = if let OutputFormat::Format(fmt) = &self {
                fmt.as_ptr()
            } else {
                ptr::null()
            };

            let format_name = if let OutputFormat::Name(format_name) = &self {
                format_name.as_ptr()
            } else {
                ptr::null()
            };

            let filename = if let OutputFormat::Filename(filename) = &self {
                filename.as_ptr()
            } else {
                ptr::null()
            };

            let ret = avformat_alloc_output_context2(&mut ctx, oformat, format_name, filename);

            // ensure `self` is not dropped before this so the char pointers stay valid
            let _ = self;

            if ret >= 0 {
                Ok(Output::from_raw(ctx).expect("ctx is non-null"))
            } else {
                Err(Error::from(ret))
            }
        }
    }
}

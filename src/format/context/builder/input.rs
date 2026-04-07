use std::ffi::{CString, OsStr};
use std::ptr;

use crate::ffi::*;
use crate::format;
use crate::format::context;
use crate::{DictionaryMut, Error};

pub struct InputBuilder<'d> {
    url: CString,
    fmt: *const AVInputFormat,
    options: Option<DictionaryMut<'d>>,
}

impl<'d> InputBuilder<'d> {
    pub fn new(url: impl AsRef<OsStr>) -> Self {
        let url = CString::new(url.as_ref().as_encoded_bytes()).unwrap();

        Self {
            url,
            fmt: ptr::null(),
            options: None,
        }
    }

    pub fn with_forced_format(mut self, format: format::Input) -> Self {
        self.fmt = format.as_ptr();
        self
    }

    pub fn with_options(mut self, options: DictionaryMut<'d>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn build(self) -> Result<context::Input, Error> {
        let mut ps = ptr::null_mut();
        unsafe {
            let options = self.options.map_or(ptr::null_mut(), |mut o| o.as_mut_ptr());

            // SAFETY:
            // - &mut ps can never be coerced to a nullptr
            // - self.url.as_ptr (CString) can never be null
            // - self.fmt may be null
            // - options may be null
            let ret = avformat_open_input(&mut ps, self.url.as_ptr(), self.fmt, options);
            if ret == 0 {
                let ret = avformat_find_stream_info(ps, ptr::null_mut());
                if ret >= 0 {
                    Ok(context::Input::from_raw(ps).expect("ps is non-null"))
                } else {
                    avformat_close_input(&mut ps);
                    Err(Error::from(ret))
                }
            } else {
                Err(Error::from(ret))
            }
        }
    }
}

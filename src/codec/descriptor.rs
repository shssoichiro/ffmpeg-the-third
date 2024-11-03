use std::ffi::CStr;
use std::ptr::NonNull;
use std::str::from_utf8_unchecked;

use crate::ffi::*;
use crate::media;

use super::profile::ProfileIter;
use super::{CodecProperties, Id};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodecDescriptor {
    ptr: NonNull<AVCodecDescriptor>,
}

impl CodecDescriptor {
    pub unsafe fn from_raw(ptr: *const AVCodecDescriptor) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self { ptr })
    }

    pub fn as_ptr(self) -> *const AVCodecDescriptor {
        self.ptr.as_ptr()
    }

    pub fn id(self) -> Id {
        unsafe { (*self.as_ptr()).id.into() }
    }

    pub fn kind(self) -> media::Type {
        unsafe { (*self.as_ptr()).type_.into() }
    }

    pub fn name(self) -> &'static str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    pub fn description(self) -> Option<&'static str> {
        unsafe {
            let long_name = (*self.as_ptr()).long_name;
            if long_name.is_null() {
                None
            } else {
                Some(from_utf8_unchecked(CStr::from_ptr(long_name).to_bytes()))
            }
        }
    }

    pub fn props(self) -> CodecProperties {
        unsafe { CodecProperties::from_bits_truncate((*self.as_ptr()).props) }
    }

    pub fn mime_types(self) -> Option<MimeTypeIter> {
        unsafe { MimeTypeIter::from_raw((*self.as_ptr()).mime_types) }
    }

    pub fn profiles(self) -> Option<ProfileIter> {
        unsafe {
            if (*self.as_ptr()).profiles.is_null() {
                None
            } else {
                Some(ProfileIter::new(self.id(), (*self.as_ptr()).profiles))
            }
        }
    }
}

pub struct CodecDescriptorIter {
    ptr: *const AVCodecDescriptor,
}

impl CodecDescriptorIter {
    pub fn new() -> Self {
        Self {
            ptr: std::ptr::null(),
        }
    }
}

impl Default for CodecDescriptorIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for CodecDescriptorIter {
    type Item = CodecDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let next = avcodec_descriptor_next(self.ptr);
            if let Some(desc) = CodecDescriptor::from_raw(next) {
                self.ptr = next;
                Some(desc)
            } else {
                None
            }
        }
    }
}

pub struct MimeTypeIter {
    ptr: NonNull<*const libc::c_char>,
}

impl MimeTypeIter {
    pub unsafe fn from_raw(ptr: *const *const libc::c_char) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self { ptr })
    }
}

impl Iterator for MimeTypeIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let next = *self.ptr.as_ptr();
            if next.is_null() {
                return None;
            }

            self.ptr = NonNull::new_unchecked(self.ptr.as_ptr().add(1));
            Some(from_utf8_unchecked(CStr::from_ptr(next).to_bytes()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::decoder::find;

    #[test]
    fn descriptor() {
        let targa = find(Id::TARGA).expect("can find targa decoder");
        let desc = targa.descriptor().expect("targa has descriptor");
        assert_eq!(desc.id(), Id::TARGA);
        assert_eq!(desc.kind(), media::Type::Video);
        assert_eq!(desc.name(), "targa");

        // --enable-small will remove all `long_name`s. So this can either be null/None
        // or the correct description
        assert!(matches!(
            desc.description(),
            None | Some("Truevision Targa image")
        ));

        let props = desc.props();
        assert!(
            props.contains(CodecProperties::INTRA_ONLY)
                && props.contains(CodecProperties::LOSSLESS)
        );

        let mut mime_types = desc.mime_types().expect("has mime types");
        assert_eq!(mime_types.next(), Some("image/x-targa"));
        assert_eq!(mime_types.next(), Some("image/x-tga"));
        assert_eq!(mime_types.next(), None);
    }
}

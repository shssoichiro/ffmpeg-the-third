use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;

use crate::ffi::*;
use crate::utils;

pub struct Iter<'a> {
    ptr: *const AVDictionary,
    cur: *mut AVDictionaryEntry,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Iter<'a> {
    pub fn new(dictionary: *const AVDictionary) -> Self {
        Iter {
            ptr: dictionary,
            cur: ptr::null_mut(),

            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let empty = CString::new("").unwrap();
            let entry = av_dict_get(self.ptr, empty.as_ptr(), self.cur, AV_DICT_IGNORE_SUFFIX);

            if !entry.is_null() {
                let key = utils::str_from_c_ptr((*entry).key);
                let val = utils::str_from_c_ptr((*entry).value);

                self.cur = entry;

                Some((key, val))
            } else {
                None
            }
        }
    }
}

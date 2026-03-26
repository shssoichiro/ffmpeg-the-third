use std::ffi::CString;

use super::{Dictionary, DictionaryMut, DictionaryRef};
use super::{Flags, Iter};
use crate::ffi::*;
use crate::macros::impl_for_many;
use crate::utils;

pub fn set(dict: &mut *mut AVDictionary, key: &str, value: &str, flags: Flags) {
    let key = CString::new(key).unwrap();
    let value = CString::new(value).unwrap();

    unsafe {
        if av_dict_set(dict, key.as_ptr(), value.as_ptr(), flags.bits()) < 0 {
            panic!("out of memory");
        }
    }
}

pub fn unset(dict: &mut *mut AVDictionary, key: &str, flags: Flags) {
    let key = CString::new(key).unwrap();

    unsafe {
        av_dict_set(dict, key.as_ptr(), std::ptr::null(), flags.bits());
    }
}

// Safety: Ensure the returned lifetime 'd is bounded to a borrow on dict
pub unsafe fn get<'d>(dict: *const AVDictionary, key: &str, flags: Flags) -> Option<&'d str> {
    let key = CString::new(key).unwrap();
    unsafe {
        let entry = av_dict_get(dict, key.as_ptr(), std::ptr::null_mut(), flags.bits());

        if entry.is_null() {
            None
        } else {
            Some(utils::str_from_c_ptr((*entry).value))
        }
    }
}

impl_for_many! {
    impl for Dictionary, DictionaryRef<'a>, DictionaryMut<'a> {
        /// Try to find a value in the dictionary.
        ///
        /// This function uses case-insensitive matching of the entire key string
        /// to find a value. If you want to customize the way FFmpeg searches
        /// for the key, see [`get_with_flags`][Self::get_with_flags].
        pub fn get<K: AsRef<str>>(&self, key: K) -> Option<&str> {
            self.get_with_flags(key, Flags::empty())
        }

        /// Try to find a value in the dictionary, using custom search flags.
        ///
        /// See [Flags][crate::dictionary::Flags] to see how each flag works.
        /// Using [`Flags::DONT_STRDUP_KEY`] is heavily discouraged unless you
        /// know what you are doing.
        pub fn get_with_flags<K: AsRef<str>>(&self, key: K, flags: Flags) -> Option<&str> {
            // SAFETY: Returned lifetime is bounded by borrow on self
            unsafe { get(self.as_ptr(), key.as_ref(), flags) }
        }

        /// Returns the number of entries in the dictionary.
        pub fn len(&self) -> usize {
            unsafe { av_dict_count(self.as_ptr()) as usize }
        }

        /// Returns `true` if the dictionary is empty.
        pub fn is_empty(&self) -> bool {
            self.as_ptr().is_null()
        }

        /// Creates an iterator over all key-value pairs in the dictionary.
        pub fn iter(&self) -> Iter<'_> {
            Iter::new(self.as_ptr())
        }
    }
}

impl_for_many! {
    impl for Dictionary, DictionaryMut<'a> {
        /// Set a value for the given key.
        ///
        /// This function will overwrite any value that already exists
        /// for the given key. If you want to customize the way FFmpeg inserts
        /// the new value, see [`set_with_flags`][Self::set_with_flags].
        pub fn set<K, V>(&mut self, key: K, value: V)
        where
            K: AsRef<str>,
            V: AsRef<str>,
        {
            self.set_with_flags(key, value, Flags::empty())
        }

        /// Set a value for the given key, using custom flags.
        ///
        /// See [Flags][crate::dictionary::Flags] to see how each flag works.
        /// Using [`Flags::DONT_STRDUP_KEY`] or [`Flags::DONT_STRDUP_VAL`] is
        /// heavily discouraged unless you know what you are doing.
        pub fn set_with_flags<K, V>(&mut self, key: K, value: V, flags: Flags)
        where
            K: AsRef<str>,
            V: AsRef<str>,
        {
            set(self.as_mut_ptr(), key.as_ref(), value.as_ref(), flags)
        }

        /// Remove a value from the dictionary for the given key.
        ///
        /// If you want to customize the way FFmpeg searches for the key,
        /// see [`unset_with_flags`][Self::unset_with_flags].
        pub fn unset<K: AsRef<str>>(&mut self, key: K) {
            self.unset_with_flags(key, Flags::empty());
        }

        /// Remove a value from the dictionary for the given key, using custom flags.
        ///
        /// See [Flags][crate::dictionary::Flags] to see how each flag works.
        /// Using [`Flags::DONT_STRDUP_KEY`] or [`Flags::DONT_STRDUP_VAL`] is
        /// heavily discouraged unless you know what you are doing.
        pub fn unset_with_flags<K: AsRef<str>>(&mut self, key: K, flags: Flags) {
            unset(self.as_mut_ptr(), key.as_ref(), flags);
        }
    }
}

impl_for_many! {
    impl for DictionaryRef<'d>, DictionaryMut<'d> {
        /// Clones the borrowed data into an owned [Dictionary].
        pub fn to_owned(&self) -> Dictionary {
            self.iter().collect()
        }
    }
}

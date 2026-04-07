//! Internal utils, not related to `avutil`

use std::ffi::CStr;

/// `ptr` must be non-null and valid.
/// Ensure that the returned lifetime is correctly bounded.
#[inline]
pub unsafe fn str_from_c_ptr<'s>(ptr: *const libc::c_char) -> &'s str {
    unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) }
}

/// `ptr` must be null or valid.
/// Ensure that the returned lifetime is correctly bounded.
#[inline]
pub unsafe fn optional_str_from_c_ptr<'s>(ptr: *const libc::c_char) -> Option<&'s str> {
    if ptr.is_null() {
        None
    } else {
        Some(str_from_c_ptr(ptr))
    }
}

/// If `ptr` is null, return an empty slice. Otherwise return the slice
/// as described by `ptr` and `len`.
///
/// # Safety
/// `ptr` and `len` must fulfill the invariants for
/// [`from_raw_parts`][std::slice::from_raw_parts].
///
/// Ensure the returned lifetime is correctly bounded
#[inline]
pub unsafe fn c_slice_or_empty<'s, T>(ptr: *const T, len: usize) -> &'s [T] {
    if ptr.is_null() {
        &[]
    } else {
        std::slice::from_raw_parts(ptr, len)
    }
}

/// If `ptr` is null, return an empty slice. Otherwise return the mutable
/// slice as described by `ptr` and `len`.
///
/// # Safety
/// `ptr` and `len` must fulfill the invariants for
/// [`from_raw_parts_mut`][std::slice::from_raw_parts_mut].
///
/// Ensure the returned lifetime is correctly bounded.
#[inline]
pub unsafe fn c_mut_slice_or_empty<'s, T>(ptr: *mut T, len: usize) -> &'s mut [T] {
    if ptr.is_null() {
        &mut []
    } else {
        std::slice::from_raw_parts_mut(ptr, len)
    }
}

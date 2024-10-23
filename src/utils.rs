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

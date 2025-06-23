use std::ptr::NonNull;

/// Pointer-based iterator, stepped through via pointer arithmetic and ended
/// when a dereferenced pointer equals a terminator value.
pub(crate) trait TerminatedPtrIter<AVType, WrapperType>:
    Sized + Iterator<Item = WrapperType>
{
    /// Create a new iterator from a non-null pointer to any value in the iteration.
    ///
    /// # Safety
    ///
    /// `ptr` and all following pointers must be dereferenceable until the terminator is reached.
    unsafe fn from_ptr(ptr: NonNull<AVType>) -> Self;

    /// Create a new iterator from a pointer to any value in the iteration.
    ///
    /// Returns `None` if `ptr` is null. See also [`from_ptr`][TerminatedPtrIter::from_ptr].
    ///
    /// # Safety
    ///
    /// See [`from_ptr`][TerminatedPtrIter::from_ptr].
    unsafe fn from_raw(ptr: *const AVType) -> Option<Self> {
        unsafe { NonNull::new(ptr as *mut _).map(|ptr| Self::from_ptr(ptr)) }
    }
}

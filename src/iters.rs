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

macro_rules! impl_slice_iter {
    (
        $iter:ident,
        $ty:ident,
        $av_ty:ty
    ) => {
        pub struct $iter<'s> {
            raw_iter: ::std::slice::Iter<'s, *mut $av_ty>,
        }

        impl<'s> $iter<'s> {
            pub fn from_slice(slice: &'s [*mut $av_ty]) -> Self {
                Self {
                    raw_iter: slice.iter(),
                }
            }
        }

        impl<'s> Iterator for $iter<'s> {
            type Item = $ty<'s>;

            fn next(&mut self) -> Option<Self::Item> {
                unsafe {
                    // SAFETY: Lifetime is bounded by Self::Item
                    self.raw_iter.next().and_then(|&ptr| $ty::from_raw(ptr))
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.raw_iter.size_hint()
            }
        }

        impl<'s> DoubleEndedIterator for $iter<'s> {
            fn next_back(&mut self) -> Option<Self::Item> {
                unsafe {
                    // SAFETY: Lifetime is bounded by Self::Item
                    self.raw_iter
                        .next_back()
                        .and_then(|&ptr| $ty::from_raw(ptr))
                }
            }
        }

        impl<'s> ExactSizeIterator for $iter<'s> {}
        impl<'s> ::std::iter::FusedIterator for $iter<'s> {}
    };
}

pub(crate) use impl_slice_iter;

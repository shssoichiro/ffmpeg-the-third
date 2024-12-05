pub trait AsPtr<T> {
    /// Returns a *const raw pointer to the underlying FFmpeg type.
    fn as_ptr(&self) -> *const T;
}

pub trait AsMutPtr<T> {
    /// Returns a *mut raw pointer to the underlying FFmpeg type.
    fn as_mut_ptr(&mut self) -> *mut T;
}

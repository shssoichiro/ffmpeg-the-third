use std::panic;
use std::process;
use std::sync::Mutex;

use crate::ffi::*;
use libc::{c_int, c_void};

type Callback = Mutex<Box<dyn FnMut() -> bool + Send>>;

/// An owned interrupt callback for an `AVFormatContext`.
///
/// `interrupt` points into a heap allocation owned by this value, so it stays valid when the
/// `Interrupt` is moved, but becomes dangling once the `Interrupt` is dropped. Keep it alive
/// until FFmpeg can no longer invoke the callback, i.e. until the format context it was
/// installed on has been closed.
pub(crate) struct Interrupt {
    pub(crate) interrupt: AVIOInterruptCB,
    closure: *mut Callback,
}

// SAFETY: `closure` is uniquely owned by this value and points to a `Mutex` wrapping a `Send`
// closure. `interrupt` only refers to that allocation.
unsafe impl Send for Interrupt {}

impl Drop for Interrupt {
    fn drop(&mut self) {
        // SAFETY: `closure` was created by `Box::into_raw` in `new` and is freed only here.
        unsafe { drop(Box::from_raw(self.closure)) };
    }
}

extern "C" fn callback(opaque: *mut c_void) -> c_int {
    // SAFETY: `opaque` is the `closure` pointer of a live `Interrupt`. Only shared references
    // are created from it; the `Mutex` serializes calls from different threads.
    let closure = unsafe { &*(opaque as *const Callback) };
    match panic::catch_unwind(panic::AssertUnwindSafe(|| (closure.lock().unwrap())())) {
        Ok(ret) => ret as c_int,
        Err(_) => process::abort(),
    }
}

/// Creates an interrupt callback that FFmpeg may invoke from any thread.
///
/// Calls to `opaque` are serialized. If it panics, the process is aborted.
pub(crate) fn new<F>(opaque: Box<F>) -> Interrupt
where
    F: FnMut() -> bool + Send + 'static,
{
    let opaque: Box<dyn FnMut() -> bool + Send> = opaque;
    let closure: *mut Callback = Box::into_raw(Box::new(Mutex::new(opaque)));
    let interrupt_cb = AVIOInterruptCB {
        callback: Some(callback),
        opaque: closure as *mut c_void,
    };
    Interrupt {
        interrupt: interrupt_cb,
        closure,
    }
}

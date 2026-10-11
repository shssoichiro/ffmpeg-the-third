use crate::ffi::*;
use crate::util::interrupt::Interrupt;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Mode {
    Input,
    Output,
}

pub struct Destructor {
    ptr: *mut AVFormatContext,
    mode: Mode,
    // Dropped after `drop` has closed the context, because FFmpeg may invoke the callback
    // while closing. Kept here rather than in `Input` so that it moves with the context.
    interrupt: Option<Interrupt>,
}

impl Destructor {
    pub unsafe fn new(ptr: *mut AVFormatContext, mode: Mode) -> Self {
        Destructor {
            ptr,
            mode,
            interrupt: None,
        }
    }

    pub(crate) fn retain_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupt = Some(interrupt);
    }
}

unsafe impl Send for Destructor {}
unsafe impl Sync for Destructor {}

impl Drop for Destructor {
    fn drop(&mut self) {
        unsafe {
            match self.mode {
                Mode::Input => avformat_close_input(&mut self.ptr),

                Mode::Output => {
                    avio_close((*self.ptr).pb);
                    avformat_free_context(self.ptr);
                }
            }
        }
    }
}

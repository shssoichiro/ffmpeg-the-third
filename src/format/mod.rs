#[cfg(feature = "ffmpeg_8_1")]
pub use crate::util::format::AlphaMode;
pub use crate::util::format::{pixel, Pixel};
pub use crate::util::format::{sample, Sample};

use crate::util::interrupt;

pub mod stream;

pub mod chapter;

pub mod context;
pub use self::context::Context;

pub mod format;
pub use self::format::{flag, Flags};
pub use self::format::{Input, Output};

pub mod network;

use std::ffi::{CString, OsStr};
use std::ptr;

use crate::ffi::*;
use crate::utils;
use crate::{AsMutPtr, Error};

pub fn version() -> u32 {
    unsafe { avformat_version() }
}

pub fn configuration() -> &'static str {
    unsafe { utils::str_from_c_ptr(avformat_configuration()) }
}

pub fn license() -> &'static str {
    unsafe { utils::str_from_c_ptr(avformat_license()) }
}

pub fn input<P: AsRef<OsStr>>(path_or_url: P) -> Result<context::Input, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_os_str(path_or_url);

        match avformat_open_input(&mut ps, path.as_ptr(), ptr::null_mut(), ptr::null_mut()) {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap(ps)),
                e => {
                    avformat_close_input(&mut ps);
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn input_with_dictionary<P, Dict>(
    path_or_url: P,
    mut options: Dict,
) -> Result<context::Input, Error>
where
    Dict: AsMutPtr<*mut AVDictionary>,
    P: AsRef<OsStr>,
{
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_os_str(path_or_url);
        let res = avformat_open_input(
            &mut ps,
            path.as_ptr(),
            ptr::null_mut(),
            options.as_mut_ptr(),
        );

        match res {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap(ps)),
                e => {
                    avformat_close_input(&mut ps);
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

/// Opens an input with an interrupt callback, then probes its streams.
///
/// FFmpeg may invoke `closure` from any thread that performs I/O on the input, and calls are
/// serialized. Returning `true` aborts the blocking operation in progress. The callback is
/// kept alive until the input has been closed. If it panics, the process is aborted.
pub fn input_with_interrupt<P, F>(path_or_url: P, closure: F) -> Result<context::Input, Error>
where
    P: AsRef<OsStr>,
    F: FnMut() -> bool + Send + 'static,
{
    unsafe {
        let path = from_os_str(path_or_url);
        let mut ps = avformat_alloc_context();
        if ps.is_null() {
            return Err(Error::Other {
                errno: libc::ENOMEM,
            });
        }
        let interrupt = interrupt::new(Box::new(closure));
        (*ps).interrupt_callback = interrupt.interrupt;

        match avformat_open_input(&mut ps, path.as_ptr(), ptr::null_mut(), ptr::null_mut()) {
            0 => {
                // Dropping the input closes the context before the callback is freed.
                let mut input = context::Input::wrap(ps);
                input.retain_interrupt(interrupt);
                match avformat_find_stream_info(input.as_mut_ptr(), ptr::null_mut()) {
                    r if r >= 0 => Ok(input),
                    e => Err(Error::from(e)),
                }
            }

            e => Err(Error::from(e)),
        }
    }
}

fn from_os_str(path_or_url: impl AsRef<OsStr>) -> CString {
    CString::new(path_or_url.as_ref().as_encoded_bytes()).unwrap()
}

fn alloc_context(
    format_name: *const libc::c_char,
    filename: *const libc::c_char,
) -> Result<context::Output, Error> {
    let mut ps = ptr::null_mut();

    unsafe {
        let res = avformat_alloc_output_context2(&mut ps, ptr::null(), format_name, filename);
        if res >= 0 {
            Ok(context::Output::wrap(ps))
        } else {
            Err(Error::from(res))
        }
    }
}

fn open_context_write(
    ctx: &mut context::Output,
    filename: *const libc::c_char,
    opts: *mut *mut AVDictionary,
) -> Result<(), Error> {
    let res = unsafe {
        avio_open2(
            &mut (*ctx.as_mut_ptr()).pb,
            filename,
            AVIO_FLAG_WRITE,
            ptr::null(),
            opts,
        )
    };

    if res >= 0 {
        Ok(())
    } else {
        Err(Error::from(res))
    }
}

pub fn output<P: AsRef<OsStr>>(path_or_url: P) -> Result<context::Output, Error> {
    let filename = from_os_str(path_or_url);
    let mut ctx = alloc_context(ptr::null(), filename.as_ptr())?;

    if !ctx.format().flags().contains(Flags::NO_FILE) {
        open_context_write(&mut ctx, filename.as_ptr(), ptr::null_mut())?;
    }

    Ok(ctx)
}

pub fn output_with<P, Dict>(path_or_url: P, mut options: Dict) -> Result<context::Output, Error>
where
    P: AsRef<OsStr>,
    Dict: AsMutPtr<*mut AVDictionary>,
{
    let path = from_os_str(path_or_url);
    let mut ctx = alloc_context(ptr::null(), path.as_ptr())?;

    if !ctx.format().flags().contains(Flags::NO_FILE) {
        open_context_write(&mut ctx, path.as_ptr(), options.as_mut_ptr())?;
    }

    Ok(ctx)
}

pub fn output_as<P: AsRef<OsStr>>(path_or_url: P, format: &str) -> Result<context::Output, Error> {
    let path = from_os_str(path_or_url);
    let format = CString::new(format).unwrap();
    let mut ctx = alloc_context(format.as_ptr(), path.as_ptr())?;

    if !ctx.format().flags().contains(Flags::NO_FILE) {
        open_context_write(&mut ctx, path.as_ptr(), ptr::null_mut())?;
    }

    Ok(ctx)
}

pub fn output_as_with<P, Dict>(
    path_or_url: P,
    format: &str,
    mut options: Dict,
) -> Result<context::Output, Error>
where
    P: AsRef<OsStr>,
    Dict: AsMutPtr<*mut AVDictionary>,
{
    let path = from_os_str(path_or_url);
    let format = CString::new(format).unwrap();
    let mut ctx = alloc_context(format.as_ptr(), path.as_ptr())?;

    if !ctx.format().flags().contains(Flags::NO_FILE) {
        open_context_write(&mut ctx, path.as_ptr(), options.as_mut_ptr())?;
    }

    Ok(ctx)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::{env, fs, process};

    use super::*;

    /// Writes 0.1 s of silent 8 kHz mono 16-bit PCM WAV, which every FFmpeg build can demux.
    fn write_wav(name: &str) -> PathBuf {
        let data_len: u32 = 1600;
        let mut bytes = Vec::with_capacity(44 + data_len as usize);
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(36 + data_len).to_le_bytes());
        bytes.extend_from_slice(b"WAVEfmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes()); // PCM
        bytes.extend_from_slice(&1u16.to_le_bytes()); // channels
        bytes.extend_from_slice(&8000u32.to_le_bytes()); // sample rate
        bytes.extend_from_slice(&16000u32.to_le_bytes()); // byte rate
        bytes.extend_from_slice(&2u16.to_le_bytes()); // block align
        bytes.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_len.to_le_bytes());
        bytes.resize(44 + data_len as usize, 0);

        let path = env::temp_dir().join(format!("ffmpeg-the-third-{}-{name}.wav", process::id()));
        fs::write(&path, bytes).unwrap();
        path
    }

    /// Sets its flag when dropped, to observe when the interrupt closure is freed.
    struct DropFlag(Arc<AtomicBool>);

    impl Drop for DropFlag {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    #[test]
    fn interrupt_callback_lives_until_input_is_closed() {
        let path = write_wav("interrupt-close");
        let calls = Arc::new(AtomicUsize::new(0));
        let dropped = Arc::new(AtomicBool::new(false));
        let flag = DropFlag(dropped.clone());
        let counter = calls.clone();

        let input = input_with_interrupt(&path, move || {
            let _ = &flag;
            counter.fetch_add(1, Ordering::SeqCst);
            false
        })
        .unwrap();
        assert!(calls.load(Ordering::SeqCst) > 0);
        assert!(!dropped.load(Ordering::SeqCst));

        drop(input);
        assert!(dropped.load(Ordering::SeqCst));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn interrupted_open_fails_and_frees_callback() {
        let path = write_wav("interrupt-abort");
        let dropped = Arc::new(AtomicBool::new(false));
        let flag = DropFlag(dropped.clone());

        let result = input_with_interrupt(&path, move || {
            let _ = &flag;
            true
        });
        assert!(matches!(result, Err(Error::Exit)));
        assert!(dropped.load(Ordering::SeqCst));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn interrupt_callback_moves_with_its_context() {
        let path = write_wav("interrupt-swap");
        let dropped = Arc::new(AtomicBool::new(false));
        let flag = DropFlag(dropped.clone());

        let mut a = input_with_interrupt(&path, move || {
            let _ = &flag;
            false
        })
        .unwrap();
        let mut b = input_with_interrupt(&path, || false).unwrap();

        // `b` now owns the context that was opened with the flagged callback.
        std::mem::swap(&mut *a, &mut *b);
        drop(a);
        let freed_early = dropped.load(Ordering::SeqCst);
        if freed_early {
            // Closing `b` would invoke the freed callback.
            std::mem::forget(b);
            panic!("the callback was freed while its context was still open");
        }

        drop(b);
        assert!(dropped.load(Ordering::SeqCst));
        fs::remove_file(path).unwrap();
    }
}

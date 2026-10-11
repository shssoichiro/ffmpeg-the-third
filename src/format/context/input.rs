use std::ffi::CString;
use std::mem;
use std::ops::{Bound, Deref, DerefMut, RangeBounds};

use super::common::Context;
use super::destructor;
use crate::ffi::*;
use crate::{format, Error, Packet, Stream};

pub struct Input {
    ptr: *mut AVFormatContext,
    ctx: Context,
}

unsafe impl Send for Input {}

impl Input {
    /// Discover stream information using a separate copy of `options` for each
    /// stream present when probing starts (for example, `threads=1`).
    /// Streams discovered during probing use FFmpeg's defaults.
    pub fn find_stream_info(&mut self, options: &crate::Dictionary) -> Result<(), Error> {
        let mut dictionaries = vec![options.clone(); self.nb_streams() as usize];
        let mut pointers: Vec<_> = dictionaries
            .iter_mut()
            .map(|dict| *dict.as_mut_ptr())
            .collect();
        // FFmpeg may replace each dictionary pointer. Restore ownership of the
        // returned dictionaries before any fallible Rust work or error return.
        let options_ptr = if pointers.is_empty() {
            std::ptr::null_mut()
        } else {
            pointers.as_mut_ptr()
        };
        let result = unsafe { avformat_find_stream_info(self.as_mut_ptr(), options_ptr) };
        for (dictionary, pointer) in dictionaries.iter_mut().zip(pointers) {
            *dictionary.as_mut_ptr() = pointer;
        }
        if result < 0 {
            Err(Error::from(result))
        } else {
            Ok(())
        }
    }

    pub unsafe fn wrap(ptr: *mut AVFormatContext) -> Self {
        Input {
            ptr,
            ctx: Context::wrap(ptr, destructor::Mode::Input),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }
}

impl Input {
    pub fn format(&self) -> format::Input {
        unsafe { format::Input::from_raw((*self.as_ptr()).iformat).expect("iformat is non-null") }
    }

    pub fn probe_score(&self) -> i32 {
        unsafe { (*self.as_ptr()).probe_score }
    }

    pub fn packets(&mut self) -> PacketIter<'_> {
        PacketIter::new(self)
    }

    pub fn pause(&mut self) -> Result<(), Error> {
        unsafe {
            match av_read_pause(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn play(&mut self) -> Result<(), Error> {
        unsafe {
            match av_read_play(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn seek<R: RangeBounds<i64>>(&mut self, ts: i64, range: R) -> Result<(), Error> {
        unsafe {
            let start = match range.start_bound().cloned() {
                Bound::Included(i) => i,
                Bound::Excluded(i) => i.saturating_add(1),
                Bound::Unbounded => i64::MIN,
            };

            let end = match range.end_bound().cloned() {
                Bound::Included(i) => i,
                Bound::Excluded(i) => i.saturating_sub(1),
                Bound::Unbounded => i64::MAX,
            };

            match avformat_seek_file(self.as_mut_ptr(), -1, start, ts, end, 0) {
                s if s >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

impl Deref for Input {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

pub struct PacketIter<'a> {
    context: &'a mut Input,
}

impl<'a> PacketIter<'a> {
    pub fn new(context: &mut Input) -> PacketIter<'_> {
        PacketIter { context }
    }
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = Result<(Stream<'a>, Packet), Error>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut packet = Packet::empty();

        match packet.read(self.context) {
            Ok(..) => unsafe {
                Some(Ok((
                    Stream::wrap(mem::transmute_copy(&self.context), packet.stream()),
                    packet,
                )))
            },

            Err(Error::Eof) => None,

            Err(e) => Some(Err(e)),
        }
    }
}

pub fn dump(ctx: &Input, index: i32, url: Option<&str>) {
    let url = url.map(|u| CString::new(u).unwrap());

    unsafe {
        av_dump_format(
            ctx.as_ptr() as *mut _,
            index,
            url.unwrap_or_else(|| CString::new("").unwrap()).as_ptr(),
            0,
        );
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{env, fs, process, ptr};

    use super::*;
    use crate::{media, Dictionary};

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
    #[test]
    fn find_stream_info_with_options() {
        let path = write_wav("find-stream-info");
        let c_path = CString::new(path.to_str().unwrap()).unwrap();
        let mut ps = ptr::null_mut();
        let opened = unsafe {
            avformat_open_input(&mut ps, c_path.as_ptr(), ptr::null_mut(), ptr::null_mut())
        };
        assert_eq!(opened, 0);
        let mut input = unsafe { Input::wrap(ps) };

        let mut options = Dictionary::new();
        options.set("threads", "1");
        input.find_stream_info(&options).unwrap();

        // The caller's dictionary is copied for each stream, not consumed.
        assert_eq!(options.get("threads"), Some("1"));
        assert_eq!(input.nb_streams(), 1);
        let stream = input.streams().next().unwrap();
        assert_eq!(stream.parameters().medium(), media::Type::Audio);

        drop(input);
        fs::remove_file(path).unwrap();
    }
}

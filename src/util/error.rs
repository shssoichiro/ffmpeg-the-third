use std::error;
use std::ffi::CStr;
use std::fmt;
use std::io;
use std::str::from_utf8_unchecked;

use crate::ffi::*;
use libc::{c_char, c_int};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Error {
    Bug,
    Bug2,
    Unknown,
    Experimental,
    BufferTooSmall,
    Eof,
    Exit,
    External,
    InvalidData,
    PatchWelcome,

    InputChanged,
    OutputChanged,

    BsfNotFound,
    DecoderNotFound,
    DemuxerNotFound,
    EncoderNotFound,
    OptionNotFound,
    MuxerNotFound,
    FilterNotFound,
    ProtocolNotFound,
    StreamNotFound,

    HttpBadRequest,
    HttpUnauthorized,
    HttpForbidden,
    HttpNotFound,
    HttpOther4xx,
    HttpServerError,

    /// For AVERROR(e) wrapping POSIX error codes, e.g. AVERROR(EAGAIN).
    Other {
        errno: c_int,
    },
}

impl From<c_int> for Error {
    fn from(value: c_int) -> Error {
        match value {
            AVERROR_BSF_NOT_FOUND => Error::BsfNotFound,
            AVERROR_BUG => Error::Bug,
            AVERROR_BUFFER_TOO_SMALL => Error::BufferTooSmall,
            AVERROR_DECODER_NOT_FOUND => Error::DecoderNotFound,
            AVERROR_DEMUXER_NOT_FOUND => Error::DemuxerNotFound,
            AVERROR_ENCODER_NOT_FOUND => Error::EncoderNotFound,
            AVERROR_EOF => Error::Eof,
            AVERROR_EXIT => Error::Exit,
            AVERROR_EXTERNAL => Error::External,
            AVERROR_FILTER_NOT_FOUND => Error::FilterNotFound,
            AVERROR_INVALIDDATA => Error::InvalidData,
            AVERROR_MUXER_NOT_FOUND => Error::MuxerNotFound,
            AVERROR_OPTION_NOT_FOUND => Error::OptionNotFound,
            AVERROR_PATCHWELCOME => Error::PatchWelcome,
            AVERROR_PROTOCOL_NOT_FOUND => Error::ProtocolNotFound,
            AVERROR_STREAM_NOT_FOUND => Error::StreamNotFound,
            AVERROR_BUG2 => Error::Bug2,
            AVERROR_UNKNOWN => Error::Unknown,
            AVERROR_EXPERIMENTAL => Error::Experimental,
            AVERROR_INPUT_CHANGED => Error::InputChanged,
            AVERROR_OUTPUT_CHANGED => Error::OutputChanged,
            AVERROR_HTTP_BAD_REQUEST => Error::HttpBadRequest,
            AVERROR_HTTP_UNAUTHORIZED => Error::HttpUnauthorized,
            AVERROR_HTTP_FORBIDDEN => Error::HttpForbidden,
            AVERROR_HTTP_NOT_FOUND => Error::HttpNotFound,
            AVERROR_HTTP_OTHER_4XX => Error::HttpOther4xx,
            AVERROR_HTTP_SERVER_ERROR => Error::HttpServerError,
            e => Error::Other {
                errno: AVUNERROR(e),
            },
        }
    }
}

impl From<Error> for c_int {
    fn from(value: Error) -> c_int {
        match value {
            Error::BsfNotFound => AVERROR_BSF_NOT_FOUND,
            Error::Bug => AVERROR_BUG,
            Error::BufferTooSmall => AVERROR_BUFFER_TOO_SMALL,
            Error::DecoderNotFound => AVERROR_DECODER_NOT_FOUND,
            Error::DemuxerNotFound => AVERROR_DEMUXER_NOT_FOUND,
            Error::EncoderNotFound => AVERROR_ENCODER_NOT_FOUND,
            Error::Eof => AVERROR_EOF,
            Error::Exit => AVERROR_EXIT,
            Error::External => AVERROR_EXTERNAL,
            Error::FilterNotFound => AVERROR_FILTER_NOT_FOUND,
            Error::InvalidData => AVERROR_INVALIDDATA,
            Error::MuxerNotFound => AVERROR_MUXER_NOT_FOUND,
            Error::OptionNotFound => AVERROR_OPTION_NOT_FOUND,
            Error::PatchWelcome => AVERROR_PATCHWELCOME,
            Error::ProtocolNotFound => AVERROR_PROTOCOL_NOT_FOUND,
            Error::StreamNotFound => AVERROR_STREAM_NOT_FOUND,
            Error::Bug2 => AVERROR_BUG2,
            Error::Unknown => AVERROR_UNKNOWN,
            Error::Experimental => AVERROR_EXPERIMENTAL,
            Error::InputChanged => AVERROR_INPUT_CHANGED,
            Error::OutputChanged => AVERROR_OUTPUT_CHANGED,
            Error::HttpBadRequest => AVERROR_HTTP_BAD_REQUEST,
            Error::HttpUnauthorized => AVERROR_HTTP_UNAUTHORIZED,
            Error::HttpForbidden => AVERROR_HTTP_FORBIDDEN,
            Error::HttpNotFound => AVERROR_HTTP_NOT_FOUND,
            Error::HttpOther4xx => AVERROR_HTTP_OTHER_4XX,
            Error::HttpServerError => AVERROR_HTTP_SERVER_ERROR,
            Error::Other { errno } => AVERROR(errno),
        }
    }
}

impl error::Error for Error {}

impl From<Error> for io::Error {
    fn from(value: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut buf = [0; AV_ERROR_MAX_STRING_SIZE];

        unsafe {
            let error_text = match *self {
                Error::Other { errno } => os_strerror(errno, &mut buf),
                av_err => {
                    if 0 == av_strerror(av_err.into(), buf.as_mut_ptr(), buf.len()) {
                        CStr::from_ptr(buf.as_ptr())
                    } else {
                        CStr::from_bytes_with_nul_unchecked(b"Unknown error\0")
                    }
                }
            };

            f.write_str(from_utf8_unchecked(error_text.to_bytes()))
        }
    }
}

// SAFETY: The buffer passed to os_strerror must be 0-initialized
// in order to satisfy the safety invariants for CStr::from_ptr.

#[cfg(unix)]
unsafe fn os_strerror(errno: c_int, buf: &mut [c_char; AV_ERROR_MAX_STRING_SIZE]) -> &CStr {
    let _err = libc::strerror_r(errno, buf.as_mut_ptr(), buf.len());
    // _err can be either ERANGE or EINVAL
    // in the second case "Unknown error: <errno>" has been written to the buf
    #[cfg(test)]
    {
        if _err == libc::ERANGE {
            panic!("Insufficient buffer size")
        }
    }
    CStr::from_ptr(buf.as_ptr())
}

#[cfg(windows)]
unsafe fn os_strerror(errno: c_int, _buf: &mut [c_char; AV_ERROR_MAX_STRING_SIZE]) -> &CStr {
    CStr::from_ptr(libc::strerror(errno))
}

#[cfg(all(not(windows), not(unix)))]
unsafe fn os_strerror(errno: c_int, buf: &mut [c_char; AV_ERROR_MAX_STRING_SIZE]) -> &CStr {
    static MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let guard = MUTEX.lock();
    libc::strncpy(
        buf.as_mut_ptr(),
        libc::strerror(errno),
        AV_ERROR_MAX_STRING_SIZE - 1,
    );
    drop(guard);
    CStr::from_ptr(buf.as_ptr())
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("ffmpeg::Error(")?;
        f.write_str(&format!("{}: ", AVUNERROR((*self).into())))?;
        fmt::Display::fmt(self, f)?;
        f.write_str(")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::EAGAIN;

    #[test]
    fn test_error_roundtrip() {
        assert_eq!(Into::<c_int>::into(Error::from(AVERROR_EOF)), AVERROR_EOF);
        assert_eq!(
            Into::<c_int>::into(Error::from(AVERROR(EAGAIN))),
            AVERROR(EAGAIN)
        );
        assert_eq!(Error::from(AVERROR(EAGAIN)), Error::Other { errno: EAGAIN });
    }

    #[cfg(unix)]
    #[test]
    fn test_posix_error_string_range() {
        let mut buf = [0; AV_ERROR_MAX_STRING_SIZE];
        for e in 1..255 {
            let _ = unsafe { os_strerror(e, &mut buf) };
        }
    }

    #[cfg(unix)]
    #[test]
    fn test_posix_error_string() {
        assert_eq!(
            Error::from(AVERROR(EAGAIN)).to_string(),
            "Resource temporarily unavailable"
        )
    }

    #[test]
    fn test_error_fmt() {
        use std::fmt::Write;

        let mut s = String::new();
        write!(&mut s, "{}", Error::InvalidData).expect("can write into string");
        assert_eq!(s, "Invalid data found when processing input");

        s.clear();

        write!(&mut s, "{}", Error::from(AVERROR(EAGAIN))).expect("can write into string");
        if cfg!(unix) {
            assert_eq!(s, "Resource temporarily unavailable");
        }
    }
}

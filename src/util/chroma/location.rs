use std::ffi::CString;

use crate::ffi::AVChromaLocation::*;
use crate::ffi::*;
use crate::utils;
use crate::Error;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Location {
    Unspecified,
    Left,
    Center,
    TopLeft,
    Top,
    BottomLeft,
    Bottom,
}

impl Location {
    /// Returns the name of this location. Should usually return Some(name).
    pub fn name(self) -> Option<&'static str> {
        unsafe { utils::optional_str_from_c_ptr(av_chroma_location_name(self.into())) }
    }

    /// Returns a chroma location for the given name or an error if the `name` is invalid
    /// or no location was found.
    pub fn from_name<S: AsRef<str>>(name: S) -> Result<Self, Error> {
        let Ok(cstr) = CString::new(name.as_ref()) else {
            // invalid argument (contains a nul byte)
            return Err(Error::from(AVERROR(libc::EINVAL)));
        };

        let ret = unsafe { av_chroma_location_from_name(cstr.as_ptr()) };

        if ret < 0 {
            Err(Error::from(ret))
        } else {
            AVChromaLocation::from_c_int(ret)
                .map(Location::from)
                .ok_or(Error::from(AVERROR(libc::EINVAL)))
        }
    }

    /// Returns the swscale (x, y) chroma positions for this chroma location.
    /// Will panic if `self` is [`Unspecified`][Location::Unspecified].
    #[cfg(feature = "ffmpeg_6_0")]
    pub fn pos(self) -> (i32, i32) {
        let mut xpos = 0;
        let mut ypos = 0;
        let ret = unsafe { av_chroma_location_enum_to_pos(&mut xpos, &mut ypos, self.into()) };
        assert_eq!(ret, 0, "av_chroma_location_enum_to_pos returned an error");

        (xpos as i32, ypos as i32)
    }

    /// Returns a chroma location for the given swscale chroma position.
    #[cfg(feature = "ffmpeg_6_0")]
    pub fn from_pos(x: i32, y: i32) -> Self {
        unsafe {
            Self::from(av_chroma_location_pos_to_enum(
                x as libc::c_int,
                y as libc::c_int,
            ))
        }
    }
}

impl From<AVChromaLocation> for Location {
    fn from(value: AVChromaLocation) -> Self {
        match value {
            AVCHROMA_LOC_UNSPECIFIED => Location::Unspecified,
            AVCHROMA_LOC_LEFT => Location::Left,
            AVCHROMA_LOC_CENTER => Location::Center,
            AVCHROMA_LOC_TOPLEFT => Location::TopLeft,
            AVCHROMA_LOC_TOP => Location::Top,
            AVCHROMA_LOC_BOTTOMLEFT => Location::BottomLeft,
            AVCHROMA_LOC_BOTTOM => Location::Bottom,
            AVCHROMA_LOC_NB => Location::Unspecified,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Location> for AVChromaLocation {
    fn from(value: Location) -> AVChromaLocation {
        match value {
            Location::Unspecified => AVCHROMA_LOC_UNSPECIFIED,
            Location::Left => AVCHROMA_LOC_LEFT,
            Location::Center => AVCHROMA_LOC_CENTER,
            Location::TopLeft => AVCHROMA_LOC_TOPLEFT,
            Location::Top => AVCHROMA_LOC_TOP,
            Location::BottomLeft => AVCHROMA_LOC_BOTTOMLEFT,
            Location::Bottom => AVCHROMA_LOC_BOTTOM,
        }
    }
}

#[cfg(test)]
mod test {
    use libc::EINVAL;

    use super::*;

    #[test]
    fn name() {
        assert_eq!(Location::BottomLeft.name(), Some("bottomleft"));
        assert_eq!(Location::Center.name(), Some("center"));
        assert_eq!(Location::Unspecified.name(), Some("unspecified"));
    }

    #[test]
    fn from_name() {
        assert_eq!(Location::from_name("topleft"), Ok(Location::TopLeft));
        assert_eq!(
            Location::from_name("asdf"),
            Err(Error::Other { errno: EINVAL })
        );

        let name = "test".to_string() + "\0something else";

        // important: no panic or segfault!
        assert_eq!(
            Location::from_name(name),
            Err(Error::Other { errno: EINVAL })
        );
    }

    #[test]
    #[cfg(feature = "ffmpeg_6_0")]
    fn pos() {
        assert_eq!(Location::BottomLeft.pos(), (0, 256));
        assert_eq!(Location::Left.pos(), (0, 128));
        assert_eq!(Location::Center.pos(), (128, 128));
    }

    #[test]
    #[cfg(feature = "ffmpeg_6_0")]
    fn from_pos() {
        assert_eq!(Location::from_pos(0, 128), Location::Left);
        assert_eq!(Location::from_pos(128, 0), Location::Top);
        assert_eq!(Location::from_pos(10, 20), Location::Unspecified);
    }
}

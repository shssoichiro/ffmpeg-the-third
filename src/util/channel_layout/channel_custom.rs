use libc::c_char;

use crate::ffi::{AVChannel, AVChannelCustom};

use super::Channel;

/// Wrapper around [`AVChannelCustom`][crate::ffi::AVChannelCustom].
///
/// This struct does not support reading or writing user data via the opaque pointer.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ChannelCustom(AVChannelCustom);

impl ChannelCustom {
    pub fn new(id: Channel) -> Self {
        Self(AVChannelCustom {
            id: AVChannel::from(id),
            name: [0; 16],
            opaque: std::ptr::null_mut(),
        })
    }

    pub fn named<S: AsRef<str>>(id: Channel, name: S) -> Self {
        let name = name.as_ref();
        let name = to_char_array(name.as_bytes());

        Self(AVChannelCustom {
            id: AVChannel::from(id),
            name,
            opaque: std::ptr::null_mut(),
        })
    }
}

fn to_char_array(bytes: &[u8]) -> [c_char; 16] {
    let mut result = [0; 16];

    // Only take the first 15 bytes, leaving at least one NUL byte
    for (b, r) in bytes.iter().take(15).zip(&mut result) {
        *r = *b as c_char;
    }

    result
}

impl From<AVChannelCustom> for ChannelCustom {
    fn from(value: AVChannelCustom) -> Self {
        Self(value)
    }
}

impl From<ChannelCustom> for AVChannelCustom {
    fn from(value: ChannelCustom) -> Self {
        value.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::alloc::Layout;

    #[test]
    fn is_repr_transparent() {
        // ChannelLayout::map relies on this being true.
        assert_eq!(
            Layout::new::<AVChannelCustom>(),
            Layout::new::<ChannelCustom>()
        );
    }

    #[test]
    fn new() {
        let custom = ChannelCustom::new(Channel::FrontRight);
        assert_eq!(custom.0.id, AVChannel::AV_CHAN_FRONT_RIGHT);
        assert_eq!(custom.0.name, [0; 16]);
        assert!(custom.0.opaque.is_null());
    }

    #[test]
    fn named() {
        // "Bottom front ri\0"
        let c_str_name = [
            66, 111, 116, 116, 111, 109, 32, 102, 114, 111, 110, 116, 32, 114, 105, 0,
        ];

        let custom = ChannelCustom::named(Channel::BottomFrontRight, "Bottom front right");
        assert_eq!(custom.0.id, AVChannel::AV_CHAN_BOTTOM_FRONT_RIGHT);
        assert_eq!(custom.0.name, c_str_name);
        assert!(custom.0.opaque.is_null());
    }
}

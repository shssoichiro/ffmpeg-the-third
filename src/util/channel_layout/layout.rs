use std::borrow::Borrow;
use std::borrow::Cow;
use std::ffi::CString;
use std::mem::{align_of, size_of};

use crate::ffi::*;
#[cfg(feature = "ffmpeg_7_0")]
use crate::Error;
use libc::{c_int, c_uint};

use super::Channel;
use super::ChannelCustom;
use super::ChannelLayoutIter;
use super::ChannelLayoutMask;
use super::ChannelOrder;

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelLayout<'a>(Cow<'a, AVChannelLayout>);

impl<'a> ChannelLayout<'a> {
    /// Get a new channel layout with an unspecified channel ordering.
    pub fn unspecified(channels: u32) -> Self {
        let mut layout = AVChannelLayout::empty();
        layout.order = AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC;
        layout.nb_channels = channels as c_int;

        Self(Cow::Owned(layout))
    }

    pub fn custom(channels: Vec<ChannelCustom>) -> Self {
        #[cold]
        fn alloc_failed(channels: usize) -> ! {
            use std::alloc::{handle_alloc_error, Layout};

            let alloc_size = channels * size_of::<AVChannelCustom>();
            let layout =
                Layout::from_size_align(alloc_size, align_of::<AVChannelCustom>()).unwrap();
            handle_alloc_error(layout)
        }

        let mut layout = AVChannelLayout::empty();
        layout.order = AVChannelOrder::AV_CHANNEL_ORDER_CUSTOM;
        layout.nb_channels = channels.len() as c_int;
        unsafe {
            layout.u.map = av_malloc_array(channels.len(), size_of::<AVChannelCustom>()) as _;
            if layout.u.map.is_null() {
                alloc_failed(channels.len());
            }

            for (i, ch) in channels.into_iter().enumerate() {
                std::ptr::write(layout.u.map.add(i), AVChannelCustom::from(ch));
            }
        }

        Self(Cow::Owned(layout))
    }

    /// Get the default channel layout for a given number of channels.
    ///
    /// If no default layout exists for the given number of channels,
    /// an unspecified layout will be returned.
    pub fn default_for_channels(channels: u32) -> Self {
        let mut layout = AVChannelLayout::empty();

        unsafe {
            av_channel_layout_default(&mut layout as _, channels as c_int);
        }

        Self(Cow::Owned(layout))
    }

    /// Get an iterator over all standard channel layouts.
    pub fn standard_layouts() -> ChannelLayoutIter {
        ChannelLayoutIter::new()
    }

    /// Initialize a native channel layout from a bitmask indicating which
    /// channels are present.
    ///
    /// This will return [None] for invalid bitmask values.
    pub fn from_mask(layout_mask: ChannelLayoutMask) -> Option<Self> {
        let mut layout = AVChannelLayout::empty();
        let ret = unsafe { av_channel_layout_from_mask(&mut layout as _, layout_mask.bits()) };

        match ret {
            0 => Some(Self(Cow::Owned(layout))),
            // This should only ever return 0 or AVERROR(EINVAL)
            _ => None,
        }
    }

    /// Initialize a channel layout from a given string description.
    ///
    /// This can be
    /// - the formal channel layout name (as returned by [`description`][ChannelLayout::description]),
    /// - one or more channel names concatenated with "+", each optionally containing a
    ///   custom name after an "@", e.g. "FL@Left+FR@Right+LFE",
    /// - a decimal or hexadecimal value of a native channel layout (e.g. "4" or "0x4"),
    /// - the number of channels with the default layout (e.g. "4c"),
    /// - the number of unordered channels (e.g. "4C" or "4 channels") or
    /// - the ambisonic order followed by optional non-diegetic channels (e.g. "ambisonic 2+stereo")
    pub fn from_string<S: AsRef<str>>(description: S) -> Option<Self> {
        let mut layout = AVChannelLayout::empty();
        let cstr = CString::new(description.as_ref()).expect("no nul byte in description");
        let ret = unsafe { av_channel_layout_from_string(&mut layout as _, cstr.as_ptr()) };

        match ret {
            0 => Some(Self(Cow::Owned(layout))),
            // This should only ever return 0 or AVERROR_INVALIDDATA
            _ => None,
        }
    }

    /// The [`ChannelOrder`][super::ChannelOrder] used in this layout.
    pub fn order(&self) -> ChannelOrder {
        ChannelOrder::from(self.0.order)
    }

    /// The number of channels in this layout.
    pub fn channels(&self) -> u32 {
        self.0.nb_channels as u32
    }

    /// If [`order`][ChannelLayout::order] is [`Native`][ChannelOrder::Native]:
    /// A [`ChannelLayoutMask`] containing the channels of this layout.
    ///
    /// If [`order`][ChannelLayout::order] is [`Ambisonic`][ChannelOrder::Ambisonic]:
    /// A [`ChannelLayoutMask`] containing the non-diegetic channels of this layout.
    ///
    /// Otherwise: [`None`].
    pub fn mask(&self) -> Option<ChannelLayoutMask> {
        match self.order() {
            ChannelOrder::Unspecified | ChannelOrder::Custom => None,
            ChannelOrder::Native | ChannelOrder::Ambisonic => unsafe {
                Some(ChannelLayoutMask::from_bits_truncate(self.0.u.mask))
            },
        }
    }

    /// Returns the custom channel map for this layout.
    ///
    /// None if [`order`][ChannelLayout::order] is not [`Custom`][ChannelOrder::Custom].
    pub fn map(&self) -> Option<&[ChannelCustom]> {
        if self.order() != ChannelOrder::Custom {
            return None;
        }

        unsafe {
            // SAFETY: ChannelCustom is repr(transparent) around AVChannelCustom
            Some(std::slice::from_raw_parts(
                self.0.u.map as _,
                self.0.nb_channels as usize,
            ))
        }
    }

    /// Extracts the owned `AVChannelLayout`.
    ///
    /// Clones it if not already owned.
    pub fn into_owned(self) -> AVChannelLayout {
        self.0.into_owned()
    }

    /// Exposes a pointer to the contained `AVChannelLayout` for FFI purposes.
    ///
    /// This is guaranteed to be a non-null pointer.
    pub fn as_ptr(&self) -> *const AVChannelLayout {
        self.0.as_ref() as _
    }

    /// Get a human-readable [`String`] describing the channel layout properties.
    ///
    /// The returned string will be in the same format that is accepted by [`from_string`][ChannelLayout::from_string],
    /// allowing to rebuild the same channel layout (excluding opaque pointers).
    pub fn description(&self) -> String {
        let mut buf = vec![0u8; 256];

        unsafe {
            let ret_val =
                av_channel_layout_describe(self.as_ptr(), buf.as_mut_ptr() as _, buf.len());

            match usize::try_from(ret_val) {
                Ok(out_len) if out_len > 0 => {
                    #[cfg(feature = "ffmpeg_6_1")]
                    // 6.1 changed out_len to include the NUL byte, which we don't want
                    let out_len = out_len - 1;

                    buf.truncate(out_len);
                    String::from_utf8_unchecked(buf)
                }
                // `av_channel_layout_describe` returned an error, or 0 bytes written.
                _ => String::new(),
            }
        }
    }

    /// Get the channel with the given index in a channel layout.
    ///
    /// Returns [`Channel::None`] when the index is invalid or the channel order is unspecified.
    pub fn channel_from_index(&self, idx: u32) -> Channel {
        Channel::from(unsafe { av_channel_layout_channel_from_index(self.as_ptr(), idx as c_uint) })
    }

    /// Get the index of a given channel in a channel layout.
    pub fn index_from_channel(&self, channel: Channel) -> Option<u32> {
        unsafe {
            u32::try_from(av_channel_layout_index_from_channel(
                self.as_ptr(),
                AVChannel::from(channel),
            ))
            .ok()
        }
    }

    /// Get the index in a channel layout of a channel described by the given string.
    ///
    /// Returns the first match. Accepts channel names in the same format as [`from_string`][ChannelLayout::from_string].
    pub fn index_from_string<S: AsRef<str>>(&self, name: S) -> Option<u32> {
        let cstr = CString::new(name.as_ref()).expect("no nul byte in name");
        let ret = unsafe { av_channel_layout_index_from_string(self.as_ptr(), cstr.as_ptr()) };

        u32::try_from(ret).ok()
    }

    /// Get a channel described by the given string.
    ///
    /// Accepts channel names in the same format as [`from_string`][ChannelLayout::from_string].
    ///
    /// Returns [`Channel::None`] when the string is invalid or the channel order is unspecified.
    pub fn channel_from_string<S: AsRef<str>>(&self, name: S) -> Channel {
        let cstr = CString::new(name.as_ref()).expect("no nul byte in name");

        Channel::from(unsafe {
            av_channel_layout_channel_from_string(self.as_ptr(), cstr.as_ptr())
        })
    }

    /// Find out what channels from a given set are present in this layout, without regard for their positions.
    pub fn subset(&self, mask: ChannelLayoutMask) -> ChannelLayoutMask {
        ChannelLayoutMask::from_bits_truncate(unsafe {
            av_channel_layout_subset(self.as_ptr(), mask.bits())
        })
    }

    /// Check whether this layout is valid (i.e. can describe audio data).
    #[doc(alias = "check")]
    pub fn is_valid(&self) -> bool {
        unsafe { av_channel_layout_check(self.as_ptr()) != 0 }
    }

    /// Change the [`ChannelOrder`] of this channel layout. If the current layout is borrowed,
    /// calling this function will clone the contained [`AVChannelLayout`].
    ///
    /// This change can be lossless or lossy:
    /// - A lossless conversion keeps all [`Channel`] designations and names intact.
    /// - A lossy conversion might lose [`Channel`] designations and names depending on the targeted
    ///   channel order.
    ///
    /// # Supported conversions
    /// - Any -> Custom: Always possible, always lossless.
    /// - Any -> Unspecified: Always possible, only lossless if every channel is designated
    ///   [`Unknown`][Channel#variant.Unknown] and no channel names are used.
    /// - Custom -> Ambisonic: Possible if it contains ambisonic channels with optional non-diegetic
    ///   channels in the end. Lossless only if no channels have custom names.
    /// - Custom -> Native: Possible if it contains native channels in native order. Lossless only
    ///   if no channels have custom names.
    ///
    /// # Returns
    /// - [`Ok`] if the conversion succeeded. The contained [`ChannelRetypeKind`] indicates
    ///   whether the conversion was lossless or not.
    /// - [`Err`] if the conversion failed. The original layout is untouched in this case.
    #[cfg(feature = "ffmpeg_7_0")]
    pub fn retype(&mut self, target: ChannelRetypeTarget) -> Result<ChannelRetypeKind, Error> {
        use std::cmp::Ordering;
        use ChannelRetypeTarget as Target;

        let (channel_order, flags) = match target {
            Target::Lossy(order) => (order, 0),
            Target::Lossless(order) => (order, AV_CHANNEL_LAYOUT_RETYPE_FLAG_LOSSLESS),
            Target::Canonical => (
                ChannelOrder::Unspecified,
                AV_CHANNEL_LAYOUT_RETYPE_FLAG_CANONICAL,
            ),
        };

        let ret = unsafe { av_channel_layout_retype(self.0.to_mut(), channel_order.into(), flags) };

        match ret.cmp(&0) {
            Ordering::Greater => Ok(ChannelRetypeKind::Lossy),
            Ordering::Equal => Ok(ChannelRetypeKind::Lossless),
            Ordering::Less => Err(Error::from(ret)),
        }
    }
}

/// Whether the retyping was lossless or not.
#[cfg(feature = "ffmpeg_7_0")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelRetypeKind {
    Lossless,
    Lossy,
}

/// The possible targets for retyping channel layouts. See [`ChannelLayout::retype`]
/// for more information.
#[cfg(feature = "ffmpeg_7_0")]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelRetypeTarget {
    /// Target a specific channel order, allowing lossy retyping.
    Lossy(ChannelOrder),
    /// Target a specific channel order, only allowing lossless retyping.
    Lossless(ChannelOrder),
    /// Automatically select the simplest channel order which allows lossless retyping.
    Canonical,
}

impl<'a> From<AVChannelLayout> for ChannelLayout<'a> {
    fn from(value: AVChannelLayout) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> From<&'a AVChannelLayout> for ChannelLayout<'a> {
    fn from(value: &'a AVChannelLayout) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'a> Borrow<AVChannelLayout> for ChannelLayout<'a> {
    fn borrow(&self) -> &AVChannelLayout {
        &self.0
    }
}

// Type alias to reduce line length below
type Scl = ChannelLayout<'static>;

// Constants
impl<'a> ChannelLayout<'a> {
    pub const MONO: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_MONO));
    pub const STEREO: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_STEREO));
    pub const _2POINT1: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_2POINT1));
    pub const _2_1: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_2_1));
    pub const SURROUND: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_SURROUND));
    pub const _3POINT1: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_3POINT1));
    pub const _4POINT0: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_4POINT0));
    pub const _4POINT1: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_4POINT1));
    pub const _2_2: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_2_2));
    pub const QUAD: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_QUAD));
    pub const _5POINT0: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT0));
    pub const _5POINT1: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT1));
    pub const _5POINT0_BACK: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT0_BACK));
    pub const _5POINT1_BACK: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT1_BACK));
    pub const _6POINT0: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_6POINT0));
    pub const _6POINT0_FRONT: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_6POINT0_FRONT));
    pub const _3POINT1POINT2: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_3POINT1POINT2));
    pub const HEXAGONAL: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_HEXAGONAL));
    pub const _6POINT1: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_6POINT1));
    pub const _6POINT1_BACK: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_6POINT1_BACK));
    pub const _6POINT1_FRONT: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_6POINT1_FRONT));
    pub const _7POINT0: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT0));
    pub const _7POINT0_FRONT: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT0_FRONT));
    pub const _7POINT1: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT1));
    pub const _7POINT1_WIDE: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT1_WIDE));
    pub const _7POINT1_WIDE_BACK: Scl =
        ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT1_WIDE_BACK));
    pub const _5POINT1POINT2_BACK: Scl =
        ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT1POINT2_BACK));
    pub const OCTAGONAL: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_OCTAGONAL));
    pub const CUBE: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_CUBE));
    pub const _5POINT1POINT4_BACK: Scl =
        ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT1POINT4_BACK));
    pub const _7POINT1POINT2: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT1POINT2));
    pub const _7POINT1POINT4_BACK: Scl =
        ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT1POINT4_BACK));
    #[cfg(feature = "ffmpeg_7_0")]
    pub const _7POINT2POINT3: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT2POINT3));
    #[cfg(feature = "ffmpeg_7_0")]
    pub const _9POINT1POINT4_BACK: Scl =
        ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_9POINT1POINT4_BACK));
    pub const HEXADECAGONAL: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_HEXADECAGONAL));
    pub const STEREO_DOWNMIX: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_STEREO_DOWNMIX));
    pub const _22POINT2: Scl = ChannelLayout(Cow::Owned(AV_CHANNEL_LAYOUT_22POINT2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unspecified() {
        let empty = ChannelLayout::unspecified(0);
        assert_eq!(empty.order(), ChannelOrder::Unspecified);
        assert_eq!(empty.channels(), 0);
        assert!(!empty.is_valid());

        let unspec = ChannelLayout::unspecified(42);
        assert_eq!(unspec.order(), ChannelOrder::Unspecified);
        assert_eq!(unspec.channels(), 42);
        assert!(unspec.is_valid());
    }

    #[test]
    fn custom() {
        let channels = vec![
            ChannelCustom::new(Channel::FrontLeft),
            ChannelCustom::new(Channel::FrontRight),
            ChannelCustom::named(Channel::LowFrequency, "bass"),
            ChannelCustom::named(Channel::BackLeft, "back left"),
            // name too long on purpose -> should truncate to 15 chars + NUL
            ChannelCustom::named(Channel::BottomFrontCenter, "bottom front center"),
        ];

        let custom = ChannelLayout::custom(channels.clone());
        assert!(custom.is_valid());
        assert_eq!(custom.channels(), 5);
        assert_eq!(custom.order(), ChannelOrder::Custom);
        assert_eq!(custom.map().unwrap(), &channels);
    }

    #[test]
    fn defaults() {
        let unspec = ChannelLayout::default_for_channels(0);
        assert!(unspec.order() == ChannelOrder::Unspecified);
        assert!(!unspec.is_valid());

        for i in 1..12 {
            let layout = ChannelLayout::default_for_channels(i);
            assert_eq!(layout.channels(), i);
            assert!(layout.is_valid(), "default layout invalid for {i} channels");
            assert!(!layout.description().is_empty());
        }
    }

    #[test]
    fn from_mask() {
        use ChannelLayout as Layout;
        use ChannelLayoutMask as Mask;

        assert_eq!(Layout::from_mask(Mask::empty()), None);

        let tests = [
            (Mask::MONO, Layout::MONO),
            (Mask::STEREO, Layout::STEREO),
            (Mask::_2POINT1, Layout::_2POINT1),
            (Mask::_2_1, Layout::_2_1),
            (Mask::SURROUND, Layout::SURROUND),
            (Mask::_3POINT1, Layout::_3POINT1),
            (Mask::_4POINT0, Layout::_4POINT0),
            (Mask::_4POINT1, Layout::_4POINT1),
            (Mask::_2_2, Layout::_2_2),
            (Mask::QUAD, Layout::QUAD),
            (Mask::_5POINT0, Layout::_5POINT0),
            (Mask::_5POINT1, Layout::_5POINT1),
            (Mask::_5POINT0_BACK, Layout::_5POINT0_BACK),
            (Mask::_5POINT1_BACK, Layout::_5POINT1_BACK),
            (Mask::_6POINT0, Layout::_6POINT0),
            (Mask::_6POINT0_FRONT, Layout::_6POINT0_FRONT),
            (Mask::HEXAGONAL, Layout::HEXAGONAL),
            (Mask::_3POINT1POINT2, Layout::_3POINT1POINT2),
            (Mask::_6POINT1, Layout::_6POINT1),
            (Mask::_6POINT1_BACK, Layout::_6POINT1_BACK),
            (Mask::_6POINT1_FRONT, Layout::_6POINT1_FRONT),
            (Mask::_7POINT0, Layout::_7POINT0),
            (Mask::_7POINT0_FRONT, Layout::_7POINT0_FRONT),
            (Mask::_7POINT1, Layout::_7POINT1),
            (Mask::_7POINT1_WIDE, Layout::_7POINT1_WIDE),
            (Mask::_7POINT1_WIDE_BACK, Layout::_7POINT1_WIDE_BACK),
            (Mask::_5POINT1POINT2_BACK, Layout::_5POINT1POINT2_BACK),
            (Mask::OCTAGONAL, Layout::OCTAGONAL),
            (Mask::CUBE, Layout::CUBE),
            (Mask::_5POINT1POINT4_BACK, Layout::_5POINT1POINT4_BACK),
            (Mask::_7POINT1POINT2, Layout::_7POINT1POINT2),
            (Mask::_7POINT1POINT4_BACK, Layout::_7POINT1POINT4_BACK),
            (Mask::HEXADECAGONAL, Layout::HEXADECAGONAL),
            (Mask::STEREO_DOWNMIX, Layout::STEREO_DOWNMIX),
            (Mask::_22POINT2, Layout::_22POINT2),
        ];

        for (mask, expected) in tests {
            let result = Layout::from_mask(mask).expect("can find layout for bitmask");
            assert_eq!(
                result.order(),
                ChannelOrder::Native,
                "layout from mask must use native order"
            );
            assert_eq!(result.mask(), Some(mask));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn from_string() {
        let test_strings = [
            ("1 channels (FRC)", ChannelOrder::Native, 1),
            ("FL@Left+FR@Right+LFE", ChannelOrder::Custom, 3),
            ("0x4", ChannelOrder::Native, 1),
            ("4c", ChannelOrder::Native, 4),
            ("7 channels", ChannelOrder::Unspecified, 7),
            ("ambisonic 2+stereo", ChannelOrder::Ambisonic, 11),
        ];

        for (s, order, channels) in test_strings {
            let result = ChannelLayout::from_string(s).expect("can find layout for description");
            assert!(result.is_valid());
            assert_eq!(result.order(), order);
            assert_eq!(result.channels(), channels);
        }
    }

    #[test]
    fn describe() {
        use ChannelLayout as Layout;
        use ChannelLayoutMask as Mask;

        let tests = [
            (Layout::MONO, "mono"),
            (Layout::STEREO, "stereo"),
            (Layout::_5POINT1, "5.1(side)"),
            (
                Layout::from_string("FL@Left+FR@Right+LFE").unwrap(),
                "3 channels (FL@Left+FR@Right+LFE)",
            ),
            (
                Layout::from_mask(Mask::FRONT_RIGHT_OF_CENTER).unwrap(),
                "1 channels (FRC)",
            ),
            #[cfg(feature = "ffmpeg_6_1")]
            (Layout::_7POINT1POINT4_BACK, "7.1.4"),
            #[cfg(not(feature = "ffmpeg_6_1"))]
            (
                Layout::_7POINT1POINT4_BACK,
                "12 channels (FL+FR+FC+LFE+BL+BR+SL+SR+TFL+TFR+TBL+TBR)",
            ),
        ];

        for (layout, expected) in tests {
            assert!(layout.is_valid());

            let desc = layout.description();
            assert!(!desc.is_empty());
            assert_eq!(desc, expected);
        }
    }

    #[cfg(feature = "ffmpeg_7_0")]
    #[test]
    fn retype() {
        use ChannelLayout as Layout;
        use ChannelOrder as Order;
        use ChannelRetypeKind as Kind;
        use ChannelRetypeTarget as Target;

        let tests = [
            (
                // Ok(Lossless) if target order == current order
                Layout::_7POINT1POINT4_BACK,
                Target::Lossless(Order::Native),
                Ok(Kind::Lossless),
                Layout::_7POINT1POINT4_BACK,
                true,
            ),
            (
                // any -> custom always lossless
                Layout::STEREO,
                Target::Lossless(Order::Custom),
                Ok(Kind::Lossless),
                Layout::custom(vec![
                    ChannelCustom::new(Channel::FrontLeft),
                    ChannelCustom::new(Channel::FrontRight),
                ]),
                true,
            ),
            (
                // any -> custom also works if lossy requested
                Layout::STEREO,
                Target::Lossy(Order::Custom),
                Ok(Kind::Lossless),
                Layout::custom(vec![
                    ChannelCustom::new(Channel::FrontLeft),
                    ChannelCustom::new(Channel::FrontRight),
                ]),
                true,
            ),
            (
                // any -> unspecified lossy unless all channels are Channel::Unknown and unnamed
                Layout::OCTAGONAL,
                Target::Lossy(Order::Unspecified),
                Ok(Kind::Lossy),
                Layout::unspecified(8),
                true,
            ),
            (
                // AVERROR(ENOSYS) if lossless requested, but only lossy possible
                Layout::OCTAGONAL,
                Target::Lossless(Order::Unspecified),
                Err(Error::Other {
                    errno: libc::ENOSYS,
                }),
                Layout::OCTAGONAL,
                true,
            ),
            (
                // custom -> native lossless without names
                Layout::custom(vec![
                    ChannelCustom::new(Channel::FrontLeft),
                    ChannelCustom::new(Channel::FrontRight),
                    ChannelCustom::new(Channel::FrontCenter),
                    ChannelCustom::new(Channel::LowFrequency),
                ]),
                Target::Lossy(Order::Native),
                Ok(Kind::Lossless),
                Layout::_3POINT1,
                true,
            ),
            (
                // custom -> native lossy with name
                Layout::custom(vec![
                    ChannelCustom::new(Channel::FrontLeft),
                    ChannelCustom::new(Channel::FrontRight),
                    ChannelCustom::named(Channel::FrontCenter, "front center"),
                    ChannelCustom::new(Channel::LowFrequency),
                ]),
                Target::Lossy(Order::Native),
                Ok(Kind::Lossy),
                Layout::_3POINT1,
                true,
            ),
            (
                // AVERROR(EINVAL) if !layout.is_valid()
                Layout::unspecified(0),
                Target::Lossy(ChannelOrder::Custom),
                Err(Error::Other {
                    errno: libc::EINVAL,
                }),
                Layout::unspecified(0),
                false,
            ),
        ];

        for (layout, target, expected_result, expected_layout, expected_valid) in tests {
            let mut layout = layout.clone();
            let actual_result = layout.retype(target);

            assert_eq!(
                layout.is_valid(),
                expected_valid,
                "is_valid should return {expected_valid} for {layout:?}, but did not."
            );
            assert_eq!(
                actual_result,
                expected_result,
                "retype should return {expected_result:?} for {layout:?}, but returned {actual_result:?}"
            );
            assert_eq!(layout, expected_layout);
        }
    }
}

use crate::ffi::*;

use std::ffi::CString;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Channel {
    None,
    FrontLeft,
    FrontRight,
    FrontCenter,
    LowFrequency,
    BackLeft,
    BackRight,
    FrontLeftOfCenter,
    FrontRightOfCenter,
    BackCenter,
    SideLeft,
    SideRight,
    TopCenter,
    TopFrontLeft,
    TopFrontCenter,
    TopFrontRight,
    TopBackLeft,
    TopBackCenter,
    TopBackRight,
    StereoLeft,
    StereoRight,
    WideLeft,
    WideRight,
    SurroundDirectLeft,
    SurroundDirectRight,
    LowFrequency2,
    TopSideLeft,
    TopSideRight,
    BottomFrontCenter,
    BottomFrontLeft,
    BottomFrontRight,

    #[cfg(feature = "ffmpeg_7_1")]
    SideSurroundLeft,
    #[cfg(feature = "ffmpeg_7_1")]
    SideSurroundRight,
    #[cfg(feature = "ffmpeg_7_1")]
    TopSurroundLeft,
    #[cfg(feature = "ffmpeg_7_1")]
    TopSurroundRight,

    /// Channel is empty and can be safely skipped.
    Unused,

    /// Channel contains data, but its position is unknown.
    Unknown,

    /// Defines the start of channel IDs when using Ambisonic.
    AmbisonicBase,
    /// Defines the end of channel IDs when using Ambisonic.
    AmbisonicEnd,
}

impl Channel {
    /// Get an abbreviated, human-readable string describing this channel.
    pub fn name(self) -> String {
        let mut buf = vec![0u8; 32];

        unsafe {
            let ret_val = av_channel_name(buf.as_mut_ptr() as _, buf.len(), AVChannel::from(self));

            match usize::try_from(ret_val) {
                Ok(out_len) if out_len > 0 => {
                    #[cfg(feature = "ffmpeg_6_1")]
                    // 6.1 changed out_len to include the NUL byte, which we don't want
                    let out_len = out_len - 1;

                    buf.truncate(out_len);
                    String::from_utf8_unchecked(buf)
                }
                // `av_channel_name` returned an error, or 0 bytes written.
                _ => String::new(),
            }
        }
    }

    /// Get a human-readable string describing this channel.
    pub fn description(self) -> String {
        let mut buf = vec![0u8; 256];

        unsafe {
            let ret_val =
                av_channel_description(buf.as_mut_ptr() as _, buf.len(), AVChannel::from(self));

            match usize::try_from(ret_val) {
                Ok(out_len) if out_len > 0 => {
                    #[cfg(feature = "ffmpeg_6_1")]
                    // 6.1 changed out_len to include the NUL byte, which we don't want
                    let out_len = out_len - 1;

                    buf.truncate(out_len);
                    String::from_utf8_unchecked(buf)
                }
                // `av_channel_description` returned an error, or 0 bytes written.
                _ => String::new(),
            }
        }
    }

    /// This is the inverse function of [`name`][Channel::name].
    pub fn from_string<S: AsRef<str>>(name: S) -> Self {
        let cstr = CString::new(name.as_ref()).expect("no nul byte in name");
        Self::from(unsafe { av_channel_from_string(cstr.as_ptr()) })
    }
}

impl From<AVChannel> for Channel {
    fn from(value: AVChannel) -> Self {
        use crate::ffi::AVChannel::*;
        use Channel::*;

        match value {
            AV_CHAN_NONE => None,
            AV_CHAN_FRONT_LEFT => FrontLeft,
            AV_CHAN_FRONT_RIGHT => FrontRight,
            AV_CHAN_FRONT_CENTER => FrontCenter,
            AV_CHAN_LOW_FREQUENCY => LowFrequency,
            AV_CHAN_BACK_LEFT => BackLeft,
            AV_CHAN_BACK_RIGHT => BackRight,
            AV_CHAN_FRONT_LEFT_OF_CENTER => FrontLeftOfCenter,
            AV_CHAN_FRONT_RIGHT_OF_CENTER => FrontRightOfCenter,
            AV_CHAN_BACK_CENTER => BackCenter,
            AV_CHAN_SIDE_LEFT => SideLeft,
            AV_CHAN_SIDE_RIGHT => SideRight,
            AV_CHAN_TOP_CENTER => TopCenter,
            AV_CHAN_TOP_FRONT_LEFT => TopFrontLeft,
            AV_CHAN_TOP_FRONT_CENTER => TopFrontCenter,
            AV_CHAN_TOP_FRONT_RIGHT => TopFrontRight,
            AV_CHAN_TOP_BACK_LEFT => TopBackLeft,
            AV_CHAN_TOP_BACK_CENTER => TopBackCenter,
            AV_CHAN_TOP_BACK_RIGHT => TopBackRight,
            AV_CHAN_STEREO_LEFT => StereoLeft,
            AV_CHAN_STEREO_RIGHT => StereoRight,
            AV_CHAN_WIDE_LEFT => WideLeft,
            AV_CHAN_WIDE_RIGHT => WideRight,
            AV_CHAN_SURROUND_DIRECT_LEFT => SurroundDirectLeft,
            AV_CHAN_SURROUND_DIRECT_RIGHT => SurroundDirectRight,
            AV_CHAN_LOW_FREQUENCY_2 => LowFrequency2,
            AV_CHAN_TOP_SIDE_LEFT => TopSideLeft,
            AV_CHAN_TOP_SIDE_RIGHT => TopSideRight,
            AV_CHAN_BOTTOM_FRONT_CENTER => BottomFrontCenter,
            AV_CHAN_BOTTOM_FRONT_LEFT => BottomFrontLeft,
            AV_CHAN_BOTTOM_FRONT_RIGHT => BottomFrontRight,

            #[cfg(feature = "ffmpeg_7_1")]
            AV_CHAN_SIDE_SURROUND_LEFT => SideSurroundLeft,
            #[cfg(feature = "ffmpeg_7_1")]
            AV_CHAN_SIDE_SURROUND_RIGHT => SideSurroundRight,
            #[cfg(feature = "ffmpeg_7_1")]
            AV_CHAN_TOP_SURROUND_LEFT => TopSurroundLeft,
            #[cfg(feature = "ffmpeg_7_1")]
            AV_CHAN_TOP_SURROUND_RIGHT => TopSurroundRight,

            AV_CHAN_UNUSED => Unused,
            AV_CHAN_UNKNOWN => Unknown,
            AV_CHAN_AMBISONIC_BASE => AmbisonicBase,
            AV_CHAN_AMBISONIC_END => AmbisonicEnd,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Channel> for AVChannel {
    fn from(value: Channel) -> Self {
        use crate::ffi::AVChannel::*;
        use Channel::*;

        match value {
            None => AV_CHAN_NONE,
            FrontLeft => AV_CHAN_FRONT_LEFT,
            FrontRight => AV_CHAN_FRONT_RIGHT,
            FrontCenter => AV_CHAN_FRONT_CENTER,
            LowFrequency => AV_CHAN_LOW_FREQUENCY,
            BackLeft => AV_CHAN_BACK_LEFT,
            BackRight => AV_CHAN_BACK_RIGHT,
            FrontLeftOfCenter => AV_CHAN_FRONT_LEFT_OF_CENTER,
            FrontRightOfCenter => AV_CHAN_FRONT_RIGHT_OF_CENTER,
            BackCenter => AV_CHAN_BACK_CENTER,
            SideLeft => AV_CHAN_SIDE_LEFT,
            SideRight => AV_CHAN_SIDE_RIGHT,
            TopCenter => AV_CHAN_TOP_CENTER,
            TopFrontLeft => AV_CHAN_TOP_FRONT_LEFT,
            TopFrontCenter => AV_CHAN_TOP_FRONT_CENTER,
            TopFrontRight => AV_CHAN_TOP_FRONT_RIGHT,
            TopBackLeft => AV_CHAN_TOP_BACK_LEFT,
            TopBackCenter => AV_CHAN_TOP_BACK_CENTER,
            TopBackRight => AV_CHAN_TOP_BACK_RIGHT,
            StereoLeft => AV_CHAN_STEREO_LEFT,
            StereoRight => AV_CHAN_STEREO_RIGHT,
            WideLeft => AV_CHAN_WIDE_LEFT,
            WideRight => AV_CHAN_WIDE_RIGHT,
            SurroundDirectLeft => AV_CHAN_SURROUND_DIRECT_LEFT,
            SurroundDirectRight => AV_CHAN_SURROUND_DIRECT_RIGHT,
            LowFrequency2 => AV_CHAN_LOW_FREQUENCY_2,
            TopSideLeft => AV_CHAN_TOP_SIDE_LEFT,
            TopSideRight => AV_CHAN_TOP_SIDE_RIGHT,
            BottomFrontCenter => AV_CHAN_BOTTOM_FRONT_CENTER,
            BottomFrontLeft => AV_CHAN_BOTTOM_FRONT_LEFT,
            BottomFrontRight => AV_CHAN_BOTTOM_FRONT_RIGHT,

            #[cfg(feature = "ffmpeg_7_1")]
            SideSurroundLeft => AV_CHAN_SIDE_SURROUND_LEFT,
            #[cfg(feature = "ffmpeg_7_1")]
            SideSurroundRight => AV_CHAN_SIDE_SURROUND_RIGHT,
            #[cfg(feature = "ffmpeg_7_1")]
            TopSurroundLeft => AV_CHAN_TOP_SURROUND_LEFT,
            #[cfg(feature = "ffmpeg_7_1")]
            TopSurroundRight => AV_CHAN_TOP_SURROUND_RIGHT,

            Unused => AV_CHAN_UNUSED,
            Unknown => AV_CHAN_UNKNOWN,
            AmbisonicBase => AV_CHAN_AMBISONIC_BASE,
            AmbisonicEnd => AV_CHAN_AMBISONIC_END,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // just test everything
    const TEST_VALUES: &[Channel] = &[
        Channel::None,
        Channel::FrontLeft,
        Channel::FrontRight,
        Channel::FrontCenter,
        Channel::LowFrequency,
        Channel::BackLeft,
        Channel::BackRight,
        Channel::FrontLeftOfCenter,
        Channel::FrontRightOfCenter,
        Channel::BackCenter,
        Channel::SideLeft,
        Channel::SideRight,
        Channel::TopCenter,
        Channel::TopFrontLeft,
        Channel::TopFrontCenter,
        Channel::TopFrontRight,
        Channel::TopBackLeft,
        Channel::TopBackCenter,
        Channel::TopBackRight,
        Channel::StereoLeft,
        Channel::StereoRight,
        Channel::WideLeft,
        Channel::WideRight,
        Channel::SurroundDirectLeft,
        Channel::SurroundDirectRight,
        Channel::LowFrequency2,
        Channel::TopSideLeft,
        Channel::TopSideRight,
        Channel::BottomFrontCenter,
        Channel::BottomFrontLeft,
        Channel::BottomFrontRight,
        Channel::Unused,
        Channel::Unknown,
        Channel::AmbisonicBase,
        Channel::AmbisonicEnd,
    ];

    #[test]
    fn name() {
        for ch in TEST_VALUES {
            let name = ch.name();
            assert!(!name.is_empty());
            println!("{name}");
        }
    }

    #[test]
    fn description() {
        for ch in TEST_VALUES {
            let desc = ch.description();
            assert!(!desc.is_empty());
            println!("{desc}");
        }
    }

    #[test]
    fn from_string() {
        for ch in TEST_VALUES {
            let name = ch.name();
            let found = Channel::from_string(name);
            assert_eq!(found, *ch);
        }
    }
}

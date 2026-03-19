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

    #[cfg(feature = "ffmpeg_8_0")]
    BinauralLeft,
    #[cfg(feature = "ffmpeg_8_0")]
    BinauralRight,

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
        use AVChannel as AV;
        use Channel::*;

        match value {
            AV::NONE => None,
            AV::FRONT_LEFT => FrontLeft,
            AV::FRONT_RIGHT => FrontRight,
            AV::FRONT_CENTER => FrontCenter,
            AV::LOW_FREQUENCY => LowFrequency,
            AV::BACK_LEFT => BackLeft,
            AV::BACK_RIGHT => BackRight,
            AV::FRONT_LEFT_OF_CENTER => FrontLeftOfCenter,
            AV::FRONT_RIGHT_OF_CENTER => FrontRightOfCenter,
            AV::BACK_CENTER => BackCenter,
            AV::SIDE_LEFT => SideLeft,
            AV::SIDE_RIGHT => SideRight,
            AV::TOP_CENTER => TopCenter,
            AV::TOP_FRONT_LEFT => TopFrontLeft,
            AV::TOP_FRONT_CENTER => TopFrontCenter,
            AV::TOP_FRONT_RIGHT => TopFrontRight,
            AV::TOP_BACK_LEFT => TopBackLeft,
            AV::TOP_BACK_CENTER => TopBackCenter,
            AV::TOP_BACK_RIGHT => TopBackRight,
            AV::STEREO_LEFT => StereoLeft,
            AV::STEREO_RIGHT => StereoRight,
            AV::WIDE_LEFT => WideLeft,
            AV::WIDE_RIGHT => WideRight,
            AV::SURROUND_DIRECT_LEFT => SurroundDirectLeft,
            AV::SURROUND_DIRECT_RIGHT => SurroundDirectRight,
            AV::LOW_FREQUENCY_2 => LowFrequency2,
            AV::TOP_SIDE_LEFT => TopSideLeft,
            AV::TOP_SIDE_RIGHT => TopSideRight,
            AV::BOTTOM_FRONT_CENTER => BottomFrontCenter,
            AV::BOTTOM_FRONT_LEFT => BottomFrontLeft,
            AV::BOTTOM_FRONT_RIGHT => BottomFrontRight,

            #[cfg(feature = "ffmpeg_7_1")]
            AV::SIDE_SURROUND_LEFT => SideSurroundLeft,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::SIDE_SURROUND_RIGHT => SideSurroundRight,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::TOP_SURROUND_LEFT => TopSurroundLeft,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::TOP_SURROUND_RIGHT => TopSurroundRight,

            #[cfg(feature = "ffmpeg_8_0")]
            AV::BINAURAL_LEFT => BinauralLeft,
            #[cfg(feature = "ffmpeg_8_0")]
            AV::BINAURAL_RIGHT => BinauralRight,

            AV::UNUSED => Unused,
            AV::UNKNOWN => Unknown,
            AV::AMBISONIC_BASE => AmbisonicBase,
            AV::AMBISONIC_END => AmbisonicEnd,

            // TODO: implement Ambisonic range
            _ => unimplemented!(),
        }
    }
}

impl From<Channel> for AVChannel {
    fn from(value: Channel) -> Self {
        use AVChannel as AV;
        use Channel::*;

        match value {
            None => AV::NONE,
            FrontLeft => AV::FRONT_LEFT,
            FrontRight => AV::FRONT_RIGHT,
            FrontCenter => AV::FRONT_CENTER,
            LowFrequency => AV::LOW_FREQUENCY,
            BackLeft => AV::BACK_LEFT,
            BackRight => AV::BACK_RIGHT,
            FrontLeftOfCenter => AV::FRONT_LEFT_OF_CENTER,
            FrontRightOfCenter => AV::FRONT_RIGHT_OF_CENTER,
            BackCenter => AV::BACK_CENTER,
            SideLeft => AV::SIDE_LEFT,
            SideRight => AV::SIDE_RIGHT,
            TopCenter => AV::TOP_CENTER,
            TopFrontLeft => AV::TOP_FRONT_LEFT,
            TopFrontCenter => AV::TOP_FRONT_CENTER,
            TopFrontRight => AV::TOP_FRONT_RIGHT,
            TopBackLeft => AV::TOP_BACK_LEFT,
            TopBackCenter => AV::TOP_BACK_CENTER,
            TopBackRight => AV::TOP_BACK_RIGHT,
            StereoLeft => AV::STEREO_LEFT,
            StereoRight => AV::STEREO_RIGHT,
            WideLeft => AV::WIDE_LEFT,
            WideRight => AV::WIDE_RIGHT,
            SurroundDirectLeft => AV::SURROUND_DIRECT_LEFT,
            SurroundDirectRight => AV::SURROUND_DIRECT_RIGHT,
            LowFrequency2 => AV::LOW_FREQUENCY_2,
            TopSideLeft => AV::TOP_SIDE_LEFT,
            TopSideRight => AV::TOP_SIDE_RIGHT,
            BottomFrontCenter => AV::BOTTOM_FRONT_CENTER,
            BottomFrontLeft => AV::BOTTOM_FRONT_LEFT,
            BottomFrontRight => AV::BOTTOM_FRONT_RIGHT,

            #[cfg(feature = "ffmpeg_7_1")]
            SideSurroundLeft => AV::SIDE_SURROUND_LEFT,
            #[cfg(feature = "ffmpeg_7_1")]
            SideSurroundRight => AV::SIDE_SURROUND_RIGHT,
            #[cfg(feature = "ffmpeg_7_1")]
            TopSurroundLeft => AV::TOP_SURROUND_LEFT,
            #[cfg(feature = "ffmpeg_7_1")]
            TopSurroundRight => AV::TOP_SURROUND_RIGHT,

            #[cfg(feature = "ffmpeg_8_0")]
            BinauralLeft => AV::BINAURAL_LEFT,
            #[cfg(feature = "ffmpeg_8_0")]
            BinauralRight => AV::BINAURAL_RIGHT,

            Unused => AV::UNUSED,
            Unknown => AV::UNKNOWN,
            AmbisonicBase => AV::AMBISONIC_BASE,
            AmbisonicEnd => AV::AMBISONIC_END,
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

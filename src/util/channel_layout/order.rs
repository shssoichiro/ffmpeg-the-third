use crate::ffi::AVChannelOrder;

use AVChannelOrder::*;
use ChannelOrder::*;

/// Specifies an order for audio channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelOrder {
    /// No channel order. Only the channel count is specified.
    Unspecified,

    /// Native channel order, i.e. the channels are in the same order in which they
    /// are defined in the [`Channel`][super::Channel] enum. This supports up to 63 channels.
    Native,

    /// The channel order does not correspond to any predefined order and is stored as an
    /// explicit map. This can be used to support layouts with more than 64 channels or with
    /// empty channels at arbitrary positions.
    Custom,

    /// The audio is represented as the decomposition of the sound field into spherical harmonics.
    Ambisonic,
}

impl From<AVChannelOrder> for ChannelOrder {
    fn from(value: AVChannelOrder) -> Self {
        match value {
            AV_CHANNEL_ORDER_UNSPEC => Unspecified,
            AV_CHANNEL_ORDER_NATIVE => Native,
            AV_CHANNEL_ORDER_CUSTOM => Custom,
            AV_CHANNEL_ORDER_AMBISONIC => Ambisonic,
            #[cfg(feature = "ffmpeg_7_0")]
            // Not part of the API, should never be used
            FF_CHANNEL_ORDER_NB => unreachable!(),
            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<ChannelOrder> for AVChannelOrder {
    fn from(value: ChannelOrder) -> Self {
        match value {
            Unspecified => AV_CHANNEL_ORDER_UNSPEC,
            Native => AV_CHANNEL_ORDER_NATIVE,
            Custom => AV_CHANNEL_ORDER_CUSTOM,
            Ambisonic => AV_CHANNEL_ORDER_AMBISONIC,
        }
    }
}

use crate::ffi::AVChannelOrder;

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
        use AVChannelOrder as AV;

        match value {
            AV::UNSPEC => Unspecified,
            AV::NATIVE => Native,
            AV::CUSTOM => Custom,
            AV::AMBISONIC => Ambisonic,

            _ => unimplemented!(),
        }
    }
}

impl From<ChannelOrder> for AVChannelOrder {
    fn from(value: ChannelOrder) -> Self {
        use AVChannelOrder as AV;

        match value {
            Unspecified => AV::UNSPEC,
            Native => AV::NATIVE,
            Custom => AV::CUSTOM,
            Ambisonic => AV::AMBISONIC,
        }
    }
}

use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum AudioService {
    Main,
    Effects,
    VisuallyImpaired,
    HearingImpaired,
    Dialogue,
    Commentary,
    Emergency,
    VoiceOver,
    Karaoke,
}

impl From<AVAudioServiceType> for AudioService {
    fn from(value: AVAudioServiceType) -> Self {
        use AVAudioServiceType as AV;

        match value {
            AV::MAIN => AudioService::Main,
            AV::EFFECTS => AudioService::Effects,
            AV::VISUALLY_IMPAIRED => AudioService::VisuallyImpaired,
            AV::HEARING_IMPAIRED => AudioService::HearingImpaired,
            AV::DIALOGUE => AudioService::Dialogue,
            AV::COMMENTARY => AudioService::Commentary,
            AV::EMERGENCY => AudioService::Emergency,
            AV::VOICE_OVER => AudioService::VoiceOver,
            AV::KARAOKE => AudioService::Karaoke,
            AV::NB => AudioService::Main,

            _ => unimplemented!(),
        }
    }
}

impl From<AudioService> for AVAudioServiceType {
    fn from(value: AudioService) -> AVAudioServiceType {
        use AVAudioServiceType as AV;

        match value {
            AudioService::Main => AV::MAIN,
            AudioService::Effects => AV::EFFECTS,
            AudioService::VisuallyImpaired => AV::VISUALLY_IMPAIRED,
            AudioService::HearingImpaired => AV::HEARING_IMPAIRED,
            AudioService::Dialogue => AV::DIALOGUE,
            AudioService::Commentary => AV::COMMENTARY,
            AudioService::Emergency => AV::EMERGENCY,
            AudioService::VoiceOver => AV::VOICE_OVER,
            AudioService::Karaoke => AV::KARAOKE,
        }
    }
}

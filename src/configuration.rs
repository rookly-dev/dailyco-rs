//! Miscellaneous enum definitions for `Daily` configuration options.
//!
use serde::{Deserialize, Serialize};

/// Signaling server region for hosting a call
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Region {
    /// Cape Town
    AfSouth1,
    /// Seoul
    ApNortheast2,
    /// Singapore
    ApSoutheast1,
    /// Sydney
    ApSoutheast2,
    /// Mumbai
    ApSouth1,
    /// Frankfurt
    EuCentral1,
    /// London
    EuWest2,
    /// Sao Paulo
    SaEast1,
    /// N. Virginia
    UsEast1,
    /// Oregon
    UsWest2,
}

/// Used to select the region where an RTMP stream should originate.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum RtmpGeoRegion {
    /// Oregon
    UsWest2,
    /// Frankfurt
    EuCentral1,
    /// Singapore
    ApSoutheast1,
}

/// Options for the default language of a Daily room
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum DailyLang {
    De,
    En,
    Es,
    Fi,
    Fr,
    It,
    Jp,
    Ka,
    Nl,
    No,
    Pt,
    Pl,
    Ru,
    Sv,
    Tr,
    /// Default to user language preferences
    User,
}

impl Default for DailyLang {
    /// Matching Daily documented default
    fn default() -> Self {
        Self::En
    }
}

/// Options for a recording type, details provided
/// [here](https://docs.daily.co/reference/rest-api/rooms/config#enable_recording)
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum RecordingType {
    /// Cloud recording, done server-side.
    Cloud,
    /// `rtp-tracks` captures each individual media track from a call separately.
    RtpTracks,
    /// `output-byte-stream` records byte-level data locally.
    OutputByteStream,
    /// Record to participant's device.
    Local,
}

/// Signaling type, seen in <https://docs.daily.co/reference/rest-api/rooms/config#signaling_imp>
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SignalingImp {
    /// `ws` signaling type
    Ws,
}

impl Default for SignalingImp {
    fn default() -> Self {
        Self::Ws
    }
}

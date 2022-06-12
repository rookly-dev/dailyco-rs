use crate::configuration::{DailyLang, RecordingType, Region, RtmpGeoRegion, SignalingImp};
use serde::Deserialize;

const fn default_as_true() -> bool {
    true
}

/// Properties for a `Daily` room, defined [here](https://docs.daily.co/reference/rest-api/rooms/config).
/// Following the API docs, fields not found in a request are assumed to have their
/// default values.
#[derive(Debug, Clone, Deserialize)]
pub struct RoomProperties {
    /// UTC timestamp before which the room cannot be joined
    pub nbf: Option<i64>,
    /// UTC timestamp for expiration of the room, after which
    /// time it will be automatically deleted at some point.
    pub exp: Option<i64>,
    /// Maximum number of participants who can enter the room.
    pub max_participants: Option<usize>,
    /// Determines if Daily Prebuilt displays the People UI
    pub enable_people_ui: Option<bool>,
    /// Sets whether the room can use Daily Prebuilt's Picture in Picture controls.
    #[serde(default)]
    pub enable_pip_ui: bool,
    /// Determines whether participants enter a waiting room with a camera, mic, and
    /// browser check before joining a call.
    pub enable_prejoin_ui: Option<bool>,
    /// Determines whether the network button, and the network panel it reveals on click, appears in this room.
    #[serde(default)]
    pub enable_network_ui: bool,
    /// Turns on a lobby experience for private rooms. A participant without a corresponding
    /// meeting token can request to be admitted to the meeting with a "knock", and wait
    /// for the meeting owner to admit them.
    #[serde(default)]
    pub enable_knocking: bool,
    /// Whether or not screen-sharing is enabled
    #[serde(default = "default_as_true")]
    pub enable_screenshare: bool,
    /// Determines whether Daily Prebuilt displays background blur controls.
    #[serde(default = "default_as_true")]
    pub enable_video_processing_ui: bool,
    /// Allow adding chat to the call
    #[serde(default)]
    pub enable_chat: bool,
    /// Keep video off when room is joined
    #[serde(default)]
    pub start_video_off: bool,
    /// Keep audio off when room is joined
    #[serde(default)]
    pub start_audio_off: bool,
    /// In Daily Prebuilt, only the meeting owners will be able to turn on camera,
    /// unmute mic, and share screen
    #[serde(default)]
    pub owner_only_broadcast: bool,
    /// Allowed recording type for the room
    pub enable_recording: Option<RecordingType>,
    /// If there's a meeting going on at room exp time, end the meeting by kicking
    /// everyone out. This behavior can be overridden by setting eject properties of
    /// a meeting token.
    #[serde(default)]
    pub eject_at_room_exp: bool,
    /// Eject a meeting participant this many seconds after the participant joins the
    /// meeting. You can use this is a default length limit to prevent long meetings.
    /// This can be overridden by setting eject properties of a meeting token.
    pub eject_after_elapsed: Option<i64>,
    /// When enabled, non-owner users join a meeting with a hidden presence, meaning
    /// they won't appear as a named participant in the meeting and have no participant
    /// events associated to them.
    #[serde(default)]
    pub enable_hidden_participants: bool,
    /// Configures a room to use multiple SFUs for a call's media. This feature enables
    /// calls to scale to large sizes and to reduce latency between participants.
    /// It is recommended specifically for interactive live streaming.
    pub enable_mesh_sfu: Option<bool>,
    /// Enables Daily Prebuilt to support group calls of up to 300 participants and
    /// owner only broadcast calls of up to 15K participants.
    pub experimental_optimize_large_calls: Option<bool>,
    /// The default language of the Daily prebuilt video call UI, for this room.
    #[serde(default)]
    pub lang: DailyLang,
    /// Sets a URL that will receive a webhook when a user joins a room.
    /// Default is NULL. Character limit for webhook URL is 255.
    pub meeting_join_hook: Option<String>,
    /// Sets the signaling type.
    #[serde(default)]
    pub signaling_imp: SignalingImp,
    /// Enforce a signaling server region
    pub geo: Option<Region>,
    /// Used to select the region where an RTMP stream should originate.
    pub rtmp_geo: Option<RtmpGeoRegion>,
    /// Reduces the volume of log messages. This feature should be enabled when there
    /// are more than 300 participants in a meeting to help improve performance.
    #[serde(default)]
    pub enable_terse_logging: bool,
}

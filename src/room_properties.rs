use serde::{Deserialize, Serialize};

use crate::configuration::{
    DailyLang, RecordingType, RecordingsBucket, Region, RtmpGeoRegion, SignalingImp,
};
use crate::utils::default_as_true;

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
    /// are more than 200 participants in a meeting to help improve performance.
    #[serde(default)]
    pub enable_terse_logging: bool,
    /// See details in the [docs](https://docs.daily.co/reference/rest-api/rooms/config#recordings_template).
    pub recordings_template: Option<String>,
    /// Configures an S3 bucket in which to store recordings.
    pub recordings_bucket: Option<RecordingsBucket>,
    /// Dictates the participant count after which room topology automatically
    /// switches from Peer-to-Peer (P2P) to Selective Forwarding Unit (SFU) mode, or vice versa.
    pub sfu_switchover: Option<f64>,
}

/// A builder to specify properties for a `Daily` room,
/// defined [here](https://docs.daily.co/reference/rest-api/rooms/config).
#[derive(Debug, Copy, Clone, Serialize, Default)]
pub struct RoomPropertiesBuilder<'a> {
    /// UTC timestamp before which the room cannot be joined
    #[serde(skip_serializing_if = "Option::is_none")]
    nbf: Option<i64>,
    /// UTC timestamp for expiration of the room, after which
    /// time it will be automatically deleted at some point.
    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<i64>,
    /// Maximum number of participants who can enter the room.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_participants: Option<usize>,
    /// Determines if Daily Prebuilt displays the People UI
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_people_ui: Option<bool>,
    /// Sets whether the room can use Daily Prebuilt's Picture in Picture controls.
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_pip_ui: Option<bool>,
    /// Determines whether participants enter a waiting room with a camera, mic, and
    /// browser check before joining a call.
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_prejoin_ui: Option<bool>,
    /// Determines whether the network button, and the network panel it reveals on click, appears in this room.
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_network_ui: Option<bool>,
    /// Turns on a lobby experience for private rooms. A participant without a corresponding
    /// meeting token can request to be admitted to the meeting with a "knock", and wait
    /// for the meeting owner to admit them.
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_knocking: Option<bool>,
    /// Whether or not screen-sharing is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_screenshare: Option<bool>,
    /// Determines whether Daily Prebuilt displays background blur controls.
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_video_processing_ui: Option<bool>,
    /// Allow adding chat to the call
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_chat: Option<bool>,
    /// Keep video off when room is joined
    #[serde(skip_serializing_if = "Option::is_none")]
    start_video_off: Option<bool>,
    /// Keep audio off when room is joined
    #[serde(skip_serializing_if = "Option::is_none")]
    start_audio_off: Option<bool>,
    /// In Daily Prebuilt, only the meeting owners will be able to turn on camera,
    /// unmute mic, and share screen
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_only_broadcast: Option<bool>,
    /// Allowed recording type for the room
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_recording: Option<RecordingType>,
    /// If there's a meeting going on at room exp time, end the meeting by kicking
    /// everyone out. This behavior can be overridden by setting eject properties of
    /// a meeting token.
    #[serde(skip_serializing_if = "Option::is_none")]
    eject_at_room_exp: Option<bool>,
    /// Eject a meeting participant this many seconds after the participant joins the
    /// meeting. You can use this is a default length limit to prevent long meetings.
    /// This can be overridden by setting eject properties of a meeting token.
    #[serde(skip_serializing_if = "Option::is_none")]
    eject_after_elapsed: Option<i64>,
    /// When enabled, non-owner users join a meeting with a hidden presence, meaning
    /// they won't appear as a named participant in the meeting and have no participant
    /// events associated to them.
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_hidden_participants: Option<bool>,
    /// Configures a room to use multiple SFUs for a call's media. This feature enables
    /// calls to scale to large sizes and to reduce latency between participants.
    /// It is recommended specifically for interactive live streaming.
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_mesh_sfu: Option<bool>,
    /// Enables Daily Prebuilt to support group calls of up to 300 participants and
    /// owner only broadcast calls of up to 15K participants.
    #[serde(skip_serializing_if = "Option::is_none")]
    experimental_optimize_large_calls: Option<bool>,
    /// The default language of the Daily prebuilt video call UI, for this room.
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<DailyLang>,
    /// Sets a URL that will receive a webhook when a user joins a room.
    /// Default is NULL. Character limit for webhook URL is 255.
    #[serde(skip_serializing_if = "Option::is_none")]
    meeting_join_hook: Option<&'a str>,
    /// Sets the signaling type.
    #[serde(skip_serializing_if = "Option::is_none")]
    signaling_imp: Option<SignalingImp>,
    /// Enforce a signaling server region
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<Region>,
    /// Used to select the region where an RTMP stream should originate.
    #[serde(skip_serializing_if = "Option::is_none")]
    rtmp_geo: Option<RtmpGeoRegion>,
    /// Reduces the volume of log messages. This feature should be enabled when there
    /// are more than 200 participants in a meeting to help improve performance.
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_terse_logging: Option<bool>,
    /// See details in the [docs](https://docs.daily.co/reference/rest-api/rooms/config#recordings_template).
    #[serde(skip_serializing_if = "Option::is_none")]
    recordings_template: Option<&'a str>,
    /// Configures an S3 bucket in which to store recordings.
    #[serde(skip_serializing_if = "Option::is_none")]
    recordings_bucket: Option<&'a RecordingsBucket>,
    /// Dictates the participant count after which room topology automatically
    /// switches from Peer-to-Peer (P2P) to Selective Forwarding Unit (SFU) mode, or vice versa.
    #[serde(skip_serializing_if = "Option::is_none")]
    sfu_switchover: Option<f64>,
}

impl<'a> RoomPropertiesBuilder<'a> {
    /// Start a new `RoomPropertiesBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// UTC timestamp before which the room cannot be joined.
    pub fn nbf(mut self, nbf: i64) -> Self {
        self.nbf = Some(nbf);
        self
    }

    /// UTC timestamp for expiration of the room, after which
    /// time it will be automatically deleted at some point.
    pub fn exp(mut self, exp: i64) -> Self {
        self.exp = Some(exp);
        self
    }

    /// Maximum number of participants who can enter the room.
    pub fn max_participants(mut self, max_participants: usize) -> Self {
        self.max_participants = Some(max_participants);
        self
    }

    /// Determines if `Daily Prebuilt` displays the People UI.
    pub fn enable_people_ui(mut self, enable_people_ui: bool) -> Self {
        self.enable_people_ui = Some(enable_people_ui);
        self
    }

    /// Sets whether the room can use `Daily Prebuilt's` Picture in Picture controls.
    pub fn enable_pip_ui(mut self, enable_pip_ui: bool) -> Self {
        self.enable_pip_ui = Some(enable_pip_ui);
        self
    }

    /// Determines whether participants enter a waiting room with a camera, mic, and
    /// browser check before joining a call.
    pub fn enable_prejoin_ui(mut self, enable_prejoin_ui: bool) -> Self {
        self.enable_prejoin_ui = Some(enable_prejoin_ui);
        self
    }

    /// Determines whether the network button, and the network panel it reveals on click, appears in this room.
    pub fn enable_network_ui(mut self, enable_network_ui: bool) -> Self {
        self.enable_network_ui = Some(enable_network_ui);
        self
    }

    /// Turns on a lobby experience for private rooms. A participant without a corresponding
    /// meeting token can request to be admitted to the meeting with a "knock", and wait
    /// for the meeting owner to admit them.
    pub fn enable_knocking(mut self, enable_knocking: bool) -> Self {
        self.enable_knocking = Some(enable_knocking);
        self
    }

    /// Whether or not screen-sharing is enabled
    pub fn enable_screenshare(mut self, enable_screenshare: bool) -> Self {
        self.enable_screenshare = Some(enable_screenshare);
        self
    }

    /// Determines whether `Daily Prebuilt` displays background blur controls.
    pub fn enable_video_processing_ui(mut self, enable_video_processing_ui: bool) -> Self {
        self.enable_video_processing_ui = Some(enable_video_processing_ui);
        self
    }

    /// Allow adding chat to the call
    pub fn enable_chat(mut self, enable_chat: bool) -> Self {
        self.enable_chat = Some(enable_chat);
        self
    }

    /// Keep video off when room is joined
    pub fn start_video_off(mut self, start_video_off: bool) -> Self {
        self.start_video_off = Some(start_video_off);
        self
    }

    /// Keep audio off when room is joined
    pub fn start_audio_off(mut self, start_audio_off: bool) -> Self {
        self.start_audio_off = Some(start_audio_off);
        self
    }

    /// In `Daily Prebuilt`, only the meeting owners will be able to turn on camera,
    /// unmute mic, and share screen
    pub fn owner_only_broadcast(mut self, owner_only_broadcast: bool) -> Self {
        self.owner_only_broadcast = Some(owner_only_broadcast);
        self
    }

    /// Allowed recording type for the room
    pub fn enable_recording(mut self, enable_recording: RecordingType) -> Self {
        self.enable_recording = Some(enable_recording);
        self
    }

    /// If there's a meeting going on at room exp time, end the meeting by kicking
    /// everyone out. This behavior can be overridden by setting eject properties of
    /// a meeting token.
    pub fn eject_at_room_exp(mut self, eject_at_room_exp: bool) -> Self {
        self.eject_at_room_exp = Some(eject_at_room_exp);
        self
    }

    /// Eject a meeting participant this many seconds after the participant joins the
    /// meeting. You can use this is a default length limit to prevent long meetings.
    /// This can be overridden by setting eject properties of a meeting token.
    pub fn eject_after_elapsed(mut self, eject_after_elapsed: i64) -> Self {
        self.eject_after_elapsed = Some(eject_after_elapsed);
        self
    }

    /// When enabled, non-owner users join a meeting with a hidden presence, meaning
    /// they won't appear as a named participant in the meeting and have no participant
    /// events associated to them.
    pub fn enable_hidden_participants(mut self, enable_hidden_participants: bool) -> Self {
        self.enable_hidden_participants = Some(enable_hidden_participants);
        self
    }

    /// Configures a room to use multiple SFUs for a call's media. This feature enables
    /// calls to scale to large sizes and to reduce latency between participants.
    /// It is recommended specifically for interactive live streaming.
    pub fn enable_mesh_sfu(mut self, enable_mesh_sfu: bool) -> Self {
        self.enable_mesh_sfu = Some(enable_mesh_sfu);
        self
    }

    /// Enables Daily Prebuilt to support group calls of up to 300 participants and
    /// owner only broadcast calls of up to 15K participants.
    pub fn experimental_optimize_large_calls(
        mut self,
        experimental_optimize_large_calls: bool,
    ) -> Self {
        self.experimental_optimize_large_calls = Some(experimental_optimize_large_calls);
        self
    }

    /// The default language of the Daily prebuilt video call UI, for this room.
    pub fn lang(mut self, lang: DailyLang) -> Self {
        self.lang = Some(lang);
        self
    }

    /// Sets a URL that will receive a webhook when a user joins a room.
    /// Default is NULL. Character limit for webhook URL is 255.
    pub fn meeting_join_hook(mut self, meeting_join_hook: &'a str) -> Self {
        self.meeting_join_hook = Some(meeting_join_hook);
        self
    }

    /// Sets the signaling type.
    pub fn signaling_imp(mut self, signaling_imp: SignalingImp) -> Self {
        self.signaling_imp = Some(signaling_imp);
        self
    }

    /// Enforce a signaling server region
    pub fn geo(mut self, geo: Region) -> Self {
        self.geo = Some(geo);
        self
    }

    /// Used to select the region where an RTMP stream should originate.
    pub fn rtmp_geo(mut self, rtmp_geo: RtmpGeoRegion) -> Self {
        self.rtmp_geo = Some(rtmp_geo);
        self
    }

    /// Reduces the volume of log messages. This feature should be enabled when there
    /// are more than 300 participants in a meeting to help improve performance.
    pub fn enable_terse_logging(mut self, enable_terse_logging: bool) -> Self {
        self.enable_terse_logging = Some(enable_terse_logging);
        self
    }

    /// See details in the [docs](https://docs.daily.co/reference/rest-api/rooms/config#recordings_template).
    pub fn recordings_template(mut self, recordings_template: &'a str) -> Self {
        self.recordings_template = Some(recordings_template);
        self
    }

    /// Dictates the participant count after which room topology automatically
    /// switches from Peer-to-Peer (P2P) to Selective Forwarding Unit (SFU) mode, or vice versa.
    pub fn sfu_switchover(mut self, sfu_switchover: f64) -> Self {
        self.sfu_switchover = Some(sfu_switchover);
        self
    }

    /// Configures an S3 bucket in which to store recordings.
    pub fn recordings_bucket(mut self, recordings_bucket: &'a RecordingsBucket) -> Self {
        self.recordings_bucket = Some(recordings_bucket);
        self
    }

    /// Ensure the room always immediately switches to SFU. Equivalent to setting
    /// `sfu_switchover` to 0.5
    pub fn sfu_always(mut self) -> Self {
        self.sfu_switchover = Some(0.5);
        self
    }
}

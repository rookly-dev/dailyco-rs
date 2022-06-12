//! Definition and creation of `Daily` rooms.
use crate::configuration::{DailyLang, RecordingType, Region, RtmpGeoRegion, SignalingImp};
use crate::room_properties::RoomProperties;
use crate::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
/// Possibilities for video room visibility.
pub enum RoomPrivacy {
    /// Public room (anyone can join)
    Public,
    /// Private room (need token to join, or owner approval)
    Private,
}

impl Default for RoomPrivacy {
    fn default() -> Self {
        // Matching dailyco default
        Self::Public
    }
}

/// A `RoomBuilder` can be used to create a `Daily` room with custom configuration.
#[serde_with::skip_serializing_none]
#[derive(Debug, Copy, Clone, Serialize, Default)]
pub struct RoomBuilder<'a> {
    name: Option<&'a str>,
    privacy: Option<RoomPrivacy>,
    properties: RoomPropertiesBuilder<'a>,
}

impl<'a> RoomBuilder<'a> {
    /// Constructs a new `RoomBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the name the room will be created with. `Daily`
    /// will randomly generate name if not provided.
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }

    /// Set the visibility for the room.
    pub fn privacy(&mut self, privacy: RoomPrivacy) -> &mut Self {
        self.privacy = Some(privacy);
        self
    }

    /// UTC timestamp before which the room cannot be joined.
    pub fn nbf(&mut self, nbf: i64) -> &mut Self {
        self.properties.nbf = Some(nbf);
        self
    }

    /// UTC timestamp for expiration of the room, after which
    /// time it will be automatically deleted at some point.
    pub fn exp(&mut self, exp: i64) -> &mut Self {
        self.properties.exp = Some(exp);
        self
    }

    /// Maximum number of participants who can enter the room.
    pub fn max_participants(&mut self, max_participants: usize) -> &mut Self {
        self.properties.max_participants = Some(max_participants);
        self
    }

    /// Determines if `Daily Prebuilt` displays the People UI.
    pub fn enable_people_ui(&mut self, enable_people_ui: bool) -> &mut Self {
        self.properties.enable_people_ui = Some(enable_people_ui);
        self
    }

    /// Sets whether the room can use `Daily Prebuilt's` Picture in Picture controls.
    pub fn enable_pip_ui(&mut self, enable_pip_ui: bool) -> &mut Self {
        self.properties.enable_pip_ui = Some(enable_pip_ui);
        self
    }

    /// Determines whether participants enter a waiting room with a camera, mic, and
    /// browser check before joining a call.
    pub fn enable_prejoin_ui(&mut self, enable_prejoin_ui: bool) -> &mut Self {
        self.properties.enable_prejoin_ui = Some(enable_prejoin_ui);
        self
    }

    /// Determines whether the network button, and the network panel it reveals on click, appears in this room.
    pub fn enable_network_ui(&mut self, enable_network_ui: bool) -> &mut Self {
        self.properties.enable_network_ui = Some(enable_network_ui);
        self
    }

    /// Turns on a lobby experience for private rooms. A participant without a corresponding
    /// meeting token can request to be admitted to the meeting with a "knock", and wait
    /// for the meeting owner to admit them.
    pub fn enable_knocking(&mut self, enable_knocking: bool) -> &mut Self {
        self.properties.enable_knocking = Some(enable_knocking);
        self
    }

    /// Whether or not screen-sharing is enabled
    pub fn enable_screenshare(&mut self, enable_screenshare: bool) -> &mut Self {
        self.properties.enable_screenshare = Some(enable_screenshare);
        self
    }

    /// Determines whether `Daily Prebuilt` displays background blur controls.
    pub fn enable_video_processing_ui(&mut self, enable_video_processing_ui: bool) -> &mut Self {
        self.properties.enable_video_processing_ui = Some(enable_video_processing_ui);
        self
    }

    /// Allow adding chat to the call
    pub fn enable_chat(&mut self, enable_chat: bool) -> &mut Self {
        self.properties.enable_chat = Some(enable_chat);
        self
    }

    /// Keep video off when room is joined
    pub fn start_video_off(&mut self, start_video_off: bool) -> &mut Self {
        self.properties.start_video_off = Some(start_video_off);
        self
    }

    /// Keep audio off when room is joined
    pub fn start_audio_off(&mut self, start_audio_off: bool) -> &mut Self {
        self.properties.start_audio_off = Some(start_audio_off);
        self
    }

    /// In `Daily Prebuilt`, only the meeting owners will be able to turn on camera,
    /// unmute mic, and share screen
    pub fn owner_only_broadcast(&mut self, owner_only_broadcast: bool) -> &mut Self {
        self.properties.owner_only_broadcast = Some(owner_only_broadcast);
        self
    }

    /// Allowed recording type for the room
    pub fn enable_recording(&mut self, enable_recording: RecordingType) -> &mut Self {
        self.properties.enable_recording = Some(enable_recording);
        self
    }

    /// If there's a meeting going on at room exp time, end the meeting by kicking
    /// everyone out. This behavior can be overridden by setting eject properties of
    /// a meeting token.
    pub fn eject_at_room_exp(&mut self, eject_at_room_exp: bool) -> &mut Self {
        self.properties.eject_at_room_exp = Some(eject_at_room_exp);
        self
    }

    /// Eject a meeting participant this many seconds after the participant joins the
    /// meeting. You can use this is a default length limit to prevent long meetings.
    /// This can be overridden by setting eject properties of a meeting token.
    pub fn eject_after_elapsed(&mut self, eject_after_elapsed: i64) -> &mut Self {
        self.properties.eject_after_elapsed = Some(eject_after_elapsed);
        self
    }

    /// When enabled, non-owner users join a meeting with a hidden presence, meaning
    /// they won't appear as a named participant in the meeting and have no participant
    /// events associated to them.
    pub fn enable_hidden_participants(&mut self, enable_hidden_participants: bool) -> &mut Self {
        self.properties.enable_hidden_participants = Some(enable_hidden_participants);
        self
    }

    /// Configures a room to use multiple SFUs for a call's media. This feature enables
    /// calls to scale to large sizes and to reduce latency between participants.
    /// It is recommended specifically for interactive live streaming.
    pub fn enable_mesh_sfu(&mut self, enable_mesh_sfu: bool) -> &mut Self {
        self.properties.enable_mesh_sfu = Some(enable_mesh_sfu);
        self
    }

    /// Enables Daily Prebuilt to support group calls of up to 300 participants and
    /// owner only broadcast calls of up to 15K participants.
    pub fn experimental_optimize_large_calls(
        &mut self,
        experimental_optimize_large_calls: bool,
    ) -> &mut Self {
        self.properties.experimental_optimize_large_calls = Some(experimental_optimize_large_calls);
        self
    }

    /// The default language of the Daily prebuilt video call UI, for this room.
    pub fn lang(&mut self, lang: DailyLang) -> &mut Self {
        self.properties.lang = Some(lang);
        self
    }

    /// Sets a URL that will receive a webhook when a user joins a room.
    /// Default is NULL. Character limit for webhook URL is 255.
    pub fn meeting_join_hook(&mut self, meeting_join_hook: &'a str) -> &mut Self {
        self.properties.meeting_join_hook = Some(meeting_join_hook);
        self
    }

    /// Sets the signaling type.
    pub fn signaling_imp(&mut self, signaling_imp: SignalingImp) -> &mut Self {
        self.properties.signaling_imp = Some(signaling_imp);
        self
    }

    /// Enforce a signaling server region
    pub fn geo(&mut self, geo: Region) -> &mut Self {
        self.properties.geo = Some(geo);
        self
    }

    /// Used to select the region where an RTMP stream should originate.
    pub fn rtmp_geo(&mut self, rtmp_geo: RtmpGeoRegion) -> &mut Self {
        self.properties.rtmp_geo = Some(rtmp_geo);
        self
    }

    /// Reduces the volume of log messages. This feature should be enabled when there
    /// are more than 300 participants in a meeting to help improve performance.
    pub fn enable_terse_logging(&mut self, enable_terse_logging: bool) -> &mut Self {
        self.properties.enable_terse_logging = Some(enable_terse_logging);
        self
    }

    /// Make the request to create the `Daily` room.
    ///
    /// # Examples
    ///
    /// Create a private room with audio and video off by default:
    ///
    /// ```no_run
    /// # use dailyco::Client;
    /// # use dailyco::room::{RoomBuilder, RoomPrivacy};
    /// # async fn run() {
    /// let client = Client::new("test-api-key").unwrap();
    /// RoomBuilder::new()
    ///   .privacy(RoomPrivacy::Private)
    ///   .start_audio_off(true)
    ///   .start_video_off(true)
    ///   .create(&client)
    ///   .await
    ///   .expect("Should create room");
    /// # }
    /// ```
    pub async fn create(&self, client: &Client) -> crate::Result<Room> {
        client.create_room(self).await
    }
}

/// Room object metadata as reported by `Daily`.
#[derive(Debug, Clone, Deserialize)]
pub struct Room {
    /// The id for this room.
    pub id: String,
    /// The name of this room.
    pub name: String,
    /// Whether the room was created programmatically or through the dashboard.
    pub api_created: bool,
    /// The visibility of this room.
    pub privacy: RoomPrivacy,
    /// The URL which can be used to join the room.
    pub url: String,
    // TODO: could be parsed directly as datetime if we depended on `chrono`
    /// Creation datetime.
    pub created_at: String,
    /// Configuration options for this room.
    pub config: RoomProperties,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Copy, Clone, Serialize, Default)]
struct RoomPropertiesBuilder<'a> {
    /// UTC timestamp before which the room cannot be joined
    nbf: Option<i64>,
    /// UTC timestamp for expiration of the room, after which
    /// time it will be automatically deleted at some point.
    exp: Option<i64>,
    /// Maximum number of participants who can enter the room.
    max_participants: Option<usize>,
    /// Determines if Daily Prebuilt displays the People UI
    enable_people_ui: Option<bool>,
    /// Sets whether the room can use Daily Prebuilt's Picture in Picture controls.
    enable_pip_ui: Option<bool>,
    /// Determines whether participants enter a waiting room with a camera, mic, and
    /// browser check before joining a call.
    enable_prejoin_ui: Option<bool>,
    /// Determines whether the network button, and the network panel it reveals on click, appears in this room.
    enable_network_ui: Option<bool>,
    /// Turns on a lobby experience for private rooms. A participant without a corresponding
    /// meeting token can request to be admitted to the meeting with a "knock", and wait
    /// for the meeting owner to admit them.
    enable_knocking: Option<bool>,
    /// Whether or not screen-sharing is enabled
    enable_screenshare: Option<bool>,
    /// Determines whether Daily Prebuilt displays background blur controls.
    enable_video_processing_ui: Option<bool>,
    /// Allow adding chat to the call
    enable_chat: Option<bool>,
    /// Keep video off when room is joined
    start_video_off: Option<bool>,
    /// Keep audio off when room is joined
    start_audio_off: Option<bool>,
    /// In Daily Prebuilt, only the meeting owners will be able to turn on camera,
    /// unmute mic, and share screen
    owner_only_broadcast: Option<bool>,
    /// Allowed recording type for the room
    enable_recording: Option<RecordingType>,
    /// If there's a meeting going on at room exp time, end the meeting by kicking
    /// everyone out. This behavior can be overridden by setting eject properties of
    /// a meeting token.
    eject_at_room_exp: Option<bool>,
    /// Eject a meeting participant this many seconds after the participant joins the
    /// meeting. You can use this is a default length limit to prevent long meetings.
    /// This can be overridden by setting eject properties of a meeting token.
    eject_after_elapsed: Option<i64>,
    /// When enabled, non-owner users join a meeting with a hidden presence, meaning
    /// they won't appear as a named participant in the meeting and have no participant
    /// events associated to them.
    enable_hidden_participants: Option<bool>,
    /// Configures a room to use multiple SFUs for a call's media. This feature enables
    /// calls to scale to large sizes and to reduce latency between participants.
    /// It is recommended specifically for interactive live streaming.
    enable_mesh_sfu: Option<bool>,
    /// Enables Daily Prebuilt to support group calls of up to 300 participants and
    /// owner only broadcast calls of up to 15K participants.
    experimental_optimize_large_calls: Option<bool>,
    /// The default language of the Daily prebuilt video call UI, for this room.
    lang: Option<DailyLang>,
    /// Sets a URL that will receive a webhook when a user joins a room.
    /// Default is NULL. Character limit for webhook URL is 255.
    meeting_join_hook: Option<&'a str>,
    /// Sets the signaling type.
    signaling_imp: Option<SignalingImp>,
    /// Enforce a signaling server region
    geo: Option<Region>,
    /// Used to select the region where an RTMP stream should originate.
    rtmp_geo: Option<RtmpGeoRegion>,
    /// Reduces the volume of log messages. This feature should be enabled when there
    /// are more than 300 participants in a meeting to help improve performance.
    enable_terse_logging: Option<bool>,
}

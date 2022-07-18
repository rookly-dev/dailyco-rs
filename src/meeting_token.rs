//! Definition and creation of `Daily` meeting tokens.
use crate::configuration::{DailyLang, RecordingType};
use crate::utils::default_as_true;
use crate::Client;
use serde::{Deserialize, Serialize};

/// A `MeetingTokenBuilder` can be used to create a `Daily` meeting token for gaining
/// access to a private room.
#[derive(Debug, Clone, Serialize, Default)]
pub struct MeetingTokenBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    room_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eject_at_token_exp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eject_after_elapsed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nbf: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_screenshare: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_video_off: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_audio_off: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_recording: Option<RecordingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_prejoin_ui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_terse_logging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cloud_recording: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_tab_on_exit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_on_meeting_exit: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<DailyLang>,
}

impl<'a> MeetingTokenBuilder<'a> {
    /// Constructs a new `MeetingTokenBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The room for which this token is valid. If `room_name` isn't set, the token is
    /// valid for all rooms in your domain.
    pub fn room_name(&mut self, room_name: &'a str) -> &mut Self {
        self.room_name = Some(room_name);
        self
    }

    /// Kick this user out of the meeting at the time this meeting token expires.
    pub fn eject_at_token_exp(&mut self, eject_at_token_exp: bool) -> &mut Self {
        self.eject_at_token_exp = Some(eject_at_token_exp);
        self
    }

    /// Kick this user out of the meeting this many seconds after they join the meeting.
    pub fn eject_after_elapsed(&mut self, eject_after_elapsed: i64) -> &mut Self {
        self.eject_after_elapsed = Some(eject_after_elapsed);
        self
    }

    /// UTC timestamp before which the token cannot be used.
    pub fn nbf(&mut self, nbf: i64) -> &mut Self {
        self.nbf = Some(nbf);
        self
    }

    /// UTC timestamp for expiration of the token.
    pub fn exp(&mut self, exp: i64) -> &mut Self {
        self.exp = Some(exp);
        self
    }

    /// The user has meeting owner privileges.
    pub fn is_owner(&mut self, is_owner: bool) -> &mut Self {
        self.is_owner = Some(is_owner);
        self
    }

    /// The user's name in this meeting.
    pub fn user_name(&mut self, user_name: &'a str) -> &mut Self {
        self.user_name = Some(user_name);
        self
    }

    /// The user's id for this meeting session.
    pub fn user_id(&mut self, user_id: &'a str) -> &mut Self {
        self.user_id = Some(user_id);
        self
    }

    /// The user is allowed to screenshare.
    pub fn enable_screenshare(&mut self, enable_screenshare: bool) -> &mut Self {
        self.enable_screenshare = Some(enable_screenshare);
        self
    }

    /// When a participant first joins a meeting, keep their camera off.
    pub fn start_video_off(&mut self, start_video_off: bool) -> &mut Self {
        self.start_video_off = Some(start_video_off);
        self
    }

    /// When a participant first joins a meeting, keep their microphone muted.
    pub fn start_audio_off(&mut self, start_audio_off: bool) -> &mut Self {
        self.start_audio_off = Some(start_audio_off);
        self
    }

    /// Allowed recording type
    pub fn enable_recording(&mut self, enable_recording: RecordingType) -> &mut Self {
        self.enable_recording = Some(enable_recording);
        self
    }

    /// Determines whether participant enters a waiting room with a camera, mic, and
    /// browser check before joining a call.
    pub fn enable_prejoin_ui(&mut self, enable_prejoin_ui: bool) -> &mut Self {
        self.enable_prejoin_ui = Some(enable_prejoin_ui);
        self
    }

    /// Reduces the volume of log messages. This feature should be enabled when there
    /// are more than 300 participants in a meeting to help improve performance.
    pub fn enable_terse_logging(&mut self, enable_terse_logging: bool) -> &mut Self {
        self.enable_terse_logging = Some(enable_terse_logging);
        self
    }

    /// Start cloud recording when the user joins the room. This can be used to always record and
    /// archive meetings, for example in a customer support context.
    pub fn start_cloud_recording(&mut self, start_cloud_recording: bool) -> &mut Self {
        self.start_cloud_recording = Some(start_cloud_recording);
        self
    }

    /// When a user leaves a meeting using the button in the in-call menu bar,
    /// the browser tab closes.
    pub fn close_tab_on_exit(&mut self, close_tab_on_exit: bool) -> &mut Self {
        self.close_tab_on_exit = Some(close_tab_on_exit);
        self
    }

    /// When a user leaves a meeting using the button in the in-call menu bar,
    /// the browser loads this URL.
    pub fn redirect_on_meeting_exit(&mut self, redirect_on_meeting_exit: &'a str) -> &mut Self {
        self.redirect_on_meeting_exit = Some(redirect_on_meeting_exit);
        self
    }

    /// The default language of the Daily prebuilt video call UI, for this room.
    pub fn lang(&mut self, lang: DailyLang) -> &mut Self {
        self.lang = Some(lang);
        self
    }

    /// Make the request to create the custom `Daily` meeting token for joining a room.
    ///
    /// # Examples
    ///
    /// Create a token to join a room with owner privileges.
    ///
    /// ```no_run
    /// # use dailyco::{Client, Result};
    /// # use dailyco::meeting_token::MeetingTokenBuilder;
    /// # async fn run() -> Result<String> {
    /// let client = Client::new("test-api-key")?;
    /// let token = MeetingTokenBuilder::new()
    ///   .room_name("room-user-should-own")
    ///   .is_owner(true)
    ///   .create(&client)
    ///   .await?;
    /// # Ok(token)
    /// # }
    /// ```
    pub async fn create(&self, client: &Client) -> crate::Result<String> {
        client.create_meeting_token(self).await
    }

    #[cfg(feature = "self-signed-tokens")]
    #[cfg_attr(docsrs, doc(cfg(feature = "self-signed-tokens")))]
    /// Self-sign a `Daily` meeting token corresponding to this configuration,
    /// skipping the round-trip required by [create](#method.create).
    ///
    /// # Optional
    ///
    /// This requires the optional `self-signed-tokens` feature enabled.
    ///
    /// # Examples
    ///
    /// Create a token to join a room with owner privileges.
    ///
    /// ```no_run
    /// # use dailyco::meeting_token::MeetingTokenBuilder;
    /// # fn run() -> String {
    /// let token = MeetingTokenBuilder::new()
    ///   .room_name("room-user-should-own")
    ///   .is_owner(true)
    ///   .self_sign("domain_id", "test-api-key");
    /// # token
    /// # }
    /// ```
    pub fn self_sign(&self, domain_id: &str, secret_key: &str) -> String {
        crate::self_sign_token::self_sign_token(self, domain_id, secret_key)
    }
}

/// A `MeetingToken` describes the configuration of a meeting token used to join a
/// `Daily` private meeting room.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MeetingToken {
    /// The room for which this token is valid. If `room_name` isn't set, the token is
    /// valid for all rooms in your domain.
    pub room_name: Option<String>,
    /// Kick this user out of the meeting at the time this meeting token expires.
    #[serde(default)]
    pub eject_at_token_exp: bool,
    /// Kick this user out of the meeting this many seconds after they join the meeting.
    pub eject_after_elapsed: Option<i64>,
    /// UTC timestamp before which the token cannot be used.
    pub nbf: Option<i64>,
    /// UTC timestamp for expiration of the token.
    pub exp: Option<i64>,
    /// The user has meeting owner privileges.
    #[serde(default)]
    pub is_owner: bool,
    /// The user's name in this meeting.
    pub user_name: Option<String>,
    /// The user's id for this meeting session.
    pub user_id: Option<String>,
    /// The user is allowed to screenshare.
    #[serde(default = "default_as_true")]
    pub enable_screenshare: bool,
    /// When a participant first joins a meeting, keep their camera off.
    #[serde(default)]
    pub start_video_off: bool,
    /// When a participant first joins a meeting, keep their microphone muted.
    #[serde(default)]
    pub start_audio_off: bool,
    /// Allowed recording type
    pub enable_recording: Option<RecordingType>,
    /// Determines whether participant enters a waiting room with a camera, mic, and
    /// browser check before joining a call.
    pub enable_prejoin_ui: Option<bool>,
    /// Reduces the volume of log messages. This feature should be enabled when there
    /// are more than 300 participants in a meeting to help improve performance.
    #[serde(default)]
    pub enable_terse_logging: bool,
    /// Start cloud recording when the user joins the room. This can be used to always record and
    /// archive meetings, for example in a customer support context.
    #[serde(default)]
    pub start_cloud_recording: bool,
    /// When a user leaves a meeting using the button in the in-call menu bar,
    /// the browser tab closes.
    #[serde(default)]
    pub close_tab_on_exit: bool,
    /// When a user leaves a meeting using the button in the in-call menu bar,
    /// the browser loads this URL.
    pub redirect_on_meeting_exit: Option<String>,
    /// The default language of the Daily prebuilt video call UI, for this room.
    pub lang: Option<DailyLang>,
}

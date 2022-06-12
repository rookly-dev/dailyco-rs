//! Definition and creation of `Daily` meeting tokens.
use crate::configuration::{DailyLang, RecordingType};
use crate::Client;
use serde::Serialize;

/// A `MeetingTokenBuilder` can be used to create a `Daily` meeting token for gaining
/// access to a private room.
#[derive(Debug, Clone, Serialize, Default)]
pub struct MeetingTokenBuilder<'a> {
    room_name: Option<&'a str>,
    eject_at_token_exp: Option<bool>,
    eject_after_elapsed: Option<i64>,
    nbf: Option<i64>,
    exp: Option<i64>,
    is_owner: Option<bool>,
    user_name: Option<&'a str>,
    user_id: Option<&'a str>,
    enable_screenshare: Option<bool>,
    start_video_off: Option<bool>,
    start_audio_off: Option<bool>,
    enable_recording: Option<RecordingType>,
    enable_prejoin_ui: Option<bool>,
    enable_terse_logging: Option<bool>,
    start_cloud_recording: Option<bool>,
    close_tab_on_exit: Option<bool>,
    redirect_on_meeting_exit: Option<&'a str>,
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
    /// # use dailyco::Client;
    /// # use dailyco::meeting_token::MeetingTokenBuilder;
    /// # async fn run() {
    /// let client = Client::new("test-api-key").unwrap();
    /// MeetingTokenBuilder::new()
    ///   .room_name("room-user-should-own")
    ///   .is_owner(true)
    ///   .create(&client)
    ///   .await
    ///   .expect("Should create token");
    /// # }
    /// ```
    pub async fn create(&self, client: &Client) -> crate::Result<String> {
        client.create_meeting_token(self).await
    }
}

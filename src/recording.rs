//! Functionality related to `Daily` recordings.
use crate::client::parse_dailyco_response;
use crate::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The recording object, representing a single saved recording, as described
/// in <https://docs.daily.co/reference/rest-api/recordings/config>
#[derive(Debug, Clone, Deserialize)]
pub struct RecordingObject {
    /// A unique, opaque ID for this object. You can use this ID in API calls,
    /// and in paginated list operations.
    pub id: Uuid,
    /// The name of the room.
    pub room_name: String,
    /// When the recording started. This is a unix timestamp (seconds since the epoch).
    pub start_ts: i64,
    /// Options: "finished","in-progress","canceled"
    pub status: RecordingStatus,
    /// The maximum number of participants that were ever in this room together during the meeting session that was recorded.
    pub max_participants: u32,
    /// How many seconds long the recording is, approximately. This property is not
    /// returned for recordings that are in-progress.
    pub duration: Option<u32>,
    /// The S3 Key associated with this recording.
    pub s3key: String,
    #[serde(rename = "mtgSessionId")]
    /// The meeting session ID for this recording.
    pub meeting_session_id: Uuid,
}

/// The status of a recording.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecordingStatus {
    /// Finished
    Finished,
    /// In-progress
    InProgress,
    /// Canceled
    Canceled,
}

/// A builder for the `/recordings/:id/access-link` request, which creates and returns
/// a recording access link;
///
/// <https://docs.daily.co/reference/rest-api/recordings/get-recording-link>
#[derive(Copy, Clone, Debug, serde::Serialize, Default)]
pub struct GetRecordingAccessLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for_secs: Option<u64>,
}

/// Access link for a recording, as described in <https://docs.daily.co/reference/rest-api/recordings/get-recording-link>
#[derive(Debug, Clone, Deserialize)]
pub struct RecordingAccessLink {
    /// The download_link is a cryptographically signed, time-limited,
    /// direct link to a .mp4 file stored on Amazon S3
    pub download_link: String,
    /// The `expires` property is a unix timestamp after which the `download_link`
    /// will no longer work.
    pub expires: i64,
}

impl GetRecordingAccessLink {
    /// Constructs a new `GetRecordingAccessLink`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify the number of seconds into the future this link will remain valid for.
    pub fn valid_for_secs(&mut self, secs: u64) -> &mut Self {
        self.valid_for_secs = Some(secs);
        self
    }

    /// Send the request to create and get an access link for a recording.
    pub async fn send(&self, client: &Client, id: Uuid) -> crate::Result<RecordingAccessLink> {
        let url = format!("{}/recordings/{id}/access-link", client.base_url);
        let resp = client.client.get(url).query(self).send().await?;
        parse_dailyco_response(resp).await
    }
}

/// The return value for the `/recordings` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct ListedRecordings {
    /// The `total_count` field is the total number of recordings stored.
    pub total_count: u32,
    /// The `data` field is a list of recording objects.
    pub data: Vec<RecordingObject>,
}

/// A builder for the `/recordings` request to return a list of cloud recordings.
///
/// Recordings are returned sorted by created_at time in reverse chronological order.
/// This endpoint is detailed in <https://docs.daily.co/reference/rest-api/recordings/list-recordings>
#[derive(Debug, Copy, Clone, Serialize, Default)]
pub struct ListRecordings<'a> {
    limit: Option<u32>,
    ending_before: Option<Uuid>,
    starting_after: Option<Uuid>,
    room_name: Option<&'a str>,
}

impl<'a> ListRecordings<'a> {
    /// Constructs a new `ListRecordings`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The `limit` argument sets the size of the page (how many objects each page contains),
    /// and defaults to a value of 100.
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// The `ending_before` argument is used to fetch previous pages of results.
    pub fn ending_before(&mut self, ending_before: Uuid) -> &mut Self {
        self.ending_before = Some(ending_before);
        self
    }

    /// The `starting_after` argument sets the starting point of the page and is used to
    /// fetch "subsequent" pages of results.
    pub fn starting_after(&mut self, starting_after: Uuid) -> &mut Self {
        self.starting_after = Some(starting_after);
        self
    }

    /// Limit the results to a specific room.
    pub fn room_name(&mut self, room_name: &'a str) -> &mut Self {
        self.room_name = Some(room_name);
        self
    }

    /// Return a list of recordings.
    pub async fn send(&self, client: &Client) -> crate::Result<ListedRecordings> {
        let url = format!("{}/recordings", client.base_url);
        let resp = client.client.get(url).query(self).send().await?;
        parse_dailyco_response(resp).await
    }
}

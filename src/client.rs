use std::fmt;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Response, Url};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use uuid::Uuid;

use crate::meeting_token::MeetingToken;
use crate::recording::RecordingObject;
use crate::room::Room;
use crate::{Error, Result};

const BASE_URL: &str = "https://api.daily.co/v1/";

/// A `Client` to make `Daily` API requests with.
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: Url,
}

impl Client {
    /// Creates a [Client](crate::Client) from an API key.
    ///
    /// # Errors
    ///
    /// If the given API key does not contain only ASCII characters, an
    /// error variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dailyco::{Client, Result};
    /// # fn main_fn() -> Result<Client> {
    /// let client = Client::new("test-api-key")?;
    /// Ok(client)
    /// # }
    /// ```
    pub fn new<T: fmt::Display>(key: T) -> Result<Self> {
        // We should be guaranteed this parsing will not fail
        let base_url = Url::parse(BASE_URL).unwrap();
        Self::with_endpoint(key, base_url)
    }

    /// Creates a [Client](crate::Client) with a custom endpoint. This is primarily
    /// intended for testing purposes - for example pointing API requests to a [wiremock server](https://github.com/LukeMathWalker/wiremock-rs).
    ///
    /// # Examples
    ///
    /// ```
    /// # use dailyco::{Client, Result};
    /// # fn main_fn() -> Result<Client> {
    /// let mock_server_addr = "http://localhost:8080";
    /// let client = Client::with_endpoint(
    ///     "test-api-key",
    ///     reqwest::Url::parse(mock_server_addr).unwrap(),
    /// )?;
    /// Ok(client)
    /// # }
    /// ```
    pub fn with_endpoint<T: fmt::Display>(key: T, endpoint: Url) -> Result<Self> {
        let mut header_val = HeaderValue::try_from(format!("Bearer {}", key))
            .map_err(|_| Error::BadAPIKey("API key must include only ASCII characters"))?;
        header_val.set_sensitive(true);

        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, header_val);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self {
            client,
            base_url: endpoint,
        })
    }
}

impl Client {
    /// Retrieve the `Daily` room corresponding to this name.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dailyco::Client;
    /// # use dailyco::Result;
    /// # use dailyco::room::Room;
    /// #
    /// # async fn run() -> Result<Room> {
    /// let client = Client::new("test-api-key")?;
    /// let room = client.get_room("room-we-just-made").await?;
    /// # Ok(room)
    /// # }
    /// ```
    pub async fn get_room(&self, room_name: &str) -> Result<Room> {
        let url = self.get_room_url_with_name(room_name);
        let resp = self.client.get(url).send().await?;

        parse_dailyco_response(resp).await
    }

    /// Validate and retrieve configuration information for the provided meeting token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dailyco::{Client, Result};
    /// # use dailyco::room::Room;
    /// # use dailyco::meeting_token::{CreateMeetingToken};
    /// #
    /// # async fn run() -> Result<()> {
    /// let client = Client::new("test-api-key")?;
    /// let token = CreateMeetingToken::new()
    ///     .room_name("room-which-exists")
    ///     .send(&client)
    ///     .await?;
    /// let validated = client.get_meeting_token(&token).await?;
    /// assert_eq!(validated.room_name, Some("room-which-exists".to_string()));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_meeting_token(&self, token: &str) -> Result<MeetingToken> {
        let url = self
            .base_url
            .join("meeting-tokens/")
            .unwrap()
            .join(token)
            .unwrap();
        let resp = self.client.get(url).send().await?;

        parse_dailyco_response(resp).await
    }

    /// Retrieve all `Daily` rooms for the account.
    ///
    /// Pagination is currently unimplemented, so queries returning
    /// more than `100` rooms will return a `crate::Error::RequiresPagination`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dailyco::{Client, Result};
    /// # use dailyco::room::Room;
    /// # async fn run() -> Result<Vec<Room>> {
    /// let client = Client::new("test-api-key")?;
    /// let rooms = client.get_rooms().await?;
    /// # Ok(rooms)
    /// # }
    /// ```
    pub async fn get_rooms(&self) -> Result<Vec<Room>> {
        #[derive(Debug, Deserialize)]
        struct GetRoomsResponse {
            total_count: usize,
            data: Vec<Room>,
        }

        let url = self.base_url.join("rooms/").unwrap();
        let resp = self.client.get(url).send().await?;
        let data: GetRoomsResponse = parse_dailyco_response(resp).await?;
        if data.total_count >= 100 {
            Err(Error::RequiresPagination)
        } else {
            Ok(data.data)
        }
    }

    /// Get information about a specific recording.
    ///
    /// <https://docs.daily.co/reference/rest-api/recordings/get-recording-information>
    pub async fn get_recording(&self, id: Uuid) -> Result<RecordingObject> {
        let url = format!("{}/recordings/{id}", self.base_url);
        let resp = self.client.get(url).send().await?;
        let data: RecordingObject = parse_dailyco_response(resp).await?;
        Ok(data)
    }

    /// Delete a specific recording
    ///
    /// <https://docs.daily.co/reference/rest-api/recordings/delete-recording>
    pub async fn delete_recording(&self, id: Uuid) -> Result<()> {
        let url = format!("{}/recordings/{id}", self.base_url);
        let resp = self.client.delete(url).send().await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::from_failed_daily_request(resp).await)
        }
    }

    /// Delete the `Daily` room with this name.
    ///
    /// Will result in an error if the room does not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dailyco::{Client, Result};
    /// # async fn run() -> Result<()> {
    /// let client = Client::new("test-api-key")?;
    /// client.delete_room("room-that-exists").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_room(&self, room_name: &str) -> Result<()> {
        let url = self.get_room_url_with_name(room_name);
        let resp = self.client.delete(url).send().await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::from_failed_daily_request(resp).await)
        }
    }

    fn get_room_url_with_name(&self, room_name: &str) -> Url {
        // Neither of these unwraps should be able to fail
        self.base_url
            .join("rooms/")
            .unwrap()
            .join(room_name)
            .unwrap()
    }
}

pub async fn parse_dailyco_response<T: DeserializeOwned>(resp: Response) -> Result<T> {
    if resp.status().is_success() {
        Ok(resp.json().await?)
    } else {
        Err(Error::from_failed_daily_request(resp).await)
    }
}

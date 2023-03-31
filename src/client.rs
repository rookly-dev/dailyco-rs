use std::fmt;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Response, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::meeting_token::{MeetingToken, MeetingTokenBuilder};
use crate::room::{Room, RoomBuilder};
use crate::{Error, Result};

const BASE_URL: &str = "https://api.daily.co/v1/";

/// A `Client` to make `Daily` API requests with. This is just a wrapper around a
/// `reqwest::Client`, so it can be treated the same way with respect to handling
/// concurrent use.
#[derive(Debug, Clone)]
pub struct Client {
    reqwest_client: reqwest::Client,
    base_url: Url,
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
    /// let client = Client::with_endpoint("test-api-key", reqwest::Url::parse(mock_server_addr).unwrap())?;
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
            reqwest_client: client,
            base_url: endpoint,
        })
    }
}

impl Client {
    /// Create a room with the given configuration. Returns the daily.co room object
    /// if successful.
    pub(crate) async fn create_room(&self, builder: &RoomBuilder<'_>) -> Result<Room> {
        // This should not be able to fail
        let room_url = self.base_url.join("rooms/").unwrap();
        let resp = self
            .reqwest_client
            .post(room_url)
            .json(builder)
            .send()
            .await?;

        parse_dailyco_response(resp).await
    }

    /// Create a meeting token with the given configuration.
    pub(crate) async fn create_meeting_token(
        &self,
        config: &MeetingTokenBuilder<'_>,
    ) -> Result<String> {
        #[derive(Deserialize)]
        /// Response from Daily for successful meeting token creation
        struct MeetingTokenResponse {
            /// The token created
            token: String,
        }

        #[derive(Serialize)]
        struct MeetingTokenBody<'a> {
            properties: &'a MeetingTokenBuilder<'a>,
        }

        // This should not be able to fail
        let token_url = self.base_url.join("meeting-tokens/").unwrap();
        let body = MeetingTokenBody { properties: config };
        let resp = self
            .reqwest_client
            .post(token_url)
            .json(&body)
            .send()
            .await?;

        parse_dailyco_response(resp)
            .await
            .map(|token_resp: MeetingTokenResponse| token_resp.token)
    }

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
        let resp = self.reqwest_client.get(url).send().await?;

        parse_dailyco_response(resp).await
    }

    /// Validate and retrieve configuration information for the provided meeting token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dailyco::{Client, Result};
    /// # use dailyco::room::Room;
    /// # use dailyco::meeting_token::MeetingTokenBuilder;
    /// #
    /// # async fn run() -> Result<()> {
    /// let client = Client::new("test-api-key")?;
    /// let token = MeetingTokenBuilder::new()
    ///   .room_name("room-which-exists")
    ///   .create(&client)
    ///   .await?;
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
        let resp = self.reqwest_client.get(url).send().await?;

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
        let resp = self.reqwest_client.get(url).send().await?;
        let data: GetRoomsResponse = parse_dailyco_response(resp).await?;
        if data.total_count >= 100 {
            Err(Error::RequiresPagination)
        } else {
            Ok(data.data)
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
        let resp = self.reqwest_client.delete(url).send().await?;

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

async fn parse_dailyco_response<T: DeserializeOwned>(resp: Response) -> Result<T> {
    if resp.status().is_success() {
        Ok(resp.json().await?)
    } else {
        Err(Error::from_failed_daily_request(resp).await)
    }
}

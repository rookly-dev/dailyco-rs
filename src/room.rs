//! Definition and creation of `Daily` rooms.
use serde::{Deserialize, Serialize};

use crate::client::parse_dailyco_response;
use crate::room_properties::{RoomProperties, RoomPropertiesBuilder};
use crate::Client;

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

/// A `CreateRoom` can be used to create a `Daily` room with custom configuration.
#[derive(Debug, Copy, Clone, Serialize, Default)]
pub struct CreateRoom<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy: Option<RoomPrivacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RoomPropertiesBuilder<'a>>,
}

impl<'a> CreateRoom<'a> {
    /// Constructs a new `CreateRoom`.
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

    /// Set the properties for this room
    pub fn properties(&mut self, properties: RoomPropertiesBuilder<'a>) -> &mut Self {
        self.properties = Some(properties);
        self
    }

    /// Make the request to create the `Daily` room.
    ///
    /// # Examples
    ///
    /// Create a private room with audio and video off by default:
    ///
    /// ```no_run
    /// # use dailyco::{Client, Result, RoomPropertiesBuilder};
    /// # use dailyco::room::{Room, CreateRoom, RoomPrivacy};
    /// # async fn run() -> Result<Room> {
    /// let client = Client::new("test-api-key")?;
    /// let created_room = CreateRoom::new()
    ///     .privacy(RoomPrivacy::Private)
    ///     .properties(
    ///         RoomPropertiesBuilder::new()
    ///             .start_audio_off(true)
    ///             .start_video_off(true),
    ///     )
    ///     .send(&client)
    ///     .await?;
    /// # Ok(created_room)
    /// # }
    /// ```
    pub async fn send(&self, client: &Client) -> crate::Result<Room> {
        // This should not be able to fail
        let room_url = client.base_url.join("rooms/").unwrap();
        let resp = client.client.post(room_url).json(self).send().await?;
        parse_dailyco_response(resp).await
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

/// An `UpdateRoom` can be used to update an existing `Daily` room.
#[derive(Debug, Copy, Clone, Serialize, Default)]
pub struct UpdateRoom<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy: Option<RoomPrivacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RoomPropertiesBuilder<'a>>,
}

impl<'a> UpdateRoom<'a> {
    /// Constructs a new `UpdateRoom`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the visibility for the room.
    pub fn privacy(&mut self, privacy: RoomPrivacy) -> &mut Self {
        self.privacy = Some(privacy);
        self
    }

    /// Update the properties for this room
    pub fn properties(&mut self, properties: RoomPropertiesBuilder<'a>) -> &mut Self {
        self.properties = Some(properties);
        self
    }

    /// Make the request to update a `Daily` room.
    ///
    /// # Examples
    ///
    /// Create a private room with audio and video off by default:
    ///
    /// ```no_run
    /// # use dailyco::{Client, Result, RoomPropertiesBuilder};
    /// # use dailyco::room::{Room, UpdateRoom, RoomPrivacy};
    /// # async fn run() -> Result<Room> {
    /// let client = Client::new("test-api-key")?;
    /// let existing_room_name = "existing_room";
    /// let updated_room = UpdateRoom::new()
    ///     .privacy(RoomPrivacy::Private)
    ///     .properties(
    ///         RoomPropertiesBuilder::new()
    ///             .start_audio_off(true)
    ///             .start_video_off(true),
    ///     )
    ///     .send(existing_room_name, &client)
    ///     .await?;
    /// # Ok(updated_room)
    /// # }
    /// ```
    pub async fn send(&self, room_name: &str, client: &Client) -> crate::Result<Room> {
        // This should not be able to fail
        let room_url = client.base_url.join(&format!("rooms/{room_name}")).unwrap();
        let resp = client.client.post(room_url).json(self).send().await?;
        parse_dailyco_response(resp).await
    }
}

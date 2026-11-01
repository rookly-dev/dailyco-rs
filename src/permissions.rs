//! Participant permissions granted by a `Daily` meeting token.
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Initial permissions for a participant who isn't a meeting owner. Unset fields fall
/// back to the room's, then the domain's, permissions.
///
/// Serializes to the format for creating a token through the REST API, and deserializes
/// from the abbreviated format `Daily` returns when retrieving a token.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct MeetingTokenPermissions {
    /// Whether the participant appears as "present" in the call, i.e. whether they
    /// appear in `participants()`.
    #[serde(
        rename(serialize = "hasPresence", deserialize = "hp"),
        skip_serializing_if = "Option::is_none"
    )]
    pub has_presence: Option<bool>,
    /// Which types of media the participant is permitted to send.
    #[serde(
        rename(serialize = "canSend", deserialize = "cs"),
        skip_serializing_if = "Option::is_none"
    )]
    pub can_send: Option<CanSend>,
    /// Which admin tasks the participant is permitted to do.
    #[serde(
        rename(serialize = "canAdmin", deserialize = "ca"),
        skip_serializing_if = "Option::is_none"
    )]
    pub can_admin: Option<CanAdmin>,
}

/// Which types of media a participant is permitted to send.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CanSend {
    /// All types of media.
    All,
    /// No media.
    Nothing,
    /// Only these types of media. An empty set is sent as [`CanSend::Nothing`].
    Only(SendKinds),
}

/// Types of media a participant can send.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct SendKinds {
    /// Camera video.
    pub video: bool,
    /// Microphone audio.
    pub audio: bool,
    /// Screenshare video.
    pub screen_video: bool,
    /// Screenshare audio.
    pub screen_audio: bool,
}

/// Which admin tasks a participant is permitted to do.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CanAdmin {
    /// All admin tasks.
    All,
    /// No admin tasks.
    Nothing,
    /// Only these admin tasks. An empty set is sent as [`CanAdmin::Nothing`].
    Only(AdminKinds),
}

/// Admin tasks a participant can do.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct AdminKinds {
    /// Manage other participants.
    pub participants: bool,
    /// Manage live streaming.
    pub streaming: bool,
    /// Manage transcription.
    pub transcription: bool,
}

/// A set of kinds within a [`CanSend`] or [`CanAdmin`] permission.
pub(crate) trait Kinds: Copy + Default + PartialEq {
    /// Each kind's flag, REST API name and abbreviated name.
    fn entries(&mut self) -> Vec<(&mut bool, &'static str, &'static str)>;

    /// The included kinds' REST API names, or abbreviated names.
    fn names(mut self, abbreviated: bool) -> Vec<&'static str> {
        self.entries()
            .into_iter()
            .filter(|(included, ..)| **included)
            .map(|(_, name, abbreviation)| if abbreviated { abbreviation } else { name })
            .collect()
    }
}

impl Kinds for SendKinds {
    fn entries(&mut self) -> Vec<(&mut bool, &'static str, &'static str)> {
        vec![
            (&mut self.video, "video", "v"),
            (&mut self.audio, "audio", "a"),
            (&mut self.screen_video, "screenVideo", "sv"),
            (&mut self.screen_audio, "screenAudio", "sa"),
        ]
    }
}

impl Kinds for AdminKinds {
    fn entries(&mut self) -> Vec<(&mut bool, &'static str, &'static str)> {
        vec![
            (&mut self.participants, "participants", "p"),
            (&mut self.streaming, "streaming", "s"),
            (&mut self.transcription, "transcription", "t"),
        ]
    }
}

/// How `Daily` encodes a [`CanSend`] or [`CanAdmin`] permission: a bool for all or
/// nothing, otherwise a set of kinds.
///
/// `Serialize` and `Deserialize` use the abbreviated format of self-signed tokens, which
/// is also what `Daily` returns when retrieving any token: a bool, or a comma-separated
/// string of abbreviated kinds such as `"v,a"`.
#[derive(Copy, Clone)]
pub(crate) enum Encoded<K> {
    Bool(bool),
    Only(K),
}

impl<K: Kinds> Encoded<K> {
    fn only(kinds: K) -> Self {
        if kinds == K::default() {
            Encoded::Bool(false)
        } else {
            Encoded::Only(kinds)
        }
    }

    /// Serialize in the REST API format: a bool, or an array of kind names.
    fn serialize_rest<S: Serializer>(self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Encoded::Bool(all) => serializer.serialize_bool(all),
            Encoded::Only(kinds) => serializer.collect_seq(kinds.names(false)),
        }
    }
}

impl<K: Kinds> Serialize for Encoded<K> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            Encoded::Bool(all) => serializer.serialize_bool(all),
            Encoded::Only(kinds) => serializer.serialize_str(&kinds.names(true).join(",")),
        }
    }
}

impl<'de, K: Kinds> Deserialize<'de> for Encoded<K> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Raw {
            Bool(bool),
            Abbreviations(String),
        }

        let abbreviations = match Raw::deserialize(deserializer)? {
            Raw::Bool(all) => return Ok(Encoded::Bool(all)),
            Raw::Abbreviations(abbreviations) => abbreviations,
        };
        let mut kinds = K::default();
        for abbreviation in abbreviations.split(',') {
            let (included, ..) = kinds
                .entries()
                .into_iter()
                .find(|(_, _, a)| *a == abbreviation)
                .ok_or_else(|| D::Error::custom(format!("unknown permission `{abbreviation}`")))?;
            *included = true;
        }
        Ok(Encoded::Only(kinds))
    }
}

impl From<CanSend> for Encoded<SendKinds> {
    fn from(can_send: CanSend) -> Self {
        match can_send {
            CanSend::All => Encoded::Bool(true),
            CanSend::Nothing => Encoded::Bool(false),
            CanSend::Only(kinds) => Encoded::only(kinds),
        }
    }
}

impl From<Encoded<SendKinds>> for CanSend {
    fn from(encoded: Encoded<SendKinds>) -> Self {
        match encoded {
            Encoded::Bool(true) => CanSend::All,
            Encoded::Bool(false) => CanSend::Nothing,
            Encoded::Only(kinds) => CanSend::Only(kinds),
        }
    }
}

impl From<CanAdmin> for Encoded<AdminKinds> {
    fn from(can_admin: CanAdmin) -> Self {
        match can_admin {
            CanAdmin::All => Encoded::Bool(true),
            CanAdmin::Nothing => Encoded::Bool(false),
            CanAdmin::Only(kinds) => Encoded::only(kinds),
        }
    }
}

impl From<Encoded<AdminKinds>> for CanAdmin {
    fn from(encoded: Encoded<AdminKinds>) -> Self {
        match encoded {
            Encoded::Bool(true) => CanAdmin::All,
            Encoded::Bool(false) => CanAdmin::Nothing,
            Encoded::Only(kinds) => CanAdmin::Only(kinds),
        }
    }
}

/// Serializes in the REST API format: a bool, or an array of media types.
impl Serialize for CanSend {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Encoded::from(*self).serialize_rest(serializer)
    }
}

/// Deserializes from the abbreviated format `Daily` returns when retrieving a token.
impl<'de> Deserialize<'de> for CanSend {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Encoded::<SendKinds>::deserialize(deserializer).map(Self::from)
    }
}

/// Serializes in the REST API format: a bool, or an array of admin tasks.
impl Serialize for CanAdmin {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Encoded::from(*self).serialize_rest(serializer)
    }
}

/// Deserializes from the abbreviated format `Daily` returns when retrieving a token.
impl<'de> Deserialize<'de> for CanAdmin {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Encoded::<AdminKinds>::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::meeting_token::{CreateMeetingToken, MeetingToken};

    const VIDEO_AND_AUDIO: SendKinds = SendKinds {
        video: true,
        audio: true,
        screen_video: false,
        screen_audio: false,
    };

    fn rest_json(permissions: MeetingTokenPermissions) -> serde_json::Value {
        serde_json::to_value(CreateMeetingToken::new().permissions(permissions)).unwrap()
    }

    #[test]
    fn serializes_rest_format() {
        let hidden = MeetingTokenPermissions {
            has_presence: Some(false),
            can_send: Some(CanSend::Nothing),
            ..Default::default()
        };
        assert_eq!(
            rest_json(hidden),
            json!({ "permissions": { "hasPresence": false, "canSend": false } })
        );

        let all = MeetingTokenPermissions {
            can_send: Some(CanSend::All),
            can_admin: Some(CanAdmin::All),
            ..Default::default()
        };
        assert_eq!(
            rest_json(all),
            json!({ "permissions": { "canSend": true, "canAdmin": true } })
        );

        let subsets = MeetingTokenPermissions {
            has_presence: Some(true),
            can_send: Some(CanSend::Only(VIDEO_AND_AUDIO)),
            can_admin: Some(CanAdmin::Only(AdminKinds {
                participants: true,
                transcription: true,
                ..Default::default()
            })),
        };
        assert_eq!(
            rest_json(subsets),
            json!({ "permissions": {
                "hasPresence": true,
                "canSend": ["video", "audio"],
                "canAdmin": ["participants", "transcription"],
            } })
        );

        let empty_subset = MeetingTokenPermissions {
            can_send: Some(CanSend::Only(SendKinds::default())),
            ..Default::default()
        };
        assert_eq!(
            rest_json(empty_subset),
            json!({ "permissions": { "canSend": false } })
        );

        assert_eq!(
            rest_json(MeetingTokenPermissions::default()),
            json!({ "permissions": {} })
        );
        assert_eq!(
            serde_json::to_value(CreateMeetingToken::new()).unwrap(),
            json!({})
        );
    }

    #[test]
    fn deserializes_retrieved_format() {
        let token: MeetingToken = serde_json::from_value(json!({
            "permissions": { "hp": false, "cs": "a,v", "ca": "t" }
        }))
        .unwrap();
        assert_eq!(
            token.permissions,
            Some(MeetingTokenPermissions {
                has_presence: Some(false),
                can_send: Some(CanSend::Only(VIDEO_AND_AUDIO)),
                can_admin: Some(CanAdmin::Only(AdminKinds {
                    transcription: true,
                    ..Default::default()
                })),
            })
        );

        let token: MeetingToken =
            serde_json::from_value(json!({ "permissions": { "cs": true, "ca": false } })).unwrap();
        assert_eq!(
            token.permissions,
            Some(MeetingTokenPermissions {
                has_presence: None,
                can_send: Some(CanSend::All),
                can_admin: Some(CanAdmin::Nothing),
            })
        );

        let token: MeetingToken = serde_json::from_value(json!({})).unwrap();
        assert_eq!(token.permissions, None);

        let unknown = serde_json::from_value::<MeetingToken>(json!({
            "permissions": { "cs": "v,cv" }
        }));
        assert!(unknown.is_err());
    }
}

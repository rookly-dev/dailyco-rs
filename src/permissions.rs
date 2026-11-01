//! Participant permissions granted by a `Daily` meeting token.
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Initial permissions for a participant who isn't a meeting owner. Unset fields fall
/// back to the room's, then the domain's, permissions.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct MeetingTokenPermissions {
    /// Whether the participant appears as "present" in the call, i.e. whether they
    /// appear in `participants()`.
    pub has_presence: Option<bool>,
    /// Which types of media the participant is permitted to send.
    pub can_send: Option<CanSend>,
    /// Which admin tasks the participant is permitted to do.
    pub can_admin: Option<CanAdmin>,
}

/// A permission that allows everything, nothing, or only some kinds of things.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Allowed<K> {
    /// All kinds.
    All,
    /// No kinds.
    Nothing,
    /// Only these kinds. An empty set is the same as [`Allowed::Nothing`].
    Only(K),
}

/// Which types of media a participant is permitted to send.
pub type CanSend = Allowed<SendKinds>;

/// Which admin tasks a participant is permitted to do.
pub type CanAdmin = Allowed<AdminKinds>;

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
    /// Custom video tracks.
    pub custom_video: bool,
    /// Custom audio tracks.
    pub custom_audio: bool,
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

/// The kinds within an [`Allowed`] permission.
trait Kinds: Copy + Default + PartialEq {
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
            (&mut self.custom_video, "customVideo", "cv"),
            (&mut self.custom_audio, "customAudio", "ca"),
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

impl MeetingTokenPermissions {
    /// Replace empty `Only` sets with `Nothing`, the form `Daily` returns them in.
    pub(crate) fn normalized(self) -> Self {
        fn normalize<K: Kinds>(allowed: Allowed<K>) -> Allowed<K> {
            match allowed {
                Allowed::Only(kinds) if kinds == K::default() => Allowed::Nothing,
                allowed => allowed,
            }
        }
        Self {
            has_presence: self.has_presence,
            can_send: self.can_send.map(normalize),
            can_admin: self.can_admin.map(normalize),
        }
    }
}

/// The REST API format for creating a token: a bool, or an array of names.
pub(crate) fn serialize_rest<S: Serializer>(
    permissions: &Option<MeetingTokenPermissions>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    #[derive(Serialize)]
    #[serde(untagged)]
    enum Value {
        Bool(bool),
        Names(Vec<&'static str>),
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Rest {
        #[serde(skip_serializing_if = "Option::is_none")]
        has_presence: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_send: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_admin: Option<Value>,
    }

    fn value<K: Kinds>(allowed: Allowed<K>) -> Value {
        match allowed {
            Allowed::All => Value::Bool(true),
            Allowed::Nothing => Value::Bool(false),
            Allowed::Only(kinds) => Value::Names(kinds.names(false)),
        }
    }

    permissions
        .map(|p| Rest {
            has_presence: p.has_presence,
            can_send: p.can_send.map(value),
            can_admin: p.can_admin.map(value),
        })
        .serialize(serializer)
}

/// The abbreviated format used by self-signed tokens, which `Daily` also returns when
/// retrieving any token: a bool, or a comma-separated string of abbreviations such as
/// `"v,a"`.
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Abbreviated {
    #[serde(rename = "hp", skip_serializing_if = "Option::is_none")]
    has_presence: Option<bool>,
    #[serde(rename = "cs", skip_serializing_if = "Option::is_none")]
    can_send: Option<AbbreviatedValue>,
    #[serde(rename = "ca", skip_serializing_if = "Option::is_none")]
    can_admin: Option<AbbreviatedValue>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum AbbreviatedValue {
    Bool(bool),
    Abbreviations(String),
}

impl From<MeetingTokenPermissions> for Abbreviated {
    fn from(p: MeetingTokenPermissions) -> Self {
        fn value<K: Kinds>(allowed: Allowed<K>) -> AbbreviatedValue {
            match allowed {
                Allowed::All => AbbreviatedValue::Bool(true),
                Allowed::Nothing => AbbreviatedValue::Bool(false),
                Allowed::Only(kinds) => {
                    AbbreviatedValue::Abbreviations(kinds.names(true).join(","))
                }
            }
        }
        Self {
            has_presence: p.has_presence,
            can_send: p.can_send.map(value),
            can_admin: p.can_admin.map(value),
        }
    }
}

impl TryFrom<Abbreviated> for MeetingTokenPermissions {
    type Error = String;

    fn try_from(a: Abbreviated) -> Result<Self, String> {
        fn allowed<K: Kinds>(value: AbbreviatedValue) -> Result<Allowed<K>, String> {
            let abbreviations = match value {
                AbbreviatedValue::Bool(true) => return Ok(Allowed::All),
                AbbreviatedValue::Bool(false) => return Ok(Allowed::Nothing),
                AbbreviatedValue::Abbreviations(abbreviations) => abbreviations,
            };
            let mut kinds = K::default();
            for abbreviation in abbreviations.split(',').map(str::trim) {
                if abbreviation.is_empty() {
                    continue;
                }
                let (included, ..) = kinds
                    .entries()
                    .into_iter()
                    .find(|(_, _, a)| *a == abbreviation)
                    .ok_or_else(|| format!("unknown permission `{abbreviation}`"))?;
                *included = true;
            }
            Ok(Allowed::Only(kinds))
        }
        let permissions = Self {
            has_presence: a.has_presence,
            can_send: a.can_send.map(allowed).transpose()?,
            can_admin: a.can_admin.map(allowed).transpose()?,
        };
        Ok(permissions.normalized())
    }
}

/// Deserialize permissions from the abbreviated format `Daily` returns when retrieving a
/// token.
pub(crate) fn deserialize_abbreviated<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<MeetingTokenPermissions>, D::Error> {
    Option::<Abbreviated>::deserialize(deserializer)?
        .map(MeetingTokenPermissions::try_from)
        .transpose()
        .map_err(D::Error::custom)
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
        custom_video: false,
        custom_audio: false,
    };

    fn rest_json(permissions: MeetingTokenPermissions) -> serde_json::Value {
        serde_json::to_value(CreateMeetingToken::new().permissions(permissions)).unwrap()
    }

    fn retrieved(json: serde_json::Value) -> Option<MeetingTokenPermissions> {
        serde_json::from_value::<MeetingToken>(json)
            .unwrap()
            .permissions
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
            can_send: Some(CanSend::Only(SendKinds {
                custom_video: true,
                ..VIDEO_AND_AUDIO
            })),
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
                "canSend": ["video", "audio", "customVideo"],
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
        assert_eq!(
            retrieved(json!({ "permissions": { "hp": false, "cs": "a,v", "ca": "t" } })),
            Some(MeetingTokenPermissions {
                has_presence: Some(false),
                can_send: Some(CanSend::Only(VIDEO_AND_AUDIO)),
                can_admin: Some(CanAdmin::Only(AdminKinds {
                    transcription: true,
                    ..Default::default()
                })),
            })
        );
        assert_eq!(
            retrieved(json!({ "permissions": { "cs": true, "ca": false } })),
            Some(MeetingTokenPermissions {
                has_presence: None,
                can_send: Some(CanSend::All),
                can_admin: Some(CanAdmin::Nothing),
            })
        );
        assert_eq!(
            retrieved(json!({ "permissions": { "cs": "v, cv,ca" } })),
            Some(MeetingTokenPermissions {
                can_send: Some(CanSend::Only(SendKinds {
                    video: true,
                    custom_video: true,
                    custom_audio: true,
                    ..Default::default()
                })),
                ..Default::default()
            })
        );
        assert_eq!(
            retrieved(json!({ "permissions": { "cs": "" } })),
            Some(MeetingTokenPermissions {
                can_send: Some(CanSend::Nothing),
                ..Default::default()
            })
        );
        assert_eq!(
            retrieved(json!({ "permissions": {} })),
            Some(MeetingTokenPermissions::default())
        );
        assert_eq!(retrieved(json!({})), None);

        let unknown =
            serde_json::from_value::<MeetingToken>(json!({ "permissions": { "cs": "v,zz" } }));
        assert!(unknown.is_err());
    }
}

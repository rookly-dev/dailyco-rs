use jsonwebtoken::{encode, EncodingKey, Header};

use crate::configuration::{DailyLang, RecordingType};
use crate::meeting_token::CreateMeetingToken;
use crate::permissions::Abbreviated;

#[derive(serde::Serialize)]
struct SelfSigningTokenPayload<'a> {
    // Domain id
    d: &'a str,
    #[serde(flatten)]
    rest: MeetingTokenRenamed<'a>,
}

pub fn self_sign_token(config: CreateMeetingToken, domain_id: &str, secret_key: &str) -> String {
    let payload = SelfSigningTokenPayload {
        d: domain_id,
        rest: config.into(),
    };
    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    // TOOD: Should safe, unless weird secret / domain inputs from user? Worth validation?
    .expect("Could not construct token");
    token
}

// TODO: very duplicative, but seems not like not a better way when
// essentially need to rename struct in 2 different ways. Definitely
// could be cleaner with a proc macro
#[derive(serde::Serialize, Clone)]
struct MeetingTokenRenamed<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "r")]
    room_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ejt")]
    eject_at_token_exp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "eje")]
    eject_after_elapsed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nbf: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "o")]
    is_owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "u")]
    user_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ud")]
    user_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ss")]
    enable_screenshare: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "vo")]
    start_video_off: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ao")]
    start_audio_off: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "er")]
    enable_recording: Option<RecordingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_prejoin_ui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_terse_logging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sr")]
    start_cloud_recording: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ctoe")]
    close_tab_on_exit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "rome")]
    redirect_on_meeting_exit: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "uil")]
    lang: Option<DailyLang>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "p")]
    permissions: Option<Abbreviated>,
}

// Same as comment above with respect to duplication here being not great
impl<'a> From<CreateMeetingToken<'a>> for MeetingTokenRenamed<'a> {
    fn from(b: CreateMeetingToken<'a>) -> Self {
        Self {
            room_name: b.room_name,
            eject_at_token_exp: b.eject_at_token_exp,
            eject_after_elapsed: b.eject_after_elapsed,
            nbf: b.nbf,
            exp: b.exp,
            is_owner: b.is_owner,
            user_name: b.user_name,
            user_id: b.user_id,
            enable_screenshare: b.enable_screenshare,
            start_video_off: b.start_video_off,
            start_audio_off: b.start_audio_off,
            enable_recording: b.enable_recording,
            enable_prejoin_ui: b.enable_prejoin_ui,
            enable_terse_logging: b.enable_terse_logging,
            start_cloud_recording: b.start_cloud_recording,
            close_tab_on_exit: b.close_tab_on_exit,
            redirect_on_meeting_exit: b.redirect_on_meeting_exit,
            lang: b.lang,
            permissions: b.permissions.map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use serde_json::json;

    use crate::meeting_token::{
        AdminKinds, CanAdmin, CanSend, CreateMeetingToken, MeetingTokenPermissions, SendKinds,
    };

    fn payload(builder: &CreateMeetingToken) -> serde_json::Value {
        let token = builder.self_sign("domain-id", "secret");
        let mut validation = Validation::default();
        validation.required_spec_claims.clear();
        decode(&token, &DecodingKey::from_secret(b"secret"), &validation)
            .unwrap()
            .claims
    }

    #[test]
    fn signs_abbreviated_permissions() {
        let hidden =
            *CreateMeetingToken::new()
                .start_video_off(true)
                .permissions(MeetingTokenPermissions {
                    has_presence: Some(false),
                    can_send: Some(CanSend::Nothing),
                    ..Default::default()
                });
        assert_eq!(
            payload(&hidden),
            json!({ "d": "domain-id", "vo": true, "p": { "hp": false, "cs": false } })
        );

        let subsets = *CreateMeetingToken::new().permissions(MeetingTokenPermissions {
            can_send: Some(CanSend::Only(SendKinds {
                video: true,
                audio: true,
                custom_audio: true,
                ..Default::default()
            })),
            can_admin: Some(CanAdmin::Only(AdminKinds {
                participants: true,
                streaming: true,
                ..Default::default()
            })),
            ..Default::default()
        });
        assert_eq!(
            payload(&subsets),
            json!({ "d": "domain-id", "p": { "cs": "v,a,ca", "ca": "p,s" } })
        );

        let all = *CreateMeetingToken::new().permissions(MeetingTokenPermissions {
            has_presence: Some(true),
            can_send: Some(CanSend::All),
            can_admin: Some(CanAdmin::Nothing),
        });
        assert_eq!(
            payload(&all),
            json!({ "d": "domain-id", "p": { "hp": true, "cs": true, "ca": false } })
        );

        let empty_subset = *CreateMeetingToken::new().permissions(MeetingTokenPermissions {
            can_send: Some(CanSend::Only(SendKinds::default())),
            ..Default::default()
        });
        assert_eq!(
            payload(&empty_subset),
            json!({ "d": "domain-id", "p": { "cs": false } })
        );

        let owner = *CreateMeetingToken::new().is_owner(true);
        assert_eq!(payload(&owner), json!({ "d": "domain-id", "o": true }));
    }
}

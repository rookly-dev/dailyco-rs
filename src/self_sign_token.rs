use crate::meeting_token::MeetingTokenBuilder;
use jsonwebtoken::{encode, EncodingKey, Header};

#[derive(serde::Serialize, Clone, Debug)]
struct SelfSigningTokenPayload<'a> {
    // Domain id
    d: &'a str,
    #[serde(flatten)]
    rest: &'a MeetingTokenBuilder<'a>,
}

pub fn self_sign_token(config: &MeetingTokenBuilder, domain_id: &str, secret_key: &str) -> String {
    let payload = SelfSigningTokenPayload {
        d: domain_id,
        rest: config,
    };
    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    // Should safe, unless weird secret / domain inputs from user? Worth validation?
    .expect("Could not construct token");
    token
}

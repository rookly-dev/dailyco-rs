use crate::helpers::get_daily_client;
use dailyco::meeting_token::{MeetingToken, MeetingTokenBuilder};
use dailyco::Client;

#[cfg(feature = "self-signed-tokens")]
pub fn get_domain_id_for_tests() -> String {
    std::env::var("DAILY_CO_DOMAIN_ID")
        .expect("Daily Domain Id required for testing self-signing meeting tokens")
}

macro_rules! meeting_token {
    ( $( $field:ident = $value:expr ),* ) => {{
        let mut builder = MeetingTokenBuilder::new();
        $(
            builder.$field($value);
        )*
        builder
    }};
}

fn get_meeting_token_test_cases(room_name: &str) -> Vec<MeetingTokenBuilder> {
    let mut builders = vec![
        meeting_token! { start_audio_off = true, user_name = "a_user", eject_after_elapsed = 50 },
        meeting_token! {
            start_audio_off = true,
            start_video_off = false,
            user_id = "hi",
            is_owner = true,
            enable_screenshare = false,
            exp = chrono::Utc::now().timestamp() + chrono::Duration::hours(3).num_seconds()
        },
        meeting_token! {
            nbf = chrono::Utc::now().timestamp() - chrono::Duration::hours(3).num_seconds(),
            enable_recording = dailyco::configuration::RecordingType::Cloud,
            eject_at_token_exp = true,
            start_cloud_recording = true,
            close_tab_on_exit = true,
            lang = dailyco::configuration::DailyLang::Es
        },
    ];
    builders.iter_mut().for_each(|b| {
        b.room_name(room_name);
    });
    builders
}

#[tokio::test]
async fn meeting_tokens_create_roundtrip() -> anyhow::Result<()> {
    let tokens = get_meeting_token_test_cases("a-room");
    let client = get_daily_client();
    for spec in tokens {
        let token_fetch = spec.create(&client).await?;
        assert_meeting_token_generation_roundtrip(&client, &token_fetch, spec.clone()).await?;
    }
    Ok(())
}

#[tokio::test]
#[cfg(feature = "self-signed-tokens")]
async fn meeting_tokens_self_sign_roundtrip() -> anyhow::Result<()> {
    let tokens = get_meeting_token_test_cases("a-room");
    let client = get_daily_client();
    let secret_key = crate::helpers::get_secret_key_for_tests();
    let domain_id = get_domain_id_for_tests();
    for spec in tokens {
        let token = spec.self_sign(&domain_id, &secret_key);
        assert_meeting_token_generation_roundtrip(&client, &token, spec.clone()).await?;
    }
    Ok(())
}

async fn assert_meeting_token_generation_roundtrip(
    client: &Client,
    token: &str,
    builder: MeetingTokenBuilder<'_>,
) -> anyhow::Result<()> {
    let returned = client.get_meeting_token(token).await?;
    assert_builder_matches_retrieved(builder, returned);
    Ok(())
}

fn assert_builder_matches_retrieved(expected: MeetingTokenBuilder, result: MeetingToken) {
    let expected: MeetingToken = expected.into();
    assert_eq!(expected, result);
}

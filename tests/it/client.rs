use dailyco::meeting_token::{MeetingToken, MeetingTokenBuilder};
use dailyco::room::{RoomBuilder, RoomPrivacy};
use dailyco::{Client, DailyCoErrorKind, Error};

fn get_secret_key_for_tests() -> String {
    std::env::var("DAILY_CO_API_KEY").expect("Requires Daily API KEY")
}

#[cfg(feature = "self-signed-tokens")]
fn get_domain_id_for_tests() -> String {
    std::env::var("DAILY_CO_DOMAIN_ID")
        .expect("Daily Domain Id required for testing self-signing meeting tokens")
}

fn get_daily_client() -> Client {
    dotenv::dotenv().unwrap();
    let key = get_secret_key_for_tests();
    Client::new(key).expect("Should make client")
}

async fn delete_room_if_exists(client: &Client, room_name: &str) {
    if let Err(err) = client.delete_room(room_name).await {
        match err {
            Error::APIError(api_err) => {
                assert!(matches!(api_err.error.unwrap(), DailyCoErrorKind::NotFound))
            }
            _ => panic!("Unexpected error"),
        }
    }
}

async fn ensure_room(client: &Client, room_name: &str) {
    delete_room_if_exists(client, room_name).await;
    RoomBuilder::new()
        .name(room_name)
        .create(client)
        .await
        .unwrap();
}

#[tokio::test]
async fn can_make_room() {
    let client = get_daily_client();
    let room_name = "test-room";
    delete_room_if_exists(&client, room_name).await;
    let room = RoomBuilder::new()
        .privacy(RoomPrivacy::Private)
        .name(room_name)
        .start_video_off(true)
        .start_audio_off(true)
        .create(&client)
        .await
        .unwrap();

    assert_eq!(room.name, room_name);
    assert_eq!(room.privacy, RoomPrivacy::Private);
    assert_eq!(room.config.start_audio_off, true);
    assert_eq!(room.config.start_video_off, true);
}

#[tokio::test]
async fn get_room_not_found() {
    let client = get_daily_client();
    let res = client.get_room("not-found").await.unwrap_err();
    match res {
        Error::APIError(err) => {
            assert!(matches!(err.error.unwrap(), DailyCoErrorKind::NotFound))
        }
        _ => panic!("Unexpected error"),
    }
}

#[tokio::test]
async fn delete_room() {
    let client = get_daily_client();
    let room_name = "room-to-delete";
    RoomBuilder::new()
        .name(room_name)
        .create(&client)
        .await
        .unwrap();
    assert!(client.delete_room(room_name).await.is_ok());
}

#[tokio::test]
async fn get_room() {
    let client = get_daily_client();
    let room_name = "room-to-get";
    delete_room_if_exists(&client, room_name).await;
    RoomBuilder::new()
        .name(room_name)
        .eject_at_room_exp(true)
        .max_participants(12)
        .create(&client)
        .await
        .unwrap();
    let room = client.get_room(room_name).await.unwrap();
    assert_eq!(room.name, room_name);
    assert!(room.api_created);
    assert_eq!(room.config.max_participants, Some(12));
    assert!(room.config.eject_at_room_exp);
}

#[tokio::test]
async fn get_rooms() {
    let client = get_daily_client();
    let created_room = RoomBuilder::new().create(&client).await.unwrap();
    let rooms = client.get_rooms().await.unwrap();
    rooms
        .iter()
        .find(|room| room.name == created_room.name)
        .unwrap();

    // Ugly solution to clean up randomly generated room name
    client.delete_room(&created_room.name).await.unwrap();
}

#[tokio::test]
async fn get_meeting_token() {
    let client = get_daily_client();
    let room_name = "meeting-token-room";
    ensure_room(&client, room_name).await;

    MeetingTokenBuilder::new()
        .room_name(room_name)
        .create(&client)
        .await
        .unwrap();
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

fn get_meeting_token_test_cases() -> Vec<MeetingTokenBuilder<'static>> {
    vec![
        meeting_token! { room_name = "hi", start_audio_off = true, user_name = "a_user", eject_after_elapsed = 50 },
        todo!(),
    ]
}

#[tokio::test]
async fn meeting_tokens_roundtrip() -> anyhow::Result<()> {
    let tokens = get_meeting_token_test_cases();
    let client = get_daily_client();
    for spec in tokens {
        assert_meeting_token_generation_roundtrip(&client, spec).await?;
    }
    Ok(())
}

async fn assert_meeting_token_generation_roundtrip(
    client: &Client,
    builder: MeetingTokenBuilder<'_>,
) -> anyhow::Result<()> {
    let token_fetch = builder.create(&client).await?;

    let returned = client.get_meeting_token(&token_fetch).await?;
    assert_builder_matches_retrieved(&builder, &returned);

    #[cfg(feature = "self-signed-tokens")]
    {
        let secret_key = get_secret_key_for_tests();
        let domain_id = get_domain_id_for_tests();
        let token_signed = builder.self_sign(&domain_id, &secret_key);
        let returned = client.get_meeting_token(&token_signed).await?;
        assert_builder_matches_retrieved(&builder, &returned);
    }
    Ok(())
}

fn assert_builder_matches_retrieved(_expected: &MeetingTokenBuilder, _result: &MeetingToken) {
    todo!()
}

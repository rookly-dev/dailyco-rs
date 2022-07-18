use crate::helpers::{delete_room_if_exists, ensure_room, get_daily_client};
use dailyco::meeting_token::MeetingTokenBuilder;
use dailyco::room::{RoomBuilder, RoomPrivacy};
use dailyco::{DailyCoErrorKind, Error};

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
async fn make_meeting_token() {
    let client = get_daily_client();
    let room_name = "meeting-token-room";
    ensure_room(&client, room_name).await;

    MeetingTokenBuilder::new()
        .room_name(room_name)
        .create(&client)
        .await
        .unwrap();
}

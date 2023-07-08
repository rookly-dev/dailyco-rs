use crate::helpers::{assert_not_found_err, delete_room_if_exists, ensure_room, get_daily_client};
use dailyco::meeting_token::CreateMeetingToken;
use dailyco::room::{CreateRoom, RoomPrivacy};

#[tokio::test]
async fn can_make_room() {
    let client = get_daily_client();
    let room_name = "test-room";
    delete_room_if_exists(&client, room_name).await;
    let room = CreateRoom::new()
        .privacy(RoomPrivacy::Private)
        .name(room_name)
        .start_video_off(true)
        .start_audio_off(true)
        .send(&client)
        .await
        .unwrap();

    assert_eq!(room.name, room_name);
    assert_eq!(room.privacy, RoomPrivacy::Private);
    assert!(room.config.start_audio_off);
    assert!(room.config.start_video_off);
}

#[tokio::test]
async fn get_room_not_found() {
    let client = get_daily_client();
    let res = client.get_room("not-found").await;
    assert_not_found_err(res);
}

#[tokio::test]
async fn delete_room() {
    let client = get_daily_client();
    let room_name = "room-to-delete";
    CreateRoom::new()
        .name(room_name)
        .send(&client)
        .await
        .unwrap();
    assert!(client.delete_room(room_name).await.is_ok());
}

#[tokio::test]
async fn get_room() {
    let client = get_daily_client();
    let room_name = "room-to-get";
    delete_room_if_exists(&client, room_name).await;
    CreateRoom::new()
        .name(room_name)
        .eject_at_room_exp(true)
        .max_participants(12)
        .send(&client)
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
    let created_room = CreateRoom::new().send(&client).await.unwrap();
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

    CreateMeetingToken::new()
        .room_name(room_name)
        .send(&client)
        .await
        .unwrap();
}

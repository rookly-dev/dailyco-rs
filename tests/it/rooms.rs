use dailyco::meeting_token::CreateMeetingToken;
use dailyco::room::{CreateRoom, RoomPrivacy, UpdateRoom};
use dailyco::RoomPropertiesBuilder;
use nanoid::nanoid;

use crate::helpers::{assert_not_found_err, cleanup_room, create_default_room, get_daily_client};

#[tokio::test]
async fn can_make_room() {
    let client = get_daily_client();
    let room_name = nanoid!(8);

    let room = CreateRoom::new()
        .privacy(RoomPrivacy::Private)
        .name(&room_name)
        .properties(
            RoomPropertiesBuilder::new()
                .start_video_off(true)
                .start_audio_off(true),
        )
        .send(&client)
        .await
        .unwrap();

    assert_eq!(room.name, room_name);
    assert_eq!(room.privacy, RoomPrivacy::Private);
    assert!(room.config.start_audio_off);
    assert!(room.config.start_video_off);

    cleanup_room(&client, &room_name).await;
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
    let room_name = nanoid!(8);
    CreateRoom::new()
        .name(&room_name)
        .send(&client)
        .await
        .unwrap();
    assert!(client.delete_room(&room_name).await.is_ok());
}

#[tokio::test]
async fn get_room() {
    let client = get_daily_client();
    let room_name = nanoid!(8);
    CreateRoom::new()
        .name(&room_name)
        .properties(
            RoomPropertiesBuilder::new()
                .eject_at_room_exp(true)
                .max_participants(12)
                .sfu_always(),
        )
        .send(&client)
        .await
        .unwrap();
    let room = client.get_room(&room_name).await.unwrap();
    assert_eq!(room.name, room_name);
    assert_eq!(room.privacy, RoomPrivacy::Public);
    assert!(room.api_created);
    assert_eq!(room.config.max_participants, Some(12));
    assert!(room.config.eject_at_room_exp);
    assert_eq!(room.config.sfu_switchover, Some(0.5));

    cleanup_room(&client, &room_name).await;
}

#[tokio::test]
async fn update_room() {
    let client = get_daily_client();
    let room = create_default_room(&client).await;
    let room_name = &room.name;
    UpdateRoom::new()
        .properties(RoomPropertiesBuilder::new().sfu_always())
        .send(room_name, &client)
        .await
        .unwrap();
    let room_after_update = client.get_room(&room_name).await.unwrap();
    assert_eq!(&room_after_update.name, room_name);
    assert_eq!(room_after_update.privacy, RoomPrivacy::Public);
    assert_eq!(room_after_update.config.sfu_switchover, Some(0.5));

    cleanup_room(&client, &room_name).await;
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

    cleanup_room(&client, &created_room.name).await;
}

#[tokio::test]
async fn make_meeting_token() {
    let client = get_daily_client();
    let room = create_default_room(&client).await;

    CreateMeetingToken::new()
        .room_name(&room.name)
        .send(&client)
        .await
        .unwrap();
    cleanup_room(&client, &room.name).await;
}

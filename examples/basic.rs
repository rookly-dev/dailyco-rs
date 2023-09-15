use dailyco::meeting_token::CreateMeetingToken;
use dailyco::room::{CreateRoom, RoomPrivacy};
use dailyco::RoomPropertiesBuilder;

#[tokio::main]
async fn main() -> dailyco::Result<()> {
    let client = dailyco::Client::new("test-api-key")?;

    // Make a customized room
    let created_room = CreateRoom::new()
        .name("my-test-room")
        .privacy(RoomPrivacy::Private)
        .properties(
            RoomPropertiesBuilder::new()
                .enable_screenshare(false)
                .max_participants(20)
                .start_audio_off(true),
        )
        .send(&client)
        .await?;

    // Since it is a private room, we will need a meeting token to join it! Let's give
    // ourselves owner privileges while we're at it.
    let _meeting_token = CreateMeetingToken::new()
        .room_name(&created_room.name)
        .is_owner(true)
        .send(&client)
        .await?;
    Ok(())
}

use dailyco::room::RoomBuilder;
use dailyco::{Client, DailyCoErrorKind, Error};

pub fn get_secret_key_for_tests() -> String {
    std::env::var("DAILY_CO_API_KEY").expect("Requires Daily API KEY")
}

pub fn get_daily_client() -> Client {
    dotenv::dotenv().unwrap();
    let key = get_secret_key_for_tests();
    Client::new(key).expect("Should make client")
}

pub async fn delete_room_if_exists(client: &Client, room_name: &str) {
    if let Err(err) = client.delete_room(room_name).await {
        match err {
            Error::APIError(api_err) => {
                assert!(matches!(api_err.error.unwrap(), DailyCoErrorKind::NotFound))
            }
            _ => panic!("Unexpected error"),
        }
    }
}

pub async fn ensure_room(client: &Client, room_name: &str) {
    delete_room_if_exists(client, room_name).await;
    RoomBuilder::new()
        .name(room_name)
        .create(client)
        .await
        .unwrap();
}

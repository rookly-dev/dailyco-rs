use dailyco::room::CreateRoom;
use dailyco::{Client, DailyCoErrorKind, Error};
use std::fmt::Debug;

pub fn get_secret_key_for_tests() -> String {
    std::env::var("DAILY_CO_API_KEY").expect("Requires Daily API KEY")
}

pub fn assert_not_found_err<T: Debug>(resp: dailyco::Result<T>) {
    match resp.expect_err("Result was not an error") {
        Error::APIError(err) => {
            assert!(matches!(
                err.error.expect("No error information"),
                DailyCoErrorKind::NotFound
            ))
        }
        err => panic!("Expected not found error, found {err}"),
    }
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
    CreateRoom::new()
        .name(room_name)
        .send(client)
        .await
        .unwrap();
}

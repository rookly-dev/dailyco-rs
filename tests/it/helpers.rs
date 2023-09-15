use std::fmt::Debug;

use dailyco::room::{CreateRoom, Room};
use dailyco::{Client, DailyCoErrorKind, Error};

pub fn get_secret_key_for_tests() -> String {
    dotenv::dotenv().unwrap();
    std::env::var("TEST_API_KEY")
        .expect("Requires a Daily API KEY for testing to be set in `TEST_API_KEY`")
}

#[cfg(feature = "self-signed-tokens")]
pub fn get_domain_id_for_tests() -> String {
    dotenv::dotenv().unwrap();
    std::env::var("TEST_DOMAIN_ID")
        .expect("Daily Domain Id required for testing self-signing meeting tokens")
}

pub fn assert_not_found_err<T: Debug>(resp: dailyco::Result<T>) {
    match resp.expect_err("Result was not an error") {
        Error::APIError(err) => {
            assert!(matches!(
                err.error.expect("No error information"),
                DailyCoErrorKind::NotFound
            ))
        }
        err => panic!("Expected not found error, found {}", err),
    }
}

pub fn get_daily_client() -> Client {
    let key = get_secret_key_for_tests();
    Client::new(key).expect("Should make client")
}

pub async fn cleanup_room(client: &Client, room_name: &str) {
    client
        .delete_room(room_name)
        .await
        .expect("Could not delete room");
}

pub async fn create_default_room(client: &Client) -> Room {
    CreateRoom::new().send(client).await.unwrap()
}

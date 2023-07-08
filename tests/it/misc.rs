use crate::helpers::{assert_not_found_err, get_daily_client};
use dailyco::recording::{GetRecordingAccessLink, ListRecordings};
use uuid::Uuid;

#[tokio::test]
async fn get_recording_not_found() {
    let client = get_daily_client();

    let res = client.get_recording(Uuid::new_v4()).await;
    assert_not_found_err(res);
}

#[tokio::test]
async fn delete_recording_not_found() {
    let client = get_daily_client();

    let res = client.delete_recording(Uuid::new_v4()).await;
    assert_not_found_err(res);
}

#[tokio::test]
async fn get_recording_access_link_not_found() {
    let client = get_daily_client();
    let res = GetRecordingAccessLink::new()
        .valid_for_secs(5000)
        .send(&client, Uuid::new_v4())
        .await;
    assert_not_found_err(res);
}

#[tokio::test]
async fn list_recordings() -> dailyco::Result<()> {
    let client = get_daily_client();

    // Would be nice to assert something on results, but who knows what's on the domain
    // used for testing
    let _ = ListRecordings::new().limit(20).send(&client).await?;
    Ok(())
}

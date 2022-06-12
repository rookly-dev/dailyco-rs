# dailyco

Rust bindings to interact with the [`Daily` API](https://docs.daily.co/reference/rest-api).

We aim to eventually support the entire API surface, for now this crate supports

- [Room](https://docs.daily.co/reference/rest-api/rooms) creation
- Getting + deleting room(s)
- [Meeting tokens](https://docs.daily.co/reference/rest-api/meeting-tokens)

## Example

Let's make a customized meeting room and generate a meeting token to join it. To run
this example you will need to make an account with [Daily](https://www.daily.co/) to
generate a valid API key.

```rust,no_run
use dailyco::meeting_token::MeetingTokenBuilder;
use dailyco::room::{RoomBuilder, RoomPrivacy};

#[tokio::main]
async fn main() -> dailyco::Result<()> {
    let client = dailyco::Client::new("test-api-key")?;

    // Make a customized room
    let created_room = RoomBuilder::new()
        .name("my-test-room")
        .privacy(RoomPrivacy::Private)
        .enable_screenshare(false)
        .max_participants(20)
        .start_audio_off(true)
        .create(&client)
        .await?;

    // Since it is a private room, we will need a meeting token to join it! Let's give
    // ourselves owner privileges while we're at it.
    let _meeting_token = MeetingTokenBuilder::new()
        .room_name(&created_room.name)
        .is_owner(true)
        .create(&client)
        .await?;
    Ok(())
}
```

## Installation

The `dailyco` client is just a thin wrapper around a `reqwest::Client`. Because of this,
in future we aim to add a feature set which forwards relevant features to `reqwest`. For
now,

```toml
[dependencies]
dailyco = { version = "0.1.0" }
```



## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
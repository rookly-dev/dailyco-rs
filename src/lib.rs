#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! This crate provides Rust bindings to interact with the [`Daily` API](https://docs.daily.co/reference/rest-api).
mod client;
pub mod configuration;
mod error;
pub mod meeting_token;
pub mod room;
mod room_properties;

#[cfg(feature = "self-signed-tokens")]
mod self_sign_token;

pub mod recording;
mod utils;

pub use room_properties::{RoomProperties, RoomPropertiesBuilder};

pub use self::client::Client;
pub use self::error::{DailyCoErrorInfo, DailyCoErrorKind, Error, Result};

#[cfg(doctest)]
doc_comment::doctest!("../README.md", readme);

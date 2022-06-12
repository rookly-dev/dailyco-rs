use std::fmt::Formatter;

use reqwest::Response;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A `Result` alias where the `Err` case is `dailyco::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// The possible errors when making requests to `Daily`.
#[derive(Error, Debug)]
pub enum Error {
    /// Error related processing the request.
    #[error("failure making the request")]
    Request(#[from] reqwest::Error),
    /// Error reported by `Daily`.
    #[error("daily request returned an error: {0}")]
    APIError(DailyCoErrorInfo),
    /// Invalid API key.
    #[error("API key problem: {0}")]
    BadAPIKey(&'static str),
    /// Request which requires pagination to return full result, unimplemented.
    #[error("Response requires pagination, which is not implemented yet.")]
    RequiresPagination,
}

impl Error {
    pub(crate) async fn from_failed_daily_request(response: Response) -> Self {
        match response.json().await {
            Ok(error) => Self::APIError(error),
            Err(err) => Self::Request(err),
        }
    }
}

/// The `error` type returned by `Daily`, defined [here](https://docs.daily.co/reference/rest-api#errors).
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DailyCoErrorKind {
    /// The API key is not valid.
    AuthenticationError,
    /// The Authorization header is missing or badly formatted.
    AuthorizationHeaderError,
    /// The JSON request body could not be parsed.
    JsonParsingError,
    /// The request could not be performed. More information is usually available in the info field
    /// of the response body. Typical causes are missing required parameters,
    /// bad parameter values, etc.
    InvalidRequestError,
    /// Too many requests were sent in too short a period of time.
    RateLimitError,
    /// Something unexpected went wrong.
    ServerError,
    /// Item not found.
    NotFound,
}

impl std::fmt::Display for DailyCoErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let disp = match self {
            Self::AuthenticationError => "authentication-error",
            Self::AuthorizationHeaderError => "authorization-header-error",
            Self::JsonParsingError => "json-parsing-error",
            Self::InvalidRequestError => "invalid-request-error",
            Self::RateLimitError => "rate-limit-error",
            Self::ServerError => "server-error",
            Self::NotFound => "not-found",
        };
        write!(f, "{}", disp)
    }
}

/// Information about the error returned by `Daily`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DailyCoErrorInfo {
    /// The fixed error type returned by `Daily`.
    pub error: Option<DailyCoErrorKind>,
    /// Informational description about the error.
    pub info: Option<String>,
}

impl std::fmt::Display for DailyCoErrorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Shouldn't hit any possible failure conditions
        let disp = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", disp)
    }
}

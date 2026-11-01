use std::fmt::Formatter;

use reqwest::Response;
use serde::{Deserialize, Serialize};

/// A `Result` alias where the `Err` case is `dailyco::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// The possible errors when making requests to `Daily`.
#[derive(Debug)]
pub enum Error {
    /// Error related processing the request.
    Request(reqwest::Error),
    /// Error reported by `Daily`.
    APIError(DailyCoErrorInfo),
    /// Invalid API key.
    BadAPIKey(&'static str),
    /// Request which requires pagination to return full result, unimplemented.
    RequiresPagination,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(_) => f.write_str("failure making the request"),
            Self::APIError(info) => write!(f, "daily request returned an error: {}", info),
            Self::BadAPIKey(problem) => write!(f, "API key problem: {}", problem),
            Self::RequiresPagination => {
                f.write_str("Response requires pagination, which is not implemented yet.")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Request(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
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
        f.write_str(disp)
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
        if let Some(error_kind) = self.error {
            write!(f, "Error: {}", error_kind)?;
        }
        if let Some(info) = &self.info {
            write!(f, "Info: {}", info)
        } else {
            f.write_str("Info: Unspecified")
        }
    }
}

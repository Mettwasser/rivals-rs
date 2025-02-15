use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    Reqwest(#[from] reqwest::Error),

    #[error("{_0}")]
    ApiError(ErrorResponse),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Display)]
#[display("{}", self.message)]
pub struct ErrorResponse {
    pub error: bool,

    pub message: String,

    pub status: i64,
}

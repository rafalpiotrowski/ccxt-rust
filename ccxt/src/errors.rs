use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("API does not support function {0}")]
    ApiFunctionNotSupported(&'static str),
    #[error("error {0} when calling API")]
    ApiCallError(String),
    #[error("Account Balance has no positions")]
    AccountBalanceEmpty(),
    #[error("http error {0}")]
    Http(#[from] super::reqwest::Error),
    #[error(transparent)]
    Serde(#[from] super::serde_json::Error),
    #[error("general exchange error")]
    ExchangeError(#[from] std::io::Error),
    #[error("undefined error")]
    Unknown,
}


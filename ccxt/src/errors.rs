use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http error {0}")]
    Http(#[from] super::reqwest::Error),
    #[error(transparent)]
    Serde(#[from] super::serde_json::Error),
    #[error("general exchange error")]
    ExchangeError(#[from] std::io::Error),
    #[error("undefined error")]
    Unknown,
}


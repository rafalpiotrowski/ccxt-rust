extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate reqwest;
extern crate anyhow;
extern crate async_trait;

pub mod errors;
pub mod exchange;
pub mod coinbase;
pub mod kraken;
pub mod rfc1123_date_format;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub type Result<R> = anyhow::Result<R, errors::Error>;
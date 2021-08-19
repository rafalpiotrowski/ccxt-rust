use async_trait::async_trait;
use crate::{DateTime, Result, exchange::*};

#[derive(Debug)]
pub struct Coinbase {
    pub exchange: Exchange
}

impl Coinbase {
    pub fn new(id: &'static str) -> Self {
        Coinbase {
            exchange: Exchange::new(id, "Coinbase")
            .rate_limit(400)
            .headers("CB-VERSION", "2018-05-30")
            .user_agent(UserAgent::Chrome)
            .countries(Country::UnitedStates)
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Time {
    iso: DateTime,
    epoch: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename(deserialize = "data"), rename_all = "camelCase")]
pub struct Data<D> {
    data: D
}

#[async_trait]
impl ServerTime for Coinbase {
    async fn get_time(&self) -> Result<DateTime> {
        
        let request_url = format!("https://api.coinbase.com/v2/time");
        println!("{}", request_url);
        let response = reqwest::get(&request_url).await?;

        let res: Data<Time> = response.json::<Data<Time>>().await?;
        println!("{:?}", res);
        Ok(res.data.iso)
    }
}
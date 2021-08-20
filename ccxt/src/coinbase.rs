use std::collections::HashMap;

use async_trait::async_trait;
use crate::{ApiRequest, DateTime, Result, exchange::*, errors::Error};

#[derive(Debug)]
pub struct Coinbase {
    pub exchange: Exchange,
    pub api: Api,
    pub http_client: reqwest::Client
}

impl Coinbase {
    pub fn new(id: &'static str) -> Self {
        Coinbase {
            exchange: Exchange::new(id, "Coinbase")
            .rate_limit(400)
            .headers("CB-VERSION", "2018-05-30")
            .user_agent(UserAgent::Chrome)
            .countries(Country::UnitedStates),
            api: Api::new("https://api.coinbase.com", "v2")
            .api_key("1234".to_string())
            .api_secret("111".to_string())
            .function(Functionality::Time,FunctionalityParams::new(AccessType::Public, Action::Get, "time")),
            http_client: reqwest::Client::new()  
            }
        }
}

impl ApiCalls for Coinbase {

    fn get_url(&self, params: &FunctionalityParams) -> String {
        format!("{}{}", self.api.url, self.get_uri_path(params))
    }

    fn get_uri_path(&self, params: &FunctionalityParams) -> String {
        format!("/{}/{}", 
            self.api.version, 
            params.uri_path)
    }

    fn get_request(&self, f: &Functionality, payload: HashMap<&str, String>) -> Result<ApiRequest>
    {
        let fp = self.api.get_function_params(f)?;

        match fp.access_type {
            AccessType::Public => {
                return Ok(
                    self.http_client.get(self.get_url(fp))
                        );
            },
            AccessType::Private => {
                Err(Error::ApiFunctionNotSupported("Private secure functions"))
            }
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
        
        let rb = self.get_request(&Functionality::Time, HashMap::<&str, String>::new())?;
        let r = rb.send().await?;

        let res: Data<Time> = r.json::<Data<Time>>().await?;
        println!("{:?}", res);
        Ok(res.data.iso)
    }
}
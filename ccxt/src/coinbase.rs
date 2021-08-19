use async_trait::async_trait;
use crate::{DateTime, Result, exchange::*};

#[derive(Debug)]
pub struct Coinbase {
    pub exchange: Exchange,
    pub api: Api
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
            .api_key("1234")
            .api_secret("111")
            .function(Functionality::Time,FunctionalityParams::new(AccessType::Public, Action::Get, "time"))
            }
        }
}

impl ApiCalls for Coinbase {

    fn get_url(&self, f: &Functionality) -> Result<String> {
        let p = self.api.get_function_params(f)?;
        let _at = match p.access_type {
            AccessType::Private => "private",
            AccessType::Public => "public"
        };
        Ok(format!("{}/{}/{}", 
            self.api.url, 
            self.api.version, 
            p.url_path))
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
        
        let request_url = self.get_url(&Functionality::Time)?;
        println!("url: {}", request_url);
        let response = reqwest::get(&request_url).await?;

        let res: Data<Time> = response.json::<Data<Time>>().await?;
        println!("{:?}", res);
        Ok(res.data.iso)
    }
}
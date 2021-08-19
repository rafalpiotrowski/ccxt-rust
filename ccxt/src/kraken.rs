use async_trait::async_trait;
use crate::{DateTime, Result, exchange::*, errors::Error};
use chrono::prelude::Utc;

#[derive(Debug)]
pub struct Kraken {
    pub exchange: Exchange,
    pub api: Api,
}

impl Kraken {
    pub fn new(id: &'static str) -> Self {
        Kraken {
            exchange: Exchange::new(id, "Kraken")
            .rate_limit(3000)
            .countries(Country::UnitedStates),
            api: Api::new("https://api.kraken.com", "0")
            .api_key("1234")
            .api_secret("111")
            .function(Functionality::Time,FunctionalityParams::new(AccessType::Public, Action::Get, "Time"))
            .function(Functionality::SystemStatus,FunctionalityParams::new(AccessType::Public, Action::Get, "SystemStatus"))
            }
    }

    fn get_function_params(&self, f: &Functionality) -> Result<&FunctionalityParams> {
        let p = self.api.functions.get(f);
        match p {
            Some(par) => Ok(par),
            None => Err(Error::ApiFunctionNotSupported("function not supported by api"))
        }
    }

    pub fn get_function_url(&self, f: &Functionality) -> Result<String> {
        let p = self.get_function_params(f)?;
        let at = match p.access_type {
            AccessType::Private => "private",
            AccessType::Public => "public"
        };
        Ok(format!("{}/{}/{}/{}", 
            self.api.url, 
            self.api.version, 
            at,
            p.url_path))
    }
}


#[derive(Deserialize, Debug)]
pub struct Time {
    #[serde(with = "kraken_date_format")]
    rfc1123: DateTime,
    unixtime: u64,
}
impl Default for Time {
    fn default() -> Self {
        Time { 
            rfc1123: Utc::now(), 
            unixtime: 0
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename(deserialize = ""), rename_all = "camelCase")]
pub struct Data<R> {
    error: Vec<String>,
    #[serde(default)]
    result: R
}

#[async_trait]
impl ServerTime for Kraken {

    async fn get_time(&self) -> Result<DateTime> {
        
        let request_url = self.get_function_url(&Functionality::Time)?;
        println!("url: {}", request_url);

        let response = reqwest::get(&request_url).await?;

        let res = response.json::<Data<Time>>().await?;
        if res.error.len() > 0 {
            return Err(Error::ApiCallError(res.error[0].clone()));
        }
        println!("{:?}", res);
        Ok(res.result.rfc1123)
    }
}

mod kraken_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    //"rfc1123": "Sun, 21 Mar 21 14:23:14 +0000"
    const FORMAT: &'static str = "%a, %e %b %y %H:%M:%S %z";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        println!("parsing: {}", s);
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
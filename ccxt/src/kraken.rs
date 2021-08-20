use std::collections::HashMap;

use async_trait::async_trait;
use crate::{DateTime, Result, exchange::*, errors::Error};
use chrono::prelude::Utc;
use sha2::{Digest, Sha256, Sha512};
use data_encoding::{BASE64};
use hmac::*;
use crate::rfc1123_date_format::*;

#[derive(Debug)]
pub struct Kraken {
    pub exchange: Exchange,
    pub api: Api,
    pub http_client: reqwest::Client
}

impl Kraken {
    pub fn new(id: &'static str, api_key: String, api_secret: String) -> Self {
        Kraken {
            exchange: Exchange::new(id, "Kraken")
            .rate_limit(3000)
            .countries(Country::UnitedStates),
            api: Api::new("https://api.kraken.com", "0")
            .api_key(api_key)
            .api_secret(api_secret)
            .function(Functionality::Time,FunctionalityParams::new(AccessType::Public, Action::Get, "Time"))
            .function(Functionality::SystemStatus,FunctionalityParams::new(AccessType::Public, Action::Get, "SystemStatus"))
            .function(Functionality::Balance,FunctionalityParams::new(AccessType::Private, Action::Post, "Balance")),
            http_client: reqwest::Client::new()    
            }
    }

    pub fn get_signature(&self, uri_path: &String, post_data: &String, nonce: &String) -> Result<String> {
        let message_presha256 = format!("{}{}", nonce, post_data);

        let mut sha256 = Sha256::default();
        sha256.update(&message_presha256.as_bytes());

        let output = sha256.finalize();

        let mut concatenated = uri_path.as_bytes().to_vec();
        for elem in output {
            concatenated.push(elem);
        }

        let s = self.api.secret.as_ref().unwrap();
        let hmac_key = BASE64.decode(s.as_bytes()).unwrap();
        let mut mac = Hmac::<Sha512>::new_from_slice(&hmac_key[..]).unwrap();
        mac.update(&concatenated);
        Ok(BASE64.encode(&mac.finalize().into_bytes()))
    }
}

impl ApiCalls for Kraken {

    fn get_url(&self, f: &Functionality) -> Result<String> {
        Ok(format!("{}{}", self.api.url, self.get_uri_path(f).unwrap()))
    }

    fn get_uri_path(&self, f: &Functionality) -> Result<String> {
        let p = self.api.get_function_params(f)?;
        Ok(format!("/{}/{}/{}",
            self.api.version, 
            match p.access_type {
                AccessType::Private => "private",
                AccessType::Public => "public"
            },
            p.uri_path))
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct AccountBalance {
    pub CHF: String
}

#[derive(Deserialize, Debug)]
pub struct Time {
    #[serde(with = "rfc1123_date_format")]
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
pub struct Status {
    timestamp: DateTime,
    status: String,
}
impl Default for Status {
    fn default() -> Self {
        Status { 
            timestamp: Utc::now(), 
            status: "".to_string()
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
impl SystemStatus for Kraken {

    async fn get_status(&self) -> Result<String> {
        let request_url = self.get_url(&Functionality::SystemStatus)?;
        println!("url: {}", request_url);
        
        let response = self.http_client.get(&request_url).send().await?;

        let res = response.json::<Data<Status>>().await?;
        if res.error.len() > 0 {
            return Err(Error::ApiCallError(res.error[0].clone()));
        }
        println!("{:?}", res);
        Ok(res.result.status)
    }
}

#[async_trait]
impl ServerTime for Kraken {

    async fn get_time(&self) -> Result<DateTime> {
        
        let request_url = self.get_url(&Functionality::Time)?;
        println!("url: {}", request_url);

        let response = self.http_client.get(&request_url).send().await?;

        let res = response.json::<Data<Time>>().await?;
        if res.error.len() > 0 {
            return Err(Error::ApiCallError(res.error[0].clone()));
        }
        println!("{:?}", res);
        Ok(res.result.rfc1123)
    }
}

#[async_trait]
impl Balance for Kraken {
    async fn get_balance(&self) -> Result<String> {
        
        let uri_path = self.get_uri_path(&Functionality::Balance)?;
        println!("uri: {}", uri_path);
        
        let nonce = Utc::now().timestamp_millis().to_string();
        
        let mut params = HashMap::new();
        params.insert("nonce", nonce.clone());
        
        let post_data = Api::encode_uri(&params);
        
        let signature = self.get_signature(&uri_path, &post_data, &nonce)?;

        let f = self.http_client.post(format!("{}{}", self.api.url, uri_path))
            .header("API-Key", self.api.key.as_ref().unwrap())
            .header("API-Sign", &signature)
            .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8".to_string())
            .form(&params);
        println!("Reqest: {:?}", f);
        let res = f.send().await?;

        let s = res.text().await?;
 /*        
        let res = res.json::<Data<AccountBalance>>().await?;
        if res.error.len() > 0 {
            return Err(Error::ApiCallError(res.error[0].clone()));
        }
        println!("{:?}", res);
         */
        Ok(s)
    }
}
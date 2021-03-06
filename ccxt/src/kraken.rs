use std::{collections::HashMap, net::ToSocketAddrs};

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use crate::{ApiRequest, DateTime, Result, errors::Error, exchange::*};
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
            .function(Functionality::Balance,FunctionalityParams::new(AccessType::Private, Action::Post, "Balance"))
            .function(Functionality::GetWebSocketsToken,FunctionalityParams::new(AccessType::Private, Action::Get, "GetWebSocketsToken"))
            .function(Functionality::SystemStatus,FunctionalityParams::new(AccessType::Public, Action::Get, "SystemStatus"))
            .function(Functionality::Time,FunctionalityParams::new(AccessType::Public, Action::Get, "Time"))
            ,
            http_client: reqwest::Client::new()    
            }
    }

    pub fn get_signature(&self, uri_path: &String, post_data: &String, nonce: &String) -> String {
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
        BASE64.encode(&mac.finalize().into_bytes())
    }

    pub async fn get_data_no_params<T>(&self, f: &Functionality) -> Result<T> 
        where T: DeserializeOwned
    {
        self.get_data::<T>(f, HashMap::<&str, String>::new()).await
    }

    pub async fn get_data<T>(&self, f: &Functionality, payload: HashMap<&str, String>) -> Result<T> 
    where T: DeserializeOwned
    {
        let rb   = self.get_request(f, payload)?;
        let r = rb.send().await?;

        let res = r.json::<Data<T>>().await?;
        if res.error.len() > 0 {
            return Err(Error::ApiCallError(res.error[0].clone()));
        }
        match res.result {
            None => Err(Error::ApiCallNoData()),
            Some(a) => Ok(a),
        }
    }
}

impl ApiCalls for Kraken {

    fn get_url(&self, params: &FunctionalityParams) -> String {
        format!("{}{}", self.api.url, self.get_uri_path(params))
    }

    fn get_uri_path(&self, params: &FunctionalityParams) -> String {
        format!("/{}/{}/{}",
            self.api.version, 
            match params.access_type {
                AccessType::Private => "private",
                AccessType::Public => "public"
            },
            params.uri_path)
    }

    fn get_request(&self, f: &Functionality, payload: HashMap<&str, String>) -> Result<ApiRequest>
    {
        let fp = self.api.get_function_params(f)?;

        match fp.access_type {
            AccessType::Public => {
                return Ok(
                    self.http_client.get(self.get_url(fp))
                        .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8".to_string())
                        .form(&payload));
            },
            AccessType::Private => {
                let uri_path = self.get_uri_path(&fp);
        
                let nonce = Utc::now().timestamp_millis().to_string();
                
                let mut params = HashMap::new();
                params.insert("nonce", nonce.clone());
                params.extend(payload);
                
                let post_data = Api::encode_uri(&params);
                
                let signature = self.get_signature(&uri_path, &post_data, &nonce);
        
                let req = self.http_client.post(format!("{}{}", self.api.url, uri_path))
                    .header("API-Key", self.api.key.as_ref().unwrap())
                    .header("API-Sign", &signature)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8".to_string())
                    .form(&params);
        
                Ok(req)
            }
        }        
    }
}

impl Kraken {
    
    pub async fn get_web_socket_token(&self) -> Result<WebSocketToken> {
        let t = 3;
        if t {
            println!("");
        }
        self.get_data_no_params::<WebSocketToken>(&Functionality::GetWebSocketsToken).await
    }
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
    result: Option<R>
}

#[async_trait]
impl SystemStatus for Kraken {
    async fn get_status(&self) -> Result<String> {
        let r = self.get_data_no_params::<Status>(& Functionality::SystemStatus).await?;
        Ok(r.status)
    }
}

#[async_trait]
impl ServerTime for Kraken {
    async fn get_time(&self) -> Result<DateTime> {
        let time = self.get_data_no_params::<Time>(& Functionality::Time).await?;
        Ok(time.rfc1123)
    }
}

#[async_trait]
impl Balance for Kraken {
    async fn get_balance(&self) -> Result<AccountBalance> {
        self.get_data_no_params::<AccountBalance>(& Functionality::Balance).await      
    }
}
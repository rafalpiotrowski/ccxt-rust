use std::{collections::BTreeMap};
use async_trait::async_trait;

use crate::{Result, DateTime, errors::Error};

#[derive(Debug,Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessType {
    Private,
    Public
}

#[derive(Debug,Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Get,
    Post,
    Delete
}

pub type UrlPath = &'static str;


#[derive(Clone, Debug, Default, PartialEq)]
pub struct Api {
    pub url: &'static str,
    pub key: Option<&'static str>,
    pub secret: Option<&'static str>,
    pub version: &'static str,
    pub functions: BTreeMap<Functionality, FunctionalityParams>
}

impl Api {
    pub fn new(url: &'static str, version: &'static str) -> Self {
        Api {
            url,
            version,
            ..Default::default()
        }
    }
    pub fn function(mut self, f: Functionality, p: FunctionalityParams) -> Self {
        self.functions.insert(f, p);
        self
    }

    pub fn api_key(mut self, value: &'static str) -> Self {
        self.key = Some(value);
        self
    }

    pub fn api_secret(mut self, value: &'static str) -> Self {
        self.secret = Some(value);
        self
    }

    pub fn get_function_params(&self, f: &Functionality) -> Result<&FunctionalityParams>
    {
        let p = self.functions.get(f);
        match p {
            Some(par) => Ok(par),
            None => Err(Error::ApiFunctionNotSupported("function not supported by api"))
        }
    }
}

#[derive(Debug,Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Functionality {
    AddOrder,
    Assets,
    AssetPairs,
    Balance,
    CancelOrder,
    ClosedOrders,
    DepositAddresses,
    DepositMethods,
    DepositStatus,
    Depth,
    GetWebSocketsToken,
    Ledgers,
    OHLC,
    OpenOrders,
    OpenPositions,
    QueryLedgers,
    QueryOrders,
    QueryTrades,
    Spread,
    SystemStatus,
    Ticker,
    Time,
    TradeBalance,
    TradeVolume,
    Trades,
    TradesHistory,
    Withdraw,
    WithdrawCancel,
    WithdrawInfo,
    WithdrawStatus,
}


#[derive(Debug,Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionalityParams {
    pub access_type: AccessType,
    pub action: Action,
    pub url_path: &'static str
}

impl FunctionalityParams {
    pub fn new(access_type: AccessType, action: Action, url_path: &'static str) -> Self {
        FunctionalityParams {
            access_type,
            action,
            url_path
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Country {
    Japan,
    Poland,
    UnitedStates
}
impl Country {
    pub fn get_iso_code(&self) -> &'static str {
        match self {
            Country::Japan => "JP",
            Country::Poland => "PL",
            Country::UnitedStates => "US",            
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserAgent {
    Chrome,
    Chrome39,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Exchange {
    pub id: &'static str,
    pub name: &'static str,
    pub countries: Vec<Country>,
    pub rate_limit: Option<u32>,
    pub certified: bool,
    pub pro: bool,
    pub user_agent: Option<(UserAgent,&'static str)>,
    pub headers: BTreeMap<&'static str, &'static str>,
}

impl Exchange {
    pub fn new(id:&'static str, name:&'static str) -> Self {
        Exchange {
            id,
            name,
            ..Default::default()
        }
    }

    pub fn rate_limit(mut self, value: u32) -> Self {
        self.rate_limit = Some(value);
        self
    }

    pub fn certified(mut self, value: bool) -> Self {
        self.certified = value;
        self
    }

    pub fn pro(mut self, value: bool) -> Self {
        self.pro = value;
        self
    }

    pub fn user_agent(mut self, agent: UserAgent) -> Self {
        self.user_agent = match agent {
            UserAgent::Chrome => Some((UserAgent::Chrome, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36")),
            UserAgent::Chrome39 => Some((UserAgent::Chrome39, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36"))
        };
        self
    }

    pub fn countries(mut self, value: Country) -> Self {
        self.countries.push(value);
        self
    }

    pub fn headers(mut self, key: &'static str, value: &'static str) -> Self {
        self.headers.insert(key, value);
        self
    }
}
pub trait ApiCalls {
    fn get_url(&self, f: &Functionality) -> Result<String>;
}

#[async_trait]
pub trait ServerTime {
    async fn get_time(&self) -> Result<DateTime>;
}

#[async_trait]
pub trait SystemStatus {
    async fn get_status(&self) -> Result<String>;
}
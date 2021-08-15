use std::collections::BTreeMap;
use async_trait::async_trait;

use crate::{Result, DateTime};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Functionality {
    Cors,
    CancelAllOrders,
    CreateDepositAddress,
    CreateOrder
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Default, PartialEq)]
pub struct Exchange {
    pub id: &'static str,
    pub name: &'static str,
    pub countries: Vec<Country>,
    pub rate_limit: Option<u32>,
    pub certified: bool,
    pub pro: bool,
    pub version: Option<&'static str>,
    pub user_agent: Option<(UserAgent,&'static str)>,
    pub headers: BTreeMap<&'static str, &'static str>,
    pub has: BTreeMap<Functionality, bool>
}

#[async_trait]
pub trait ServerTime {
    async fn get_time(&self) -> Result<DateTime>;
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

    pub fn version(mut self, value: &'static str) -> Self {
        self.version = Some(value);
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

    pub fn has(mut self, key: Functionality, value: bool) -> Self {
        self.has.insert(key, value);
        self
    }

    pub fn headers(mut self, key: &'static str, value: &'static str) -> Self {
        self.headers.insert(key, value);
        self
    }
}
use std::time::Duration;


use serde::{Serialize, Deserialize, Deserializer};
use reqwest_middleware::ClientWithMiddleware;

use crate::primitives::frozen::text::BoxUuid;
use crate::sql_models::user::implements::User;
use crate::sql_models::person::implements::Person;
use crate::sql_models::company::implements::Company;


#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
pub struct SessionUser {
    pub user: User,
    pub person: Person,
    pub company: Company
}

#[derive(Debug)]
pub struct SessionUserDto {
    pub user: serde_json::Value,
    pub person: serde_json::Value,
    pub company: serde_json::Value
}

#[derive(Debug, Clone)]
pub struct ActiveSession {
    pub session_user: SessionUser,
    pub local_db: sqlx::SqlitePool,
    pub token: BoxUuid
}


impl std::convert::TryFrom<SessionUserDto> for SessionUser {
    type Error = serde_json::Error;
    fn try_from(dto: SessionUserDto) -> Result<Self, Self::Error> {
        Ok(Self { 
            user: serde_json::from_value(dto.user)?,
            person: serde_json::from_value(dto.person)?,
            company: serde_json::from_value(dto.company)?,
        })
    }
}


pub fn make_client(
    conn_timeout: Duration,
    tot_timeout: Duration,
    request_interval: Duration,
    retries: u32,
) -> ClientWithMiddleware {
    
    let client= reqwest::Client::builder()
        .connect_timeout(conn_timeout)
        .timeout(tot_timeout)
        .build()
        .expect("FATAL: Failed to build reqwest::Client");

    let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder()
        .jitter(reqwest_retry::Jitter::None)
        .retry_bounds(request_interval, request_interval * 100)
        .base(2)
        .build_with_max_retries(retries);

    reqwest_middleware::ClientBuilder::new(client)
        .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
        
}

pub mod time_parser {
    use super::*;
    use serde::Deserialize;

    pub fn duration_from_f64<'de, D>(func: D) -> Result<Duration, D::Error> 
    where 
        D: Deserializer<'de>  
    {
        let millis_raw = f64::deserialize(func)?;

        Ok(Duration::from_millis(millis_raw as u64))
    }  
}

#[macro_export]
macro_rules! make_header {
    ( [ $($key:expr => $val:expr),* $(,)? ] ) => {
        {
            let mut new_header = reqwest::header::HeaderMap::new();
            $(
                new_header.insert($key,
                    reqwest::header::HeaderValue::from_str(&$val.to_string())
                        .expect("HEADER_VALUE_ERROR!!!")
                );
            )*
            new_header
        }       
    };
}
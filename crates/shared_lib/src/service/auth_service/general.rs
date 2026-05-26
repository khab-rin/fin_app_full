use std::time::Duration;
use std::sync::OnceLock;

use serde::{Deserialize, Deserializer};
use reqwest_middleware::ClientWithMiddleware;



#[derive(Deserialize, Debug)]
pub struct NetWork {
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub inst_conn_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub inst_tot_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub inst_request_interval: Duration,
    pub inst_retries: u32,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub inst_poll_intervals: Duration,

    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub std_conn_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub std_tot_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub std_request_interval: Duration,
    pub std_retries: u32,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub std_poll_intervals: Duration,

    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub rel_conn_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub rel_tot_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub rel_request_interval: Duration,
    pub rel_retries: u32,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub rel_poll_intervals: Duration
}

#[derive(Default, Debug)]
pub struct Clients {
    pub instant: OnceLock<reqwest_middleware::ClientWithMiddleware>,
    pub standard: OnceLock<reqwest_middleware::ClientWithMiddleware>,
    pub relaxed: OnceLock<reqwest_middleware::ClientWithMiddleware>
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
    pub fn duration_from_u64<'de, D>(func: D) -> Result<Duration, D::Error> 
    where D: Deserializer<'de>  {
        let secs = u64::deserialize(func)?;
        Ok(Duration::from_secs(secs))
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
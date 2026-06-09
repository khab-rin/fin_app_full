use std::time::Duration;

use serde::{Deserialize, Deserializer};
use reqwest_middleware::ClientWithMiddleware;


pub fn make_client(
    conn_timeout: Duration,
    tot_timeout: Duration,
    request_interval: Duration,
    retries: u32,
) -> ClientWithMiddleware {
    
    let client= reqwest::Client::builder()
        .connect_timeout(conn_timeout)
        .timeout(tot_timeout)
        .no_proxy()
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
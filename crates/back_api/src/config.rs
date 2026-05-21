use std::sync::OnceLock;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, CONTENT_TYPE, ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Deserializer};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_middleware::ClientWithMiddleware;
use retry_policies::RetryPolicy;

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

pub(crate) mod time_parser {
    use super::*;
    pub(crate) fn duration_from_u64<'de, D>(func: D) -> Result<Duration, D::Error> 
    where D: Deserializer<'de>  {
        let secs = u64::deserialize(func)?;
        Ok(Duration::from_secs(secs))
    }  
}


#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    #[serde(skip)]
    pub(crate) data_base: DataBase,
    pub(crate) network: NetWork,
    pub(crate) dadata: Dadata,
    pub(crate) smsru: SmsRu,
    pub(crate) email_sender: EmailSender,
    #[serde(skip)]
    pub(crate) crypto_servise: CryptoService,
    #[serde(skip)]
    pub(crate) clients: Clients,
    #[serde(skip)]
    pub(crate) policies: RetryPolicies,
    #[serde(skip)]
    pub(crate) headers: Headers
}

#[derive(Default, Debug)]
pub(crate) struct Clients {
    pub(crate) instant: OnceLock<reqwest_middleware::ClientWithMiddleware>,
    pub(crate) standard: OnceLock<reqwest_middleware::ClientWithMiddleware>,
    pub(crate) relaxed: OnceLock<reqwest_middleware::ClientWithMiddleware>
}

#[derive(Default, Debug)]
pub(crate) struct RetryPolicies {
    pub(crate) instant: OnceLock<ExponentialBackoff>,
    pub(crate) standard: OnceLock<ExponentialBackoff>,
    pub(crate) relaxed: OnceLock<ExponentialBackoff>,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct DataBase {
    #[serde(skip, default)]
    pub(crate) database_url: String, 
}

#[derive(Deserialize, Debug)]
pub(crate) struct NetWork {
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) inst_conn_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) inst_tot_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) inst_request_interval: Duration,
    pub(crate) inst_retries: u32,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) inst_poll_intervals: Duration,

    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) std_conn_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) std_tot_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) std_request_interval: Duration,
    pub(crate) std_retries: u32,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) std_poll_intervals: Duration,

    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) rel_conn_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) rel_tot_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) rel_request_interval: Duration,
    pub(crate) rel_retries: u32,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) rel_poll_intervals: Duration
}

#[derive(Deserialize, Debug)]
pub(crate) struct Dadata {
    pub(crate) dadata_comp_url: String,
    pub(crate) dadata_adr_paid_url: String,
    #[serde(skip, default)]
    pub(crate) comp_free_api: String,
    #[serde(skip, default)]
    pub(crate) paid_api_key: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SmsRu {
    pub(crate) call_add_url: String,
    pub(crate) get_stat_url: String,
    #[serde(skip, default)]
    pub(crate) api: String 
}

#[derive(Deserialize, Debug)]
pub(crate) struct EmailSender {
    pub(crate) base_url: String,
    pub(crate) from: String,
    #[serde(skip, default)]
    pub(crate) api: String
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct CryptoService {
    #[serde(skip, default)]
    pub(crate) url: String,
}

#[derive(Default, Debug)]
pub(crate) struct Headers {
    pub(crate) dadata_header: OnceLock<HeaderMap>,
}

pub(crate) struct BackApiState {
    pub(crate) pool: sqlx::PgPool,
    pub(crate) config: &'static Config
}

impl Config {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<Config> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            dotenvy::dotenv().ok();
            let toml_str = include_str!("../config.toml"); 

            let mut config: Config = toml::from_str(toml_str)
                .expect("CONFIG_TOML_PARSE_ERROR!!!");

            config.data_base.database_url = std::env::var("DATABASE_URL")
                .expect("MISSING_ENV_DATABASE_URL!!");

            config.dadata.comp_free_api = std::env::var("DADATA_COMP_FREE_API")
                .expect("MISSING_ENV_DADATA_API_KEY!!!");

            config.dadata.paid_api_key = std::env::var("DADATA_PAID_KEY")
                .expect("MISSING_ENV_DADATA_PAID_KEY");
            
            config.smsru.api = std::env::var("SMSRU_API_KEY")
                .expect("MISS_SMSRU_API_KEY");

            config.email_sender.api = std::env::var("EMAIL_SENDER_API_KEY")
                .expect("MISS_EMAIL_SENDER_API_KEY");

            config.crypto_servise.url = std::env::var("CRYPTO_SERVICE_URL")
                .expect("MISS_CRYPTO_SERVICE_API_URL");

            let inst_client = make_client(
                config.network.inst_conn_timeout, 
                config.network.inst_tot_timeout, 
                config.network.inst_request_interval, 
                config.network.inst_retries);
            
            config.clients.instant.set(inst_client).expect("STATIC_MEMORY_ERROR!!!");
            
            let std_client = make_client(
                config.network.std_conn_timeout, 
                config.network.std_tot_timeout, 
                config.network.std_request_interval, 
                config.network.std_retries);
            
            config.clients.standard.set(std_client).expect("STATIC_MEMORY_ERROR!!!");
            
            let rel_client = make_client(
                config.network.rel_conn_timeout, 
                config.network.rel_tot_timeout, 
                config.network.rel_request_interval, 
                config.network.rel_retries);
            
            config.clients.relaxed.set(rel_client).expect("STATIC_MEMORY_ERROR!!!");
   
            let dadata_header = make_header!([
                CONTENT_TYPE => "application/json",
                ACCEPT => "application/json",
                AUTHORIZATION => format!("Token {}", config.dadata.comp_free_api),
                HeaderName::from_static("x-secret") =>
                    config.dadata.paid_api_key
            ]);

            config.headers.dadata_header.set(dadata_header).expect("STATIC_MEMORY_ERROR!!!");

            config
        })
    }

    pub fn get_inst_client(&self) -> &'static reqwest_middleware::ClientWithMiddleware {
        Self::global().clients.instant.get().expect("STATIC_MEMORY_ERROR!!!")
    }

    pub fn get_std_client(&self) -> &'static reqwest_middleware::ClientWithMiddleware {
        Self::global().clients.standard.get().expect("STATIC_MEMORY_ERROR!!!")
    }

    pub fn get_rel_client(&self) -> &'static reqwest_middleware::ClientWithMiddleware {
        Self::global().clients.relaxed.get().expect("STATIC_MEMORY_ERROR!!!")
    }

    pub(crate) fn get_dadata_header(&self) -> &'static HeaderMap {
        Self::global().headers.dadata_header.get().expect("STATIC_MEMORY_ERROR!!!")
    }

}


// pub(crate) fn make_client(
//     conn_timeout: Duration,
//     tot_timeout: Duration,
//     request_interval: Duration,
//     retries: u32,
// ) -> (reqwest::Client, ExponentialBackoff) {
    
//     let client= reqwest::Client::builder()
//         .connect_timeout(conn_timeout)
//         .timeout(tot_timeout)
//         .build()
//         .expect("FATAL: Failed to build reqwest::Client");

//     let policy = ExponentialBackoff::builder()
//         .jitter(reqwest_retry::Jitter::None)
//         .retry_bounds(request_interval, request_interval * 100)
//         .base(2)
//         .build_with_max_retries(retries);

//     (client, policy)
        
// }



pub(crate) fn make_client(
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
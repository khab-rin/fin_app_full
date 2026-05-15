use std::default;
use std::sync::OnceLock;
use std::time::Duration;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, CONTENT_TYPE, ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Deserializer, Serialize};

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
    pub(crate) client: OnceLock<Client>,
    #[serde(skip)]
    pub(crate) headers: Headers
}

#[derive(Deserialize, Debug)]
pub(crate) struct EmailSender {
    pub(crate) base_url: String,
    pub(crate) from: String,
    #[serde(skip, default)]
    pub(crate) api: String
}


#[derive(Deserialize, Debug, Default)]
pub(crate) struct DataBase {
    #[serde(skip, default)]
    pub(crate) database_url: String, 
}

#[derive(Deserialize, Debug)]
pub(crate) struct NetWork {
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) conn_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) tot_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) poll_intervals: Duration,
    pub(crate) connect_retrys: u8
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

#[derive(Default, Debug)]
pub(crate) struct Headers {
    pub(crate) dadata_header: OnceLock<HeaderMap>,
}

pub(crate) struct ApiState {
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
            
            let client = Client::builder()
                .connect_timeout(config.network.conn_timeout)
                .timeout(config.network.tot_timeout)
                .build()
                .expect("FATAL: Failed to build reqwest::Client");

            config.client.set(client).expect("STATIC_MEMORY_ERROR!!!");

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

    pub(crate) fn get_client(&self) -> &'static Client {
        Self::global().client.get().expect("STATIC_MEMORY_ERROR!!!")
    }

    pub(crate) fn get_dadata_header(&self) -> &'static HeaderMap {
        Self::global().headers.dadata_header.get().expect("STATIC_MEMORY_ERROR!!!")
    }

}
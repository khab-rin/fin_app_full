use std::sync::OnceLock;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, CONTENT_TYPE, ACCEPT, AUTHORIZATION};
use serde::Deserialize;

use shared_lib::service::auth_service::general::*;
use shared_lib::service::auth_service::back_api_state::*;
use shared_lib::make_header;

pub struct BackApiState {
    pub pool_fast: sqlx::PgPool,
    pub pool_long: sqlx::PgPool,
    pub config: &'static Config
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(skip)]
    pub data_base: DataBase,
    pub network: NetWork,
    pub postgresql_options: PostgresQlOptions,
    pub dadata: Dadata,
    pub smsru: SmsRu,
    pub email_sender: EmailSender,
    #[serde(skip)]
    pub crypto_servise: CryptoService,
    #[serde(skip)]
    pub clients: Clients,
    
    #[serde(skip)]
    pub headers: Headers
}

#[derive(Deserialize, Debug)]
pub struct PostgresQlOptions {
    pub fast_max_conn: u32,
    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub fast_timeout: Duration,
    pub long_max_conn: u32,
    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub long_timeout: Duration
}

#[derive(Deserialize, Debug)]
pub struct NetWork {
    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub inst_conn_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub inst_tot_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub inst_request_interval: Duration,
    pub inst_retries: u32,

    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub std_conn_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub std_tot_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub std_request_interval: Duration,
    pub std_retries: u32,

    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub inst_poll_intervals: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub std_poll_intervals: Duration,
    }

#[derive(Default, Debug)]
pub struct Clients {
    pub instant: OnceLock<reqwest_middleware::ClientWithMiddleware>,
    pub standard: OnceLock<reqwest_middleware::ClientWithMiddleware>,
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

    pub(crate) fn get_dadata_header(&self) -> &'static HeaderMap {
        Self::global().headers.dadata_header.get().expect("STATIC_MEMORY_ERROR!!!")
    }

}


pub(crate) async fn create_pool(base_url: &str, max_conn: &u32, timeout: &Duration) -> sqlx::PgPool {
    let mut attempts = 0;
    loop {
        attempts += 1;
        let pool_result = sqlx::postgres::PgPoolOptions::new()
            .max_connections(*max_conn)
            .acquire_timeout(*timeout)
            .connect(base_url)
            .await;

        match pool_result {
            Ok(p) => return p,
            Err(er) => {
                tracing::warn!("Попытка подключения к БД №{} не удалась", attempts);
                if attempts == 5 {
                    tracing::error!("ФАТАЛЬНАЯ ОШИБКА: Не удалось подключиться к PostgreSQL.");
                    panic!("Database connection failed: {}", er);
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

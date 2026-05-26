use std::sync::OnceLock;

use serde::Deserialize;
use reqwest::header::{CONTENT_TYPE, ACCEPT, HeaderMap};

use shared_lib::service::auth_service::general::*;
use shared_lib::service::auth_service::client_state::*;
use shared_lib::make_header;


pub struct ClientState {
    pub config: &'static Config,
    pub session: tokio::sync::Mutex<Option<ActiveSession>>
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(skip)]
    pub clients: Clients,
    #[serde(skip)]
    pub headers: Headers,
    #[serde(skip)]
    pub data_base: String,
    pub sqlite_options: SqliteOptions,
    pub network: NetWork,
}



impl Config {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<Config> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            dotenvy::dotenv().ok();
            let toml_str = include_str!("../config.toml");
            let mut config: Config = toml::from_str(toml_str)
                .expect("CONFIG MAPPING ERROR");

            let back_api_header = make_header!([
                CONTENT_TYPE => "application/json",
                ACCEPT => "application/json"
            ]);

            config.headers.back_api_header.set(back_api_header).expect("STATIC_MEMORY_ERROR!!!");

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

    pub(crate) fn back_api_header(&self) -> &'static HeaderMap {
        Self::global().headers.back_api_header.get().expect("STATIC_MEMORY_ERROR!!!")
    }
}

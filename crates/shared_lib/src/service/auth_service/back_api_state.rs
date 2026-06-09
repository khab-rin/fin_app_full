use std::sync::OnceLock;

use serde::Deserialize;
use reqwest::header::HeaderMap;

#[derive(Deserialize, Debug, Default)]
pub struct DataBase {
    #[serde(skip, default)]
    pub database_url: String, 
}

#[derive(Deserialize, Debug)]
pub struct Dadata {
    pub dadata_comp_url: String,
    pub dadata_adr_paid_url: String,
    #[serde(skip, default)]
    pub comp_free_api: String,
    #[serde(skip, default)]
    pub paid_api_key: String,
}

#[derive(Deserialize, Debug)]
pub struct SmsRu {
    pub call_add_url: String,
    pub get_stat_url: String,
    #[serde(skip, default)]
    pub api: String 
}

#[derive(Deserialize, Debug)]
pub struct EmailSender {
    pub base_url: String,
    pub from: String,
    #[serde(skip, default)]
    pub api: String
}


#[derive(Deserialize, Debug, Default)]
pub struct CryptoService {
    #[serde(skip, default)]
    pub url: String,
}

#[derive(Default, Debug)]
pub struct Headers {
    pub dadata_header: OnceLock<HeaderMap>,
}
use std::time::Duration;
use std::sync::OnceLock;

use serde::{Serialize, Deserialize};
use reqwest::header::HeaderMap;

use crate::primitives::frozen::implements::{BoxUuid, PersInn, CompInn, Kpp, Phone};
use crate::primitives::frozen::implements_base::String1_50;
use crate::sql_models::company::implements::Company;
use crate::sql_models::person::implements::Person;
use crate::sql_models::user::implements::User;
use crate::service::auth_service::general::time_parser;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLogInfo {
    pub pers_inn: PersInn,
    pub comp_inn: CompInn,
    pub kpp: Kpp, 
    pub token: BoxUuid
}

#[derive(Serialize, Deserialize, Default, ts_rs::TS)]
pub struct NickData{
    pub nick_names: Vec<String1_50>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TempInfo {
    pub file_hash: Option<String>,
    pub phone: Option<Phone>,
    pub nick: Option<String1_50>
}

#[derive(Debug, Clone)]
pub struct ActiveSession {
    pub session_user: SessionUser,
    pub local_db: sqlx::SqlitePool,
    pub token: BoxUuid
}

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




#[derive(Default, Debug)]
pub struct Headers {
    pub back_api_header: OnceLock<HeaderMap>
}

#[derive(Deserialize, Debug)]
pub struct SqliteOptions {
    pub max_connections: u32,
    #[serde(deserialize_with = "time_parser::duration_from_f64")]
    pub duration: Duration
}





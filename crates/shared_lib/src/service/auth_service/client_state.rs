use std::time::Duration;
use std::sync::OnceLock;

use serde::{Serialize, Deserialize};
use reqwest::header::HeaderMap;

use crate::Status;
use crate::primitives::frozen::implements::{BoxUuid, Inn, Kpp};
use crate::sql_models::company::implements::Company;
use crate::sql_models::person::implements::Person;
use crate::sql_models::user::implements::User;
use crate::service::auth_service::general::time_parser;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLogInfo {
    pub pers_inn: Inn,
    pub comp_inn: Inn,
    pub kpp: Kpp, 
    pub token: BoxUuid
}

pub struct ActiveSession {
    pub user: SessionUser,
    pub local_db: sqlx::SqlitePool,
    pub token: BoxUuid
}

#[derive(Serialize, Deserialize, Clone)]
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
    type Error = Status;
    fn try_from(dto: SessionUserDto) -> Result<Self, Self::Error> {
        Ok(Self { 
            user: serde_json::
                from_value(dto.user)
                .map_err(|_| Status::MappingError)?,
            
            person: serde_json::
                from_value(dto.person)
                .map_err(|_| Status::MappingError)?,
            
            company: serde_json::
                from_value(dto.company)
                .map_err(|_| Status::MappingError)?
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
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub duration: Duration
}





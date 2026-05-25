use serde::{Serialize, Deserialize};

use crate::Status;
use crate::primitives::frozen::implements::{BoxUuid, Inn, Kpp};
use crate::sql_models::company::implements::Company;
use crate::sql_models::person::implements::Person;
use crate::sql_models::user::implements::User;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLogInfo {
    pub label: String,
    pub pers_inn: Inn,
    pub comp_inn: Inn,
    pub kpp: Kpp, 
    pub token: BoxUuid
}

pub struct ClientState {
    pub client: reqwest::Client,
    pub api_url: String,
    pub app_name: String,
    pub app_path: std::path::PathBuf,
    pub session: tokio::sync::Mutex<Option<ActiveSession>>
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

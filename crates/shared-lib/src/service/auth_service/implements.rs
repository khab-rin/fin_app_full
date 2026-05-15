use serde::{Deserialize, Serialize};


use crate::Status;
use crate::sql_models::company_models::implements::Company;
use crate::sql_models::person_models::implements::Person;
use crate::sql_models::user_models::implements::User;
use crate::primitives::frozen::implements::{BoxUuid, Email, Phone};
use crate::primitives::frozen::implements::{Inn, Kpp};

#[derive(Serialize, Deserialize)]
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
                .map_err(|_| Status::UserWrongMapping)?,
            
            person: serde_json::
                from_value(dto.person)
                .map_err(|_| Status::PersonWrongMapping)?,
            
            company: serde_json::
                from_value(dto.company)
                .map_err(|_| Status::CompanyWrongMapping)?
         })
    }
}

pub struct AppState {
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

#[derive(Serialize, Deserialize)]
pub enum RegisterResponse {
    Success(Box<SessionUserToken>),
    Verify(VerifyData)
}

#[derive(Serialize, Deserialize)]
pub struct SessionUserToken {
    pub user: SessionUser,
    pub token: BoxUuid
}

#[derive(Serialize, Deserialize)]
pub struct VerifyData {
    pub device_id: BoxUuid,
    pub method: VerifyMethod
}

#[derive(Serialize, Deserialize)]
pub enum VerifyMethod {
    CallIn { phone: Phone, external_id: String },
    NeedPassword {},
    Unpossible {status: Status},
    WarnConnectTry {token: BoxUuid},
    NeedRegistrtion {},
    TryLater {},
    WrongPassword {},

}


#[derive(Serialize, Deserialize, Debug)]
pub struct RestoreByTokenRequest {
    pub token: BoxUuid,
    pub device_id: BoxUuid
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestoreByAuthDataRequest {
    pub auth_data: AuthData,
    pub device_id: BoxUuid
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthData {
    pub pers_inn: Inn,
    pub password: String,
    pub comp_inn: Inn,
    pub kpp: Kpp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthCheckPassword {
    pub user_id: BoxUuid, 
    pub phone: Phone,
    pub password_hash: String,
    pub token: Option<BoxUuid>
}

#[derive(Deserialize, Debug)]
pub struct WarnEmailData {
    pub email: Email,
    pub pers_inn: Inn,
    pub comp_inn: Inn,
    pub kpp: Kpp
}

#[derive(Deserialize, Debug)]
pub struct RestoreByTelCallRequest {
    pub device_id: BoxUuid,
    pub external_id: String 
}


#[derive(Deserialize, Debug)]
pub enum SmsRuResponseTextCode {
    #[serde(rename = "400")]
    Polling,
    #[serde(rename = "401")]
    SuccessConfirmed,
    #[serde(rename = "402")]
    TimeOut,
    UnknownCode
}

#[derive(Deserialize, Debug)]
pub struct SmsruCallResponse {
    pub status: String,
    pub status_code: i32,
    pub check_id: Option<String>,
    pub call_phone: Option<Phone>,
    pub call_phone_pretty: Option<String>,
    pub call_phone_html: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct SmsruGetResResponse {
    pub status: String,
    pub status_code: i32,
    pub check_status: Option<SmsRuResponseTextCode>,
    pub check_status_text: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationRequestDto {
    person: Person,
    comp_inn: Inn,
    kpp: Kpp,
    
    #[serde(with = "serde_bytes")]
    pub document: Vec<u8>,  
    
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>, 
}

use serde::{Deserialize, Serialize};


use crate::Status;
use crate::sql_models::person::implements::Person;
use crate::primitives::frozen::implements::{BoxUuid, Email, Inn, Kpp, Phone, Snils};
use crate::primitives::composite::implements::Fio;
use crate::service::auth_service::client_state::SessionUser;



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
    MissedFile {},
    WrongSignFile {},
    WrongPerson {},
    UserAlreadyExists {}
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
pub struct RegistrationRequest {
    pub person: Person,
    pub comp_inn: Inn,
    pub kpp: Kpp,
    pub password: String,
    pub device_id: BoxUuid,
    pub phone: Phone,
    pub email: Email,
    pub doc_hash: String,
    #[serde(with = "serde_bytes")]
    pub document: Vec<u8>,  
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>, 
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoVerifyRequest {
    pub document: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoVerifyPersonResponse {
    pub is_signed: bool,
    pub snils: Option<Snils>,
    pub inn: Option<Inn>,
    pub fio: Option<Fio>
}

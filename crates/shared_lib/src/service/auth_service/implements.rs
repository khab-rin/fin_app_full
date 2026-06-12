use serde::{Deserialize, Serialize};

use crate::sql_models::person::implements::Person;
use crate::primitives::frozen::implements_base::String1_50;
use crate::primitives::frozen::implements::{BoxUuid, CompInn, Email, FirstName, Kpp, MidName, Password, PersInn, Phone, Snils, SurName};
use crate::service::auth_service::client_state::SessionUser;


#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub struct SessionUserToken {
    pub user: SessionUser,
    pub token: BoxUuid
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum AuthStep {
    Loading { text: TextInfo },
    NickName { text: TextInfo },
    NeedPassword {text: TextInfo},
    NeedRegistration {text: TextInfo},
    CallIn { phone: Phone, external_id: String, text: TextInfo },
    CallInWaiting { text: TextInfo },
    SuccessFull { session_user_token: Box<SessionUserToken> },
    SuccessShort {},
    TryLater { text: TextInfo },
    TokenDevicePairMiss { text: TextInfo }
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS,)]
pub enum TextInfo {
    #[serde(rename = "Пользователь не найден, требуется пройти регистрацию")]
    MissUserNeedRegistration,

    #[serde(rename = "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение")]
    ClientApiSystemError,

    #[serde(rename = "Ошибка при запросе в интернет, проверьте подключение к сети")]
    ClientApiQueryError,

    #[serde(rename = "Ошибка в работе серверной части приложения, попробуйте авторизоваться позже, либо сделайте запрос в техподдержку")]
    BackApiError,

    #[serde(rename = "Возможная попытка несанкцианированного доступа")]
    IllegalAccess,

    #[serde(rename = "Пароль к связке входных параметров неверный")]
    WrongPassword,

    #[serde(rename = "Некорректные файлы подписи, пройдите процесс заново")]
    WrongSignFile,

    #[serde(rename = "Пользователь с данными входными данными уже существует, введите пароль")]
    UserAlreadyExists,

    #[serde(rename = "Выявлены различия в файле заявлении и подписанном файле, пройдите регистрацию заново")]
    MissedFile,

    #[serde(rename = "Выявлены различия в данных пользователя и данных владельца подписи, пройдите регистрацию заново")]
    WrongPerson,
    
    #[serde(rename = "Пользователь не найден на устройстве, требуется авторизоваться по паролю или пройти регистрацию")]
    NewUserInSystem,

    #[serde(rename = "Вход пользователя с нового устройста, для подтверждения позвоните с указанного при регистрации номера по указанному номеру в течение 5 минут, затем нажмите далее. Звонок бесплатный и скинется после первого гудка")]
    CallIn,

    #[serde(rename = "Звонок с указанного телефона по указанному номеру не был получен, время истекло, повторите процесс заново")]
    CallInnTimeOut,

    #[serde(rename = "Звонок по указанному номеру не был осуществлен, позвоните по этому номеру")]
    CallInWaiting,

    #[serde(rename = "Выберите из списка нужного пользователя, в случае отсутствия авторизуйтесь через пароль, либо зарегистрируйтесь")]
    InitInfo,

    #[serde(rename = "Страница загружается, подождите пожалуйста. В случае зависания попробуйте обновить или перезагрузить приложение")]
    LoadingInfo,

    #[serde(rename = "")]
    Nothing,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TokenDeviceData {
    pub token: BoxUuid,
    pub device_id: BoxUuid
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub struct PasswordDataClientShort {
    pub nick: String1_50,
    pub password: String,
    pub pers_inn: PersInn,
    pub comp_inn: CompInn,
    pub kpp: Kpp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordDataClient {
    pub password: String,
    pub device_id: BoxUuid,
    pub pers_inn: PersInn,
    pub comp_inn: CompInn,
    pub kpp: Kpp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordDataBackApi {
    pub user_id: BoxUuid, 
    pub phone: Phone,
    pub password_hash: String,
    pub token: Option<BoxUuid>
}

#[derive(Deserialize, Debug)]
pub struct WarnEmailData {
    pub email: Email,
    pub pers_inn: PersInn,
    pub comp_inn: CompInn,
    pub kpp: Kpp
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub struct ExternalDeviceData {
    pub external_id: String, 
    pub device_id: BoxUuid,
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
    pub check_status: Option<i32>,
    pub check_status_text: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub struct SvelteRegistrationData {
    pub nick: String1_50,
    pub sur_name: SurName,
    pub first_name: FirstName,
    pub mid_name: MidName,
    pub pers_inn: PersInn,
    pub snils: Snils,
    pub comp_inn: CompInn,
    pub kpp: Kpp,
    pub password: Password,
    pub phone: Phone,
    pub email: Email,
    pub document_path: String,  
    pub signature_path: String, 
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub struct IngoingData {
    pub sur_name: SurName,
    pub first_name: FirstName,
    pub mid_name: Option<MidName>,
    pub pers_inn: PersInn,
    pub snils: Snils,
    pub comp_inn: CompInn,
    pub kpp: Kpp,
    pub phone: Phone,
    pub email: Email,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationData {
    pub person: Person,
    pub comp_inn: CompInn,
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
pub struct CryptoVerifyData {
    pub document: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoServiceResponse {
    pub is_signed: bool,
    pub text: String
}



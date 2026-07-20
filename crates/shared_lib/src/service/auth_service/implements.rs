use serde::{Deserialize, Serialize};

use crate::primitives::frozen::implements::{BoxUuid, CompInn, Email, FirstName, Kpp, MidName, Password, PersInn, Phone, Snils, SurName};
use crate::service::auth_service::client_state::SessionUser;


#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub struct SessionUserToken {
    pub user: SessionUser,
    pub token: BoxUuid
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum AuthStep {
    CallIn { phone: Phone, external_id: String, text: AuthInfo },
    CallInWaiting { text: AuthInfo },

    Loading { text: AuthInfo },
    
    NickName { text: AuthInfo },

    Password {text: AuthInfo},
    
    RegisterStep1 {text: AuthInfo},
    RegisterStep1Duplicate {
        sur_name: String,
        first_name: String,
        mid_name: String,
        pers_inn: String,
        snils: String,
        comp_inn: String,
        kpp: String,
        phone: String,
        email: String,
        text: AuthInfo
    },
    RegisterStep1Success {
        doc_name: String,
        doc_file: Vec<u8>,
        json_name: String,  
        json_file: Vec<u8>,
        text: AuthInfo
    },
    RegisterStep2 {text: AuthInfo},
    
    SuccessFull { session_user_token: Box<SessionUserToken> },
    SuccessShort {},
    
    TokenDevicePairMiss { text: AuthInfo },
    TryLater { text: AuthInfo },  
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS,)]
pub enum AuthInfo {
    #[serde(rename = "Ошибка в работе серверной части приложения, попробуйте авторизоваться позже, либо сделайте запрос в техподдержку")]
    BackApiError,

    #[serde(rename = "Вход пользователя с нового устройста, для подтверждения позвоните с указанного при регистрации номера по указанному номеру в течение 5 минут, затем нажмите далее. Звонок бесплатный и скинется после первого гудка")]
    CallIn,

    #[serde(rename = "Звонок с указанного телефона по указанному номеру не был получен, время истекло, повторите процесс заново")]
    CallInnTimeOut,

    #[serde(rename = "Звонок по указанному номеру не был осуществлен, позвоните по этому номеру")]
    CallInWaiting,

    #[serde(rename = "Ошибка при запросе в интернет, проверьте подключение к сети")]
    ClientApiQueryError,

    #[serde(rename = "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение")]
    ClientApiSystemError,

    #[serde(rename = "Выберите из списка нужного пользователя, в случае отсутствия авторизуйтесь через пароль, либо зарегистрируйтесь")]
    InitInfo,

    #[serde(rename = "Возможная попытка несанкцианированного доступа")]
    IllegalAccess,

    #[serde(rename = "Страница загружается, подождите пожалуйста. В случае зависания попробуйте обновить или перезагрузить приложение")]
    LoadingInfo,

    #[serde(rename = "Выявлены различия в файле заявлении и подписанном файле, пройдите регистрацию заново")]
    MissedFile,

    #[serde(rename = "Пользователь не существует, либо не прошел регистрацию, либо не синхронизирован. Попробуйте войти по паролю, либо пройдите регистрацию")]
    MissToken,

    #[serde(rename = "Пользователь не найден, требуется пройти регистрацию")]
    MissUserNeedRegistration,

    #[serde(rename = "Пользователь не найден на устройстве, требуется авторизоваться по паролю или пройти регистрацию")]
    NewUserInSystem,

    #[serde(rename = "Заполните поля регистрации строго как в документах")]
    RegisterStep1,

    #[serde(rename = "Сохраните файлы, ознакомьтесь с заявлением в формате doc и подпишите ЭЦП физ лица пользователя документ в формате json")]
    RegisterStep1Success,

    #[serde(rename = "Укажите путь до xml файла заявления и путь до файла открепленной подписи. Подпись должна быть для указанного файла xml")]
    RegisterStep2,

    #[serde(rename = "Пользователь с данными входными данными уже существует, авторизуйтей по паролю, либо пройдите процедуру восстановления пароля по эл. почте/номеру телефона")]
    UserAlreadyExists,

    #[serde(rename = "Пароль к связке входных параметров неверный")]
    WrongPassword,

    #[serde(rename = "Выявлены различия в данных пользователя и данных владельца подписи, пройдите регистрацию заново")]
    WrongPerson,

    #[serde(rename = "Некорректные файлы подписи, пройдите процесс заново")]
    WrongSignFile,

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


#[derive(Serialize, Deserialize, Debug, Clone, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename_all = "camelCase")]
pub struct RegInitData {
    pub sur_name: SurName,
    pub first_name: FirstName,
    pub mid_name: Option<MidName>,
    pub pers_inn: PersInn,
    pub snils: Snils,
    pub comp_inn: CompInn,
    pub kpp: Kpp,
    pub phone: Phone,
    pub email: Email,
    pub password: Password,
    pub device_id: BoxUuid
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RegFilesData {
    pub json_file: Vec<u8>,  
    pub sign_file: Vec<u8>, 
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename_all = "camelCase")]
pub struct RegFilesPathData {
    pub json_path: String,  
    pub sign_path: String, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonSignCheckResult {
    pub is_signed: bool,
    pub text: String
}



#[derive(Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename_all = "camelCase")]
pub enum AuthTs {
    AuthInfo(AuthInfo),
    AuthStep(AuthStep),

    ExternalDeviceData(ExternalDeviceData),

    NickData(crate::service::auth_service::client_state::NickData),

    RegFilesPathData(RegFilesPathData),
    RegInitData(RegInitData),
    PasswordDataClientShort(PasswordDataClientShort),
    
    
}


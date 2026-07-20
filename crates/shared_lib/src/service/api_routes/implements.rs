pub enum ApiRoutes {
    AutoAddCompany,
    AuthRegisterStep1,
    AuthRegisterStep2,
    AuthRestorePassword,
    AuthRestoreTellCall,
    AuthRestoreToken,
    MchdLend,
    SqlPersonGetByInn
}

impl ApiRoutes {
    pub fn get_path(&self) -> &str {
        match self {
            Self::AutoAddCompany => "/api/companys/auto-add",
            Self::AuthRegisterStep1 => "/api/auth/register_step1",
            Self::AuthRegisterStep2 => "/api/auth/register_step2",
            Self::AuthRestorePassword => "/api/auth/restore_by_password",
            Self::AuthRestoreTellCall => "/api/auth/restore_by_tel_call",
            Self::AuthRestoreToken => "/api/auth/restore_by_token",
            Self::MchdLend => "/api/mchd/lend_mchd_for_register",
            Self::SqlPersonGetByInn => "/api/sql/person_get_by_inn"
        }
    }
}

pub enum CryptoApiRoutes {
    CryptoVerifyPerson
}

impl CryptoApiRoutes {
    pub fn get_path(&self) -> &str {
        match self {
            Self::CryptoVerifyPerson => "/crypapi/verify/person"
        }
    }
}
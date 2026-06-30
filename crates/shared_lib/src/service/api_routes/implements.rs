pub enum ApiRoutes {
    AutoAddCompany,
    AuthRestoreToken,
    AuthRestorePassword,
    AuthRegister,
    AuthRestoreTellCall,
    MchdLend,
    SqlPersonGetByInn
}

impl ApiRoutes {
    pub fn get_path(&self) -> &str {
        match self {
            Self::AutoAddCompany => "/api/companys/auto-add",
            Self::AuthRestoreToken => "/api/auth/restore_by_token",
            Self::AuthRestorePassword => "/api/auth/restore_by_password",
            Self::AuthRegister => "/api/auth/register_user",
            Self::AuthRestoreTellCall => "/api/auth/restore_by_tel_call",
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
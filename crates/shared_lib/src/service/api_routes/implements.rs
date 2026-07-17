pub enum ApiRoutes {
    AutoAddCompany,
    AuthInitUser,
    AuthRegister,
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
            Self::AuthInitUser => "/api/auth/init_user",
            Self::AuthRegister => "/api/auth/register_user",
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
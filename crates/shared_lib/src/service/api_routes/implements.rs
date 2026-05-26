use crate::service::api_routes::implements::ApiRoutes::AuthRestorePassword;

pub enum ApiRoutes {
    AutoAddCompany,
    AuthRestoreToken,
    AuthRestorePassword,
    AuthRegister,
    
}

impl ApiRoutes {
    pub fn get_path(&self) -> &str {
        match self {
            Self::AutoAddCompany => "/api/companys/auto-add",
            Self::AuthRestoreToken => "/api/auth/restore_by_token",
            Self::AuthRestorePassword => "/api/auth/restore_by_password",
            Self::AuthRegister => "/api/auth/register_user"
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
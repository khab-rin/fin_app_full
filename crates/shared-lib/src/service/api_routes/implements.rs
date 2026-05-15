pub enum ApiRoutes {
    Register,
    Login,
    AuthRestoreToken,
    AutoAddCompany
}

impl ApiRoutes {
    pub fn get_path(&self) -> &str {
        match self {
            Self::Register => "/api/auth/register",
            Self::Login => "/api/auth/login",
            Self::AuthRestoreToken => "/api/auth/RestoreToken",
            Self::AutoAddCompany => "/api/companys/auto-add"
        }
    }
}
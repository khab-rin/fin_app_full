pub enum ApiRoutes {
    AuthRegisterStep1,
    AuthRegisterStep2,
    AuthRestorePassword,
    AuthRestoreTellCall,
    AuthRestoreToken,
    MchdLend,
    MchdShowPowers,
    SqlComppanysAddByInnKpp,
    SqlContractAddNew,
	SqlOperationsAddMany,
    SqlPersonGetByInn
}

impl ApiRoutes {
    pub fn get_path(&self) -> &str {
        match self {
            
            Self::AuthRegisterStep1 => "/api/auth/register_step1",
            Self::AuthRegisterStep2 => "/api/auth/register_step2",
            Self::AuthRestorePassword => "/api/auth/restore_by_password",
            Self::AuthRestoreTellCall => "/api/auth/restore_by_tel_call",
            Self::AuthRestoreToken => "/api/auth/restore_by_token",
            Self::MchdLend => "/api/mchd/lend_mchd_for_register",
            Self::MchdShowPowers => "/api/mchd/lend_show_powers",
            Self::SqlComppanysAddByInnKpp => "/api/companys/add_by_inn_kpp",
            Self::SqlContractAddNew => "/api/contracts/add_new",
			Self::SqlOperationsAddMany => "/api/operations/add_many",
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
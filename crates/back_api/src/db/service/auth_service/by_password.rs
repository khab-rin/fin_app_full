use std::sync::Arc;

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};

use shared_lib::{ProcessError, Status};
use shared_lib::service::auth_service::implements::{
    PasswordDataBackApi, 
    AuthStep
};
use shared_lib::service::auth_service::implements::{
    PasswordDataClient,
    TokenDeviceData,
    AuthInfo
};

use crate::db::sql_queries::users::get::auth_check_passw_by_authdata::get_restore_password_data;
use crate::db::sql_queries::call_cf::set::new_cf::new_cf;
use crate::db::service::auth_service::by_device_token::restore_session_by_token;
use crate::db::service::auth_service::smsru_phone_query::smsru_get_phone;
use crate::config::BackApiState;

pub(crate) async fn restore_session_by_passord(
    state: &Arc<BackApiState>,
    data: &PasswordDataClient
) -> Result<AuthStep, Status> {

    let failed_result = Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});

    let auth_check_password_option = match get_restore_password_data(state, data).await {
        Ok(opt) => opt,
        Err(err) => { 
            err.process_err(err, "");
            return failed_result; 
        }
    };

    let auth_check_password = match auth_check_password_option {
        Some(a) => a,
        None => {
            return Ok(AuthStep::RegisterStep1 {text: AuthInfo::MissUserNeedRegistration});
        }
    };

    let PasswordDataBackApi { 
        user_id, 
        phone, 
        password_hash, 
        token } = auth_check_password;
    
    let server_parsed_hash = match PasswordHash::new(&password_hash) {
        Ok(hash) => hash,
        Err(err) => {
            err.process_err(Status::SystemErr, "");
            return failed_result;
        }
    };

    match Argon2::default().verify_password(data.password.as_bytes(), &server_parsed_hash) {
        Ok(_) => {}
        Err(err) => {
            err.process_err(Status::SystemErr, "");
            return Ok(AuthStep::Password {text: AuthInfo::WrongPassword});
        }  
    }
    
    if let Some(t) = token {
        let token_device_data = TokenDeviceData {
            token: t,
            device_id: data.device_id.clone()
        };
        return restore_session_by_token(state, &token_device_data).await;
    } 

    let (external_id, call_phone) = match smsru_get_phone(state, &phone).await {
        Ok(res) => res,
        Err(err) => {
            err.process_err(err, "");
            return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
        }
    };

    match new_cf(state, &user_id, &data.device_id, &external_id).await {
        Ok(true) => {
            Ok(AuthStep::CallIn { phone: call_phone, external_id, text: AuthInfo::CallIn })
        }
        Ok(false) => {
            Status::Tech.process_err(Status::SystemLogicErr, "");
            Ok(AuthStep::TryLater {text: AuthInfo::BackApiError})
        }
        Err(err) => {
            err.process_err(Status::SystemLogicErr, "");
            Ok(AuthStep::TryLater {text: AuthInfo::BackApiError})
        } 
    }
    
}
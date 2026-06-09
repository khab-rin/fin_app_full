use std::sync::Arc;

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    PasswordDataBackApi, 
    AuthStep
};
use shared_lib::service::auth_service::implements::{
    PasswordDataClient,
    TokenDeviceData,
    TextInfo
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

    let failed_data = data;

    let auth_check_password_option = match get_restore_password_data(state, data).await {
        Ok(opt) => opt,
        Err(err) => {
            tracing::error!(
                err = ?err,
                failed_data = ?failed_data,
                "FUN restore_user_by_authdata FAILED BY get_auth_password_check FUN"
            );
            return Ok(AuthStep::TryLater {text: TextInfo::BackApiError});
        }
    };

    let auth_check_password = match auth_check_password_option {
        Some(a) => a,
        None => {
            return Ok(AuthStep::NeedRegistration {text: TextInfo::MissUserNeedRegistration});
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
            tracing::error!(
                tech_err = ?err,
                user = %user_id,
                "WRONG PASSWORD DATA IN SQL Users"
            );
            return Ok(AuthStep::TryLater {text: TextInfo::BackApiError});
        }
    };

    match Argon2::default().verify_password(data.password.as_bytes(), &server_parsed_hash) {
        Ok(_) => {}
        Err(err) => {
            tracing::warn!(
                tech_err = ?err,
                user_id = %user_id,
                "USER_SENDED_WRONG_PASSWORD!!!"
            );
            return Ok(AuthStep::NeedPassword {text: TextInfo::WrongPassword});
            }  
        };
    
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
            tracing::error!(
                err = ?err,
                user_id = %user_id,
                "FUN restore_user_by_authdata FAILED ON GETTING CALL BACK PHONE BY FUN smsru_get_phone"
            );
            return Ok(AuthStep::TryLater {text: TextInfo::BackApiError});
        }
    };

    match new_cf(state, &user_id, &data.device_id, &external_id).await {
        Ok(true) => {
            Ok(AuthStep::CallIn { phone: call_phone, external_id, text: TextInfo::CallIn })
        }
        Ok(false) => {
            tracing::error!(
                user_id = %user_id,
                "WRONG LOGIC IN FUN new_cf AND SQL QUERYS" 
            );
            Ok(AuthStep::TryLater {text: TextInfo::BackApiError})
        }
        Err(err) => {
            tracing::error!(
                user_id = %user_id,
                err = ?err,
                "WRONG LOGIC IN FUN new_cf AND SQL QUERYS" 
            );
            Ok(AuthStep::TryLater {text: TextInfo::BackApiError})
        } 
    }
    
}
use std::sync::Arc;

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{AuthCheckPassword, VerifyData, VerifyMethod};
use shared_lib::service::auth_service::implements::{
    RestoreByAuthDataRequest,
    RegisterResponse,
    RestoreByTokenRequest
};

use crate::db::sql_queries::users::get::auth_check_passw_by_authdata::get_auth_password_check;
use crate::db::sql_queries::call_cf::set::new_cf::new_cf;
use crate::db::service::auth_service::by_device_token::get_user;
use crate::db::service::auth_service::smsru_phone_query::smsru_get_phone;
use crate::config::BackApiState;

pub(crate) async fn restore_user_by_authdata(
    state: &Arc<BackApiState>,
    data: &RestoreByAuthDataRequest
) -> Result<RegisterResponse, Status> {

    let failed_result = Ok(RegisterResponse::
        Verify(VerifyData { 
            device_id: data.device_id.clone(), 
            method: VerifyMethod::TryLater {} 
        }));

    let failed_data = &data.auth_data;

    let auth_vec = match get_auth_password_check(state, data).await {
        Ok(vec) => vec,
        Err(err) => {
            tracing::error!(
                err = ?err,
                failed_data = ?failed_data,
                "Fun restore_user_by_authdata failed on users query"
            );
            return failed_result;
        }
    };

    let RestoreByAuthDataRequest { auth_data, device_id } = data;
    let password = &auth_data.password;

    let AuthCheckPassword { 
        user_id, 
        phone, 
        password_hash, 
        token } = match auth_vec.into_iter().next() {
            Some(elem) => elem,
            None => {
                tracing::error!(
                    failed_data = ?failed_data,
                    "WRONG LOGIC IN FUN restore_user_by_authdata AND SQL QUERYS"
                );
                return failed_result;
            }
        };
    
    let server_parsed_hash = match PasswordHash::new(&password_hash) {
        Ok(hash) => hash,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                user = %user_id,
                "WRONG PASSWORD DATA IN SQL Users"
            );
            return failed_result;
        }
    };

    match Argon2::default().verify_password(password.as_bytes(), &server_parsed_hash) {
        Ok(_) => {}
        Err(err) => {
            tracing::warn!(
                tech_err = ?err,
                user_id = %user_id,
                "USER_SENDED_WRONG_PASSWORD!!!"
            );
            let verify = VerifyData {
                device_id: device_id.clone(),
                method: VerifyMethod::WrongPassword {}
            };
            return Ok(RegisterResponse::Verify(verify));
            }
        };
    
    if let Some(t) = token {
        let restore_by_token_request = RestoreByTokenRequest {
            token: t,
            device_id: device_id.clone()
        };
        return get_user(state, &restore_by_token_request).await;
    } 

    let (external_id, call_phone) = match smsru_get_phone(state, &phone).await {
        Ok(res) => res,
        Err(err) => {
            tracing::error!(
                err = ?err,
                user_id = %user_id,
                "FUN restore_user_by_authdata FAILED ON GETTING CALL BACK PHONE BY FUN smsru_get_phone"
            );
            return failed_result;
        }
    };

    match new_cf(state, &user_id, device_id, &external_id).await {
        Ok(true) => {
            let verify = VerifyData {
                device_id: device_id.clone(),
                method: VerifyMethod::CallIn { phone: call_phone, external_id }
            };
            Ok(RegisterResponse::Verify(verify))
        }
        Ok(false) => {
            tracing::error!(
                user_id = %user_id,
                "WRONG LOGIC IN FUN new_cf AND SQL QUERYS" 
            );
            failed_result
        }
        Err(err) => {
            tracing::error!(
                user_id = %user_id,
                err = ?err,
                "WRONG LOGIC IN FUN new_cf AND SQL QUERYS" 
            );
            failed_result
        } 
    }
    
}
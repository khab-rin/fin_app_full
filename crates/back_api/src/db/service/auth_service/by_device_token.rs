use std::sync::Arc;

use shared_lib::Status;
use shared_lib::service::auth_service::client_state::SessionUser;
use shared_lib::service::auth_service::implements::{RegisterResponse, RestoreByTokenRequest, SessionUserToken, VerifyData, VerifyMethod};


use crate::config::BackApiState;
use crate::db::service::auth_service::delete_token::delete_warn_token_device;
use crate::db::sql_queries::users::get::by_device_token::get_user_by_device_token;


pub(crate) async fn get_user(
    state: &Arc<BackApiState>,
    payload: &RestoreByTokenRequest
) -> Result<RegisterResponse, Status> {

    let session_users_dto = get_user_by_device_token(state, &payload)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::BackAuthGetSeeionUserQuery
            )
        }).map_err(|_| Status::BackAuthGetSeeionUserQuery)?;
        
    let session_user_dto = session_users_dto
        .into_iter()
        .next();

    if session_user_dto.is_none() {
        if let Err(err) = delete_warn_token_device(state, &payload).await {
            tracing::warn!(
                error = ?err,
                "DELETING_TOKENS_ERROR_DDURING_AUTH!!!"
            )
        };

        let res = VerifyData { 
            device_id: payload.device_id.clone(), 
            method: VerifyMethod::WarnConnectTry { token: payload.token.clone() } };
        return Ok(RegisterResponse::Verify(res));
    }

    let user: SessionUser  = session_user_dto
        .ok_or(Status::BackAuthRestoreSessionLogicErr)?
        .try_into()
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::BackAuthUserTryFromDto
            )
        }).map_err(|_| Status::BackAuthUserTryFromDto)?;
    
    let RestoreByTokenRequest { token, ..} = payload;
    
    let user_token = SessionUserToken {user, token: token.clone()};

    let response = RegisterResponse::Success(Box::new(user_token));
    
    Ok(response)
    
}
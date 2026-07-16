use std::sync::Arc;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    TokenDeviceData, 
    SessionUserToken, 
    AuthStep,
    AuthInfo
};

use crate::config::BackApiState;
use crate::db::service::auth_service::delete_token::delete_warn_token_device;
use crate::db::sql_queries::users::get::session_user_by_device_token::get_user_by_device_token;

pub(crate) async fn restore_session_by_token(
    state: &Arc<BackApiState>,
    payload: &TokenDeviceData 
) -> Result<AuthStep, Status> {

    let session_user_option = match get_user_by_device_token(state, payload).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                err = ?err,
                failed_data = ?payload,
                "FUN get_user FAILED BY FUN get_user_by_device_token"
            );
            return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
        }
    };

    let session_user = match session_user_option {
        Some(s_u) => s_u,
        None => {
            match delete_warn_token_device(state, payload).await {
                Ok(_) => {},
                Err(err) => {
                    tracing::warn!(
                        error = ?err,
                        "DELETING_TOKENS_ERROR_DDURING_AUTH!!!"
                    )
                }
            };
            return Ok(AuthStep::TokenDevicePairMiss { 
                text: AuthInfo::IllegalAccess 
            });
        }
    };

    let TokenDeviceData { token, ..} = payload;
    
    let user_token = SessionUserToken {user: session_user, token: token.clone()};

    let result = AuthStep::SuccessFull { session_user_token: Box::new(user_token) };
    
    Ok(result)
    
}
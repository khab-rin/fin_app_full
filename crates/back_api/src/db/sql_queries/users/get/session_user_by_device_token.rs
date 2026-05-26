use shared_lib::Status;
use shared_lib::service::auth_service::implements::TokenDeviceData;
use shared_lib::service::auth_service::client_state::{SessionUser, SessionUserDto};

use crate::config::BackApiState;

pub(crate) async fn get_user_by_device_token(
    state: &BackApiState,
    payload: &TokenDeviceData
    
) -> Result<Option<SessionUser>, Status> {

    let session_users_dto_opt = match sqlx::
        query_file_as!(
            SessionUserDto,
            "src/db/sql_queries/users/get/session_user_by_device_token.sql",
            payload.device_id.as_ref(),
            payload.token.as_ref()
        ).fetch_optional(&state.pool)
        .await {
            Ok(o) => o,
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::SqlQueryWrongLogic,
                    "FUN get_user_by_device_token FAILED BY SQL QUERY"
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        };

    
    let session_user_dto = match session_users_dto_opt {
        Some(s_u) => s_u,
        None => return Ok(None)
    };

    match session_user_dto.try_into() {
        Ok(session_user) => return Ok(Some(session_user)),
        Err(err) => {
            tracing::error!(
                err = ?err,
                "FUN get_user_by_device_token FAILED BY MAPPING SessionUser"
            );
            return Err(err);
        }
    }
    
}
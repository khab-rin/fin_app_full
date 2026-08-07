use shared_lib::{Status, ProcessError};
use shared_lib::service::auth_service::implements::TokenDeviceData;
use shared_lib::service::auth_service::general::{SessionUser, SessionUserDto};

use crate::config::BackApiState;

pub(crate) async fn get_user_by_device_token(
    state: &BackApiState,
    payload: &TokenDeviceData
    
) -> Result<Option<SessionUser>, Status> {

    let session_users_dto_opt = sqlx::query_file_as!(
            SessionUserDto,
            "src/db/sql_queries/users/get/session_user_by_device_token.sql",
            payload.device_id.as_ref(),
            payload.token.as_ref()
        ).fetch_optional(&state.pool_fast)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;
    
    
    let session_user_dto = match session_users_dto_opt {
        Some(s_u) => s_u,
        None => return Ok(None)
    };

    let session_user = session_user_dto
        .try_into()
        .map_err(|err:serde_json::Error| err.process_err(Status::MappingError, ""))?;

    Ok(Some(session_user))
    
}
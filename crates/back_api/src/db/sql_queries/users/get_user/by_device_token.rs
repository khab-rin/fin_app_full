use shared_lib::Status;
use shared_lib::service::auth_service::implements::{SessionUserDto, RestoreByTokenRequest};

use crate::config::ApiState;

pub(crate) async fn get_user_by_device_token(
    state: &ApiState,
    payload: &RestoreByTokenRequest
) -> Result<Vec<SessionUserDto>, Status> {
        let session_users_dto = sqlx::
            query_file_as!(
                SessionUserDto,
                "src/db/sql_queries/users/get_user/by_device_token.sql",
                payload.device_id.as_ref(),
                payload.token.as_ref()
            ).fetch_all(&state.pool)
            .await
            .inspect_err(|err| {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::BackAuthSqlGetUserQueryLogic
                )
            }).map_err(|_| Status::BackAuthSqlGetUserQueryLogic)?;

    Ok(session_users_dto)
}
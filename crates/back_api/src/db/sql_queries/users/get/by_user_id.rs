use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;
use shared_lib::service::auth_service::client_state::{SessionUserDto, SessionUser};

use crate::config::BackApiState;

pub(crate) async fn get_user_by_user_id(
    state: &Arc<BackApiState>,
    user_id: &BoxUuid
) -> Result<SessionUser, Status> {

    let session_user_dto: SessionUserDto = match sqlx::
        query_file_as!(
            SessionUserDto,
            "src/db/sql_queries/users/get/by_user_id.sql",
            user_id.as_ref()
        ).fetch_one(&state.pool_fast).await {
            Ok(dto) => dto,
            Err(err) => {
                tracing::error!(
                    user_id = %user_id,
                    tech_err = ?err,
                    local_err = ?Status::SqlQueryWrongLogic
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        };


    match session_user_dto.try_into() {
        Ok(session_user) => Ok(session_user),
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN get_user_by_device_token FAILED BY MAPPING SessionUser"
            );
            Err(Status::MappingError)
        }
    }
    
}
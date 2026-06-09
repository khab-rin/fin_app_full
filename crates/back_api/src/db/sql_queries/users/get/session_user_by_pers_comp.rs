use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;
use shared_lib::service::auth_service::client_state::{SessionUser, SessionUserDto};

use crate::config::BackApiState;

pub(crate) async fn get_session_user_by_pers_comp(
    state: &Arc<BackApiState>,
    pers_id: &BoxUuid,
    comp_id: &BoxUuid
) -> Result<Option<SessionUser>, Status> {

    let session_user_dto_opt = match sqlx::
        query_file_as!(
            SessionUserDto,
            "src/db/sql_queries/users/get/session_user_by_pers_comp.sql",
            pers_id.as_ref(),
            comp_id.as_ref()
        ).fetch_optional(&state.pool_fast).await {
            Ok(o) => o,
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::SqlQueryWrongLogic,
                    failed_data = ?(pers_id, comp_id),
                    "FUN session_user_dto_opt FAILED BY SQL QUERY"
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        };

    let session_user_dto = match session_user_dto_opt {
        Some(dto) => dto,
        None => return Ok(None)
    };

    match session_user_dto.try_into() {
        Ok(session_user) => Ok(Some(session_user)),
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
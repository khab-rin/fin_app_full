use std::sync::Arc;

use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::BoxUuid;
use shared_lib::service::auth_service::general::{SessionUser, SessionUserDto};

use crate::config::BackApiState;

pub(crate) async fn get_session_user_by_pers_comp(
    state: &Arc<BackApiState>,
    pers_id: &BoxUuid,
    comp_id: &BoxUuid
) -> Result<Option<SessionUser>, Status> {

    let session_user_dto_opt = sqlx::query_file_as!(
            SessionUserDto,
            "src/db/sql_queries/users/get/session_user_by_pers_comp.sql",
            pers_id.as_ref(),
            comp_id.as_ref()
        ).fetch_optional(&state.pool_fast)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    let session_user_dto = match session_user_dto_opt {
        Some(dto) => dto,
        None => return Ok(None)
    };

    let session_user = session_user_dto
        .try_into()
        .map_err(|err:serde_json::Error| err.process_err(Status::MappingError, ""))?;

    Ok(Some(session_user))
}
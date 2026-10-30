use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::BoxUuid;
use shared_lib::service::auth_service::general::{SessionUserDto, SessionUser};

use crate::config::BackApiState;

pub(crate) async fn get_user_by_user_id(
    state: &BackApiState,
    user_id: &BoxUuid
) -> Result<Option<SessionUser>, Status> {

	let session_user_dto_option = sqlx::query_file_as!(
		SessionUserDto,
		"src/db/sql_queries/users/get/by_user_id.sql",
		user_id.as_ref()
	).fetch_optional(&state.pool_fast)
	.await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	if let Some(session_user_dto) = session_user_dto_option {
		let session_user: SessionUser = session_user_dto.try_into()
			.map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;
		return Ok(Some(session_user));
	}
    
	Ok(None)
}
use shared_lib::{ProcessError, Status};
use shared_lib::primitives::frozen::text::BoxUuid;

use crate::config::BackApiState;

pub(crate) async fn set_guid_by_user_id(
    state: &BackApiState,
    user_id: &BoxUuid,
    guid: &BoxUuid
) -> Result<(), Status> {

    if let Err(err) = sqlx::query_file!(
        "src/db/sql_queries/users/set/guid_by_user_id.sql",
        user_id.as_ref(),
        guid.as_ref()
    ).fetch_optional(&state.pool_fast).await {
        return Err(err.process_err(Status::SqlQueryWrongLogic, ""));
    }

    Ok(())

}
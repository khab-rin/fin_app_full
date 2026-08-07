
use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::BoxUuid;

use crate::config::BackApiState;


pub(crate) async fn new_session(
    state: &BackApiState,
    user_id: &BoxUuid,
    device_id: &BoxUuid
) -> Result<BoxUuid, Status> {

    let record = sqlx::query_file!(
            "src/db/sql_queries/sessions/set/new_session.sql",
            user_id.as_ref(),
            device_id.as_ref(),
        ).fetch_one(&state.pool_fast)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    Ok(record.token) 

}
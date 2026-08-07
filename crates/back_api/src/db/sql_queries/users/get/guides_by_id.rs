use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::BoxUuid;

use crate::config::BackApiState;

pub(crate) async fn get_guids_by_user_id(
    state: &BackApiState,
    user_id: &BoxUuid
) -> Result<Option<Vec<BoxUuid>>, Status> {


    let record_option = sqlx::query_file!(
        "src/db/sql_queries/users/get/guides_by_id.sql",
        user_id.as_ref()
    ).fetch_optional(&state.pool_fast)
    .await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    let record = match record_option {
        Some(r) => r,
        None => return Ok(None)
    };


    Ok(Some(record.guids))
    
}
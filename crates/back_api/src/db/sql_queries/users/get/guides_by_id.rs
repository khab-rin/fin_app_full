use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;

use crate::config::BackApiState;

pub(crate) async fn get_guids_by_user_id(
    state: &BackApiState,
    user_id: &BoxUuid
) -> Result<Option<Vec<BoxUuid>>, Status> {


    let record_option = match sqlx::query_file!(
        "src/db/sql_queries/users/get/guides_by_id.sql",
        user_id.as_ref()
    ).fetch_optional(&state.pool_fast).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                tech_err = ?err, local_err = ?&Status::SqlQueryWrongLogic,
                "FUN get_guids_by_user_id FAILED BY SQL QUERY"
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };

    let record = match record_option {
        Some(r) => r,
        None => return Ok(None)
    };


    Ok(Some(record.guids))
    
}
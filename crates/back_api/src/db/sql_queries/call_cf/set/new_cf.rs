use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;

use crate::config::BackApiState;

pub(crate) async fn new_cf(
    state: &Arc<BackApiState>,
    user_id: &BoxUuid,
    device_id: &BoxUuid,
    external_id: &String
) -> Result<bool, Status> {

    let insert_res = sqlx::
        query_file!(
            "src/db/sql_queries/call_cf/set/new_cf.sql",
            user_id.as_ref(),
            device_id.as_ref(),
            external_id
        ).fetch_optional(&state.pool_fast)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::SqlQueryWrongLogic
            )
        })
        .map_err(|_| Status::SqlQueryWrongLogic)?;

    Ok(insert_res.is_some())
}
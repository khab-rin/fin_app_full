use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;

use crate::config::ApiState;

pub(crate) async fn new_session(
    state: &Arc<ApiState>,
    user_id: &BoxUuid,
    device_id: &BoxUuid
) -> Result<BoxUuid, Status> {

    match sqlx::
        query_file!(
            "src/db/sql_queries/sessions/set/new_session.sql",
            user_id.as_ref(),
            device_id.as_ref(),
        ).fetch_one(&state.pool)
        .await {
            Ok(t) => Ok(t.token),
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::BackSqlQuerySessions,
                    "FUN new_session FAILED"
                );
                Err(Status::BackSqlQuerySessions)
            }
        }
}
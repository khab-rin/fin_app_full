use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;

use crate::config::BackApiState;

pub(crate) async fn exists_user_by_pers_comp(
    state: &Arc<BackApiState>,
    pers_id: &BoxUuid,
    comp_id: &BoxUuid
) -> Result<bool, Status> {

    let exist_opt = match sqlx::
        query_file!(
            "src/db/sql_queries/users/get/exists_user_by_pers_comp.sql",
            pers_id.as_ref(),
            comp_id.as_ref()
        ).fetch_optional(&state.pool).await {
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

    Ok(exist_opt.is_some())
}
use std::sync::Arc;

use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::BoxUuid;

use crate::config::BackApiState;

pub(crate) async fn exists_user_by_pers_comp(
    state: &Arc<BackApiState>,
    pers_id: &BoxUuid,
    comp_id: &BoxUuid
) -> Result<bool, Status> {

    let exist_opt = sqlx::query_file!(
            "src/db/sql_queries/users/get/exists_user_by_pers_comp.sql",
            pers_id.as_ref(),
            comp_id.as_ref()
        ).fetch_optional(&state.pool_fast)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    Ok(exist_opt.is_some())
}
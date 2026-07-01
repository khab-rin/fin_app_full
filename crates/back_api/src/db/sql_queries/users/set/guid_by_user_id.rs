use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;

use crate::config::BackApiState;

pub(crate) async fn set_guid_by_user_id(
    state: &BackApiState,
    user_id: &BoxUuid,
    guid: &BoxUuid
) -> Result<(), Status> {

    match sqlx::query_file!(
        "src/db/sql_queries/users/set/guid_by_user_id.sql",
        user_id.as_ref(),
        guid.as_ref()
    ).fetch_optional(&state.pool_fast).await {
        Ok(_) => Ok(()),
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::SqlQueryWrongLogic,
                "FUN set_guid_by_user_id FAILED BY SQL QUERY"
            );
            Err(Status::SqlQueryWrongLogic)
        }
    }

}
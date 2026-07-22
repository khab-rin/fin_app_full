use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;

use crate::config::BackApiState;
use crate::db::sql_queries::users::get::guides_by_id::get_guids_by_user_id;

pub(crate) async fn del_guids_by_user_id(
    state: &BackApiState,
    user_id: &BoxUuid,
    del_guids: &[BoxUuid]
) -> Result<(), Status> {

    if del_guids.is_empty() { return Ok(()); }

    let prev_guids_option = match get_guids_by_user_id(state, user_id).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN set_guid_by_user_id FAILED BY FUN get_guids_by_user_id"
            );
            return Err(err);
        }
    };

    let prev_guids = match prev_guids_option {
        Some(g) => g,
        None => return Ok(())
    };

    let mut new_guids: Vec<uuid::Uuid> = vec!();

    for g in prev_guids {
        if del_guids.contains(&g) {
            continue;
        } else {
            new_guids.push(*g);
        }
    }

    match sqlx::query_file!(
        "src/db/sql_queries/users/set/del_guids.sql",
        user_id.as_ref(),
        &new_guids
    ).fetch_optional(&state.pool_fast).await {
        Ok(_) => Ok(()),
        Err(err) => {
            tracing::error!(
                tech_err = ?err, local_err = ?Status::SqlQueryWrongLogic,
                "FUN set_guid_by_user_id FAILED BY FUN get_guids_by_user_id"
            );
            Err(Status::SqlQueryWrongLogic)
        } 
    }

}
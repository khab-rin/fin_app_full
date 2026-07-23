use sqlx::Row;
use std::collections::HashSet;

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
                "FUN del_guids_by_user_id FAILED BY FUN get_guids_by_user_id"
            );
            return Err(err);
        }
    };

    let prev_guids = match prev_guids_option {
        Some(g) => g,
        None => return Ok(())
    };

    tracing::info!(prev_guids = ?prev_guids);

    let mut new_guids_set: HashSet<BoxUuid> =HashSet::new();

    for g in prev_guids {
        if del_guids.contains(&g) {
            continue;
        } else {
            new_guids_set.insert(g);
        }
    }

    let new_guids_vec: Vec<uuid::Uuid> = new_guids_set.into_iter().map(|x| *x).collect();


    let row = if new_guids_vec.is_empty() {
        match sqlx::query(
            "UPDATE users
            SET 
                guids = '{}',
                last_update = CURRENT_TIMESTAMP
            WHERE user_id = $1
            RETURNING user_id;"
        ).bind(user_id.as_ref())
        .fetch_optional(&state.pool_fast).await {
            Ok(o) => o.map(|row| row.get("user_id")),
            Err(err) => {
                tracing::error!(
                    tech_err = ?err, local_err = ?Status::SqlQueryWrongLogic,
                    "FUN del_guids_by_user_id FAILED BY sqlx::query"
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        }
    } else {
        match sqlx::query_file!(
            "src/db/sql_queries/users/set/del_guids.sql",
            user_id.as_ref(),
            &new_guids_vec
        ).fetch_optional(&state.pool_fast).await {
            Ok(o) => o.map(|record| record.user_id),
            Err(err) => {
                tracing::error!(
                    tech_err = ?err, local_err = ?Status::SqlQueryWrongLogic,
                    "FUN del_guids_by_user_id FAILED BY sqlx::query_file!"
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        }
    };

    tracing::info!(row = ?row);

    if row.is_none() {
        tracing::warn!(user_id = ?user_id, "UPDATE executed, but user_id not found!");
    }

    Ok(())

}
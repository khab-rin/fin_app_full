use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, DateTime};
use shared_lib::sql_models::user::implements::{User, UserSetData};

use crate::config::BackApiState;

pub(crate) async fn set_user(
    state: &Arc<BackApiState>,
    set_data: &UserSetData,
) -> Result<User, Status> {

    let UserSetData {
        pers_id,
        comp_id,
        phone,
        password_hash,
        email,
        mchd_tax_guid,
        mchd_tax_file,
        mchd_home_guid,
        mchd_home_file,
    } = set_data;

    match sqlx::
        query_file_as!(
            User,
            "src/db/sql_queries/users/set/user.sql",
            pers_id.as_ref(),
            comp_id.as_ref(),
            phone.as_ref(),
            password_hash,
            email.as_ref(),
            mchd_tax_guid.as_deref(),
            mchd_tax_file.as_deref(),
            mchd_home_guid.as_deref(),
            mchd_home_file.as_deref(),
        ).fetch_one(&state.pool_fast).await {
            Ok(u) => Ok(u),
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::SqlQueryWrongLogic,
                    failed_data = ?set_data,
                    "FUN set_user FAILED BY SQL QUERY"
                );
                Err(Status::SqlQueryWrongLogic)
            }
        }

}
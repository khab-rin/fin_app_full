use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, DateTime};
use shared_lib::sql_models::user::implements::{User, UserSetData, UserDto};

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
        tax_powers,
        mchd_home_guid,
        home_powers,
    } = set_data;

    let user_dto = match sqlx::
        query_file_as!(
            UserDto,
            "src/db/sql_queries/users/set/user.sql",
            pers_id.as_ref(),
            comp_id.as_ref(),
            phone.as_ref(),
            password_hash,
            email.as_ref(),
            mchd_tax_guid.as_deref(),
            serde_json::to_value(tax_powers).unwrap_or_default(),
            mchd_home_guid.as_deref(),
            serde_json::to_value(home_powers).unwrap_or_default(),
        ).fetch_one(&state.pool_fast).await {
            Ok(u) => u,
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::SqlQueryWrongLogic,
                    failed_data = ?set_data,
                    "FUN set_user FAILED BY SQL QUERY"
                );
                return Err(Status::SqlQueryWrongLogic)
            }
        };

    match user_dto.try_into() {
        Ok(u) => Ok(u),
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                failed_data = ?set_data,
                "FUN set_user FAILED BY MAPPING User FROM UserDto"
            );
            return Err(Status::MappingError);
        }
    }


}
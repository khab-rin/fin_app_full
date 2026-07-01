use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, DateTime};
use shared_lib::sql_models::user::implements::{User, UserSetData, UserDto};

use crate::config::BackApiState;

pub(crate) async fn add_user(
    state: &Arc<BackApiState>,
    set_data: &UserSetData,
) -> Result<User, Status> {

    let UserSetData {
        pers_id,
        comp_id,
        phone,
        password_hash,
        email,
        guids,
    } = set_data;

    let guids_vec: Vec<uuid::Uuid> = guids.iter().map(|x| x.as_ref().clone()).collect();

    let user_dto = match sqlx::
        query_file_as!(
            UserDto,
            "src/db/sql_queries/users/add/user.sql",
            pers_id.as_ref(),
            comp_id.as_ref(),
            phone.as_ref(),
            password_hash,
            email.as_ref(),
            &guids_vec,

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
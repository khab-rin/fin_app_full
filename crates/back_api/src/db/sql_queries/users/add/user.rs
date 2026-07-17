use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, DateTime};
use shared_lib::sql_models::user::implements::{User, UserSetData, UserDto};

use crate::config::BackApiState;
use crate::db::sql_queries::users::get::by_pers_comp_id::get_user_by_inn_pers_comp_kpp;

pub(crate) async fn add_user(
    state: &BackApiState,
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

    let exist_user_dto_option = match get_user_by_inn_pers_comp_kpp(state, pers_id, comp_id).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN add_user FAILED BY FUN get_user_by_inn_pers_comp_kpp"
            );
            return Err(err);
        }
    };

    if exist_user_dto_option.is_some() {
        tracing::error!(
            local_err = ?Status::SystemLogicErr,
            "FUN add_user FAILED BY TRYING INSERT DUPLICATE USER"
        );
        return Err(Status::SystemLogicErr);
    };

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
                    "FUN add_user FAILED BY SQL QUERY"
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
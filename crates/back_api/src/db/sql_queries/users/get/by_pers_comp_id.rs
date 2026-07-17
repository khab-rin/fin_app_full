use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{CompInn, PersInn, Kpp, BoxUuid, DateTime};
use shared_lib::sql_models::user::implements::{UserDto, User};

use crate::config::BackApiState;

pub(crate) async fn get_user_by_inn_pers_comp_kpp(
    state: &BackApiState, 
    pers_id: &BoxUuid,
    comp_id: &BoxUuid,
) -> Result<Option<User>, Status> {

    let user_dto_option = match sqlx::
        query_file_as!(
            UserDto,
            "src/db/sql_queries/users/get/by_pers_comp_id.sql",
            pers_id.as_ref(),
            comp_id.as_ref(),
        ).fetch_optional(&state.pool_fast).await {
            Ok(o) => o,
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::SqlQueryWrongLogic,
                    "FUN get_user_by_inn_pers_comp_kpp FILED BY SQL QUERY"
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        };
        
    let user_dto = match user_dto_option {
        Some(d) => d,
        None => return Ok(None)
    };

    match user_dto.try_into() {
        Ok(u) => Ok(Some(u)),
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN get_user_by_inn_pers_comp_kpp FILED BY MAPPING DTO"
            );
            Err(Status::MappingError)
        }
    }

}
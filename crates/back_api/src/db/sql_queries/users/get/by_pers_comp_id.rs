use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{BoxUuid, DateTime};
use shared_lib::sql_models::user::implements::{UserDto, User};

use crate::config::BackApiState;

pub(crate) async fn get_user_by_pers_comp_id(
    state: &BackApiState, 
    pers_id: &BoxUuid,
    comp_id: &BoxUuid,
) -> Result<Option<User>, Status> {

    let user_dto_option = sqlx::query_file_as!(
            UserDto,
            "src/db/sql_queries/users/get/by_pers_comp_id.sql",
            pers_id.as_ref(),
            comp_id.as_ref(),
        ).fetch_optional(&state.pool_fast)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;  
        
    let user_dto = match user_dto_option {
        Some(d) => d,
        None => return Ok(None)
    };

    let user = user_dto
        .try_into()
        .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;  

    Ok(Some(user))

}
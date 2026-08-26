use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{BoxUuid, DateTime};
use shared_lib::sql_models::user::implements::{User, UserSetData, UserDto};

use crate::config::BackApiState;
use crate::db::sql_queries::users::get::by_pers_comp_id::get_user_by_pers_comp_id;

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

    let guids_vec: Vec<uuid::Uuid> = guids.iter().map(|x| *x.as_ref()).collect();

    let exist_user_dto_option = get_user_by_pers_comp_id(state, pers_id, comp_id)
        .await
        .map_err(|err| err.process_err(err, ""))?;  

    if exist_user_dto_option.is_some() {
        return Err(Status::Tech.process_err(Status::SystemLogicErr, ""));
    };
	
	let user_id = BoxUuid::unchecked(uuid::Uuid::new_v4());

    let user_dto = sqlx::query_file_as!(
            UserDto,
            "src/db/sql_queries/users/add/user.sql",
			user_id.as_ref(),
            pers_id.as_ref(),
            comp_id.as_ref(),
            phone.as_ref(),
            password_hash,
            email.as_ref(),
            &guids_vec,
        ).fetch_one(&state.pool_fast)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;  

    user_dto
        .try_into()
        .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))  


}
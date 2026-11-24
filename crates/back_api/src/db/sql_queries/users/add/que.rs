use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{BoxUuid, DateTime};
use shared_lib::sql_models::user::implements::{User, UserSetData, UserDto};


pub async fn sql_insert_update_user_function<'t, E>(
    conn: E,
    set_data: &UserSetData,
) -> Result<User, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
    let UserSetData {
		user_id,
        pers_id,
        comp_id,
        phone,
        password_hash,
        email,
        guids,
    } = set_data;

    let guids_vec: Vec<uuid::Uuid> = guids.iter().map(|x| *x.as_ref()).collect();


    let user_dto = sqlx::query_file_as!(
            UserDto,
            "src/db/sql_queries/users/add/que_insert_update_user.sql",
			user_id.as_ref(),
            pers_id.as_ref(),
            comp_id.as_ref(),
            phone.as_ref(),
            password_hash,
            email.as_ref(),
            &guids_vec,
        ).fetch_one(conn)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;  

    user_dto
        .try_into()
        .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, None))  


}
use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::person::implements::{Person};

use crate::config::BackApiState;
use crate::db::sql_queries::persons::get::que::sql_get_person_by_inn_function;
use crate::db::sql_queries::persons::add::que::sql_insert_update_person_function;

pub(crate) async fn get_update_insert_person_function(
    state: &BackApiState,
    person: &Person
) -> Result<Person, Status> {

	let mut conn = state.pool_fast.acquire().await
		.map_err(|err| err.process_err(Status::SqlPoolErr, None))?;

    let exist_person_option = sql_get_person_by_inn_function(&mut *conn, &person.pers_inn)
        .await
        .map_err(|err| err.process_err(err, None))?;

    let person = match exist_person_option {
        Some(mut exist_person) => {
            exist_person.metadata.merge(person.metadata.clone());
            exist_person
        },
        None => {person.clone()}
    };

    sql_insert_update_person_function(&mut *conn, &person).await
		.map_err(|err| err.process_err(err, None))
    
}
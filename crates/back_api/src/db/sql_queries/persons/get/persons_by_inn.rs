use shared_lib::primitives::frozen::text::{PersInn, BoxUuid, DateTime};
use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::person::implements::{Person, PersonDto};

use crate::config::BackApiState;
use crate::db::sql_queries::persons::get::helper::dtos_to_persons;

pub(crate) async fn get_persons_by_inn(
    state: &BackApiState, 
    data: &[PersInn]
) -> Result<Vec<Person>, Status> {
    let inn_data: Vec<String> = data.iter().map(|inn| inn.to_string()).collect();

    let persons_dto = sqlx::query_file_as!(
            PersonDto,
            "src/db/sql_queries/persons/get/persons_by_inn.sql",
            &inn_data[..]
        ).fetch_all(&state.pool_long)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    
    dtos_to_persons(persons_dto)
}
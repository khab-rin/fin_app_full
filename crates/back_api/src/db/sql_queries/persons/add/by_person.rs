use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{BoxUuid, PersInn, DateTime};
use shared_lib::sql_models::person::implements::{Person, PersonDto};

use crate::config::BackApiState;
use crate::db::sql_queries::persons::get::person_by_inn::get_person_by_inn;

pub(crate) async fn add_person(
    state: &BackApiState,
    person: &Person
) -> Result<Person, Status> {

    let exist_person_option = get_person_by_inn(state, &person.pers_inn)
        .await
        .map_err(|err| err.process_err(err, ""))?;

    let person = match exist_person_option {
        Some(mut exist_person) => {
            exist_person.metadata.merge(person.metadata.clone());
            exist_person
        },
        None => {person.clone()}
    };

    let person_dto = sqlx::query_file_as!(
            PersonDto,
            "src/db/sql_queries/persons/add/by_person.sql",
            person.pers_id.as_ref(),
            person.pers_inn.as_ref(),
            serde_json::to_value(&person.metadata).unwrap_or_default()
        ).fetch_one(&state.pool_fast)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;


    let person = person_dto
        .try_into()
        .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;

        
    Ok(person)
    
}
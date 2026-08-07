use shared_lib::primitives::frozen::text::{PersInn, BoxUuid, DateTime};
use shared_lib::{ProcessError, Status};
use shared_lib::sql_models::person::implements::{Person, PersonDto};

use crate::config::BackApiState;


pub(crate) async fn get_person_by_userid(
    state: &BackApiState,
    user_id: &BoxUuid
) -> Result<Option<Person>, Status> {

    let person_dto_option = sqlx::query_file_as!(
        PersonDto,
        "src/db/sql_queries/persons/get/person_by_userid.sql",
        user_id.as_ref()
    ).fetch_optional(&state.pool_fast)
    .await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    let person_dto = match person_dto_option {
        Some(d) => d,
        None => return Ok(None)
    };

    let person: Person = person_dto
        .try_into()
        .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?; 


    Ok(Some(person))
}
use shared_lib::primitives::frozen::text::{PersInn, BoxUuid, DateTime};
use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::person::implements::{Person, PersonDto};

use crate::config::BackApiState;

pub(crate) async fn get_person_by_inn(
    state: &BackApiState,
    pers_inn: &PersInn
) -> Result<Option<Person>, Status> {

    let person_dto_option = sqlx::query_file_as!(
        PersonDto,
        "src/db/sql_queries/persons/get/person_by_inn.sql",
        pers_inn.as_ref()
    ).fetch_optional(&state.pool_fast)
    .await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    match person_dto_option {
        Some(person_dto) => {
            let person = person_dto
                .try_into()
                .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;
            Ok(Some(person))
        },
        None => Ok(None)
    }

}
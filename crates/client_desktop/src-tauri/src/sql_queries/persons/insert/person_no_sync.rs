use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{PersInn, BoxUuid, DateTime};
use shared_lib::sql_models::person::implements::{PersonDto, Person};

use crate::state::ClientState;

pub(crate) async fn insert_person_no_sync(
    state: &ClientState,
    person: &Person
) -> Result<Person, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN insert_person_sync FAILED MISS SESSION, local_err = {}",
                err
            );
            return Err(err);
        }
    };

    let metadata = serde_json::to_value(&person.metadata).unwrap_or_default();

    let person_dto = match sqlx::query_file_as!(
        PersonDto,
        "src/sql_queries/persons/insert/person_sync.sql",
        person.pers_id,
        person.pers_inn,
        metadata,
        person.last_update
    ).fetch_one(&session.local_db).await {
        Ok(p) => p,
        Err(err) => {
            log::error!(
                "FUN insert_person_sync FAILED BY SQL QUERY, tech_err = {}, local_err = {}",
                err, Status::SqlQueryWrongLogic
            );
            return Err(Status::SqlQueryWrongLogic);
        } 
    };

    let person = match person_dto.try_into() {
        Ok(p) => {p},
        Err(err) => {
            log::error!(
                "FUN insert_person_sync FAILED BY MAPPING Person, tech_err = {}, local_err = {}",
                err, Status::MappingError
            );
            return Err(Status::MappingError);
        }
    };

    Ok(person)

}
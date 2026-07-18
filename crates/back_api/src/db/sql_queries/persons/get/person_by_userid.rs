use shared_lib::primitives::frozen::implements::{PersInn, BoxUuid, DateTime};
use shared_lib::Status;
use shared_lib::sql_models::person::implements::{Person, PersonDto};

use crate::config::BackApiState;


pub(crate) async fn get_person_by_userid(
    state: &BackApiState,
    user_id: &BoxUuid
) -> Result<Option<Person>, Status> {

    let person_dto_option = match sqlx::query_file_as!(
        PersonDto,
        "src/db/sql_queries/persons/get/person_by_userid.sql",
        user_id.as_ref()
    ).fetch_optional(&state.pool_fast).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::SqlQueryWrongLogic,
                "FUN get_person_by_userid FAILED BY SQL QUERY"
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };

    let person_dto = match person_dto_option {
        Some(d) => d,
        None => return Ok(None)
    };

    let person: Person = match person_dto.try_into() {
        Ok(p) => p,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN get_person_by_userid FAIELD BY DTO MAPPING"
            );
            return Err(Status::MappingError);
        }
    };


    Ok(Some(person))
}
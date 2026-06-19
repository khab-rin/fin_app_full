use shared_lib::primitives::frozen::implements::{PersInn, BoxUuid, DateTime};
use shared_lib::Status;
use shared_lib::sql_models::person::implements::{Person, PersonDto};

use crate::config::BackApiState;

pub(crate) async fn get_person_by_inn(
    state: &BackApiState,
    inn: &PersInn
) -> Result<Option<Person>, Status> {

    let person_dto_option = match sqlx::query_file_as!(
        PersonDto,
        "src/db/sql_queries/persons/get/person_by_inn.sql",
        inn.as_ref()
    ).fetch_optional(&state.pool_fast).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                teck_err = ?err,
                local_err = ?Status::SqlQueryWrongLogic,
                "FUN get_person_by_inn FAILED BY SQL QUERY"
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };

    match person_dto_option {
        Some(person_dto) => {
            match person_dto.try_into() {
                Ok(p) => Ok(Some(p)),
                Err(err) => {
                    tracing::error!(
                        teck_err = ?err,
                        local_err = ?Status::MappingError,
                        "FUN get_person_by_inn FAILED BY SQL QUERY"
                    );
                    Err(Status::MappingError)
                }
            }
        },
        None => Ok(None)
    }

}
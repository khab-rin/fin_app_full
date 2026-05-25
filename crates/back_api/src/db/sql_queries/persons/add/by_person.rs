use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, Inn, DateTime};
use shared_lib::sql_models::person::implements::{Person, PersonDto};

use crate::config::BackApiState;

pub(crate) async fn add_person(
    state: &Arc<BackApiState>,
    person: &Person
) -> Result<Person, Status> {

    let person_option = match sqlx::
        query_file_as!(
            PersonDto,
            "src/db/sql_queries/persons/add/by_person.sql",
            person.pers_id.as_ref(),
            person.inn.as_ref(),
            serde_json::to_value(&person.metadata).unwrap_or_default()
        ).fetch_optional(&state.pool)
        .await {
            Ok(r) => r,
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::SqlQueryWrongLogic,
                    "FUN add_person FAILED BY WRONG QUERY LOGIC"
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        };

    let person_dto = match person_option {
        Some(r) => r,
        None => {
            tracing::error!(
                local_err = ?Status::SqlQueryWrongLogic,
                "FUN add_person FAILED BY WRONG QUERY LOGIC"
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };

    match person_dto.try_into() {
        Ok(p) => Ok(p),
        Err(err) => {
            tracing::error!(
                err = ?err,
                "FUN add_person FAILED BY MAPPING Person"
            );
            Err(err)
        }
    }
}
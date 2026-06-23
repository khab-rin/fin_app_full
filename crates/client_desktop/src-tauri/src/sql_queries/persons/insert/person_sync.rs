use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{PersInn, BoxUuid, DateTime};
use shared_lib::sql_models::person::implements::Person;

use crate::state::ClientState;
use crate::sql_queries::persons::get::by_inn::get_person_by_inn;

pub(crate) async fn insert_person_sync(
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

    let prev_person_option = match get_person_by_inn(state, &person.pers_inn).await {
        Ok(o) => o,
        Err(err) => {
            log::error!("FUN insert_person_sync FAILED BY FUN get_person_by_inn");
            return Err(err);
        }
    };

    let mut prev_person = match prev_person_option {
        Some(p) => p,
        None => person.clone()
    };

    prev_person.metadata.merge(person.metadata.clone());

    let metadata_value: serde_json::Value = serde_json::to_value(&prev_person.metadata)
        .unwrap_or(serde_json::Value::Null);


    match sqlx::query_file!(
        "src/sql_queries/persons/insert/person_sync.sql",
        prev_person.pers_id,
        prev_person.pers_inn,
        metadata_value,
        prev_person.last_update
    ).fetch_optional(&session.local_db).await {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "FUN insert_person_sync FAILED BY SQL QUERY, tech_err = {}, local_err = {}",
                err, Status::SqlQueryWrongLogic
            );
            return Err(Status::SqlQueryWrongLogic);
        } 
    };

    Ok(prev_person)

}
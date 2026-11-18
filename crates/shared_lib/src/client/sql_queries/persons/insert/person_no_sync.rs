use crate::{ClientState, ProcessError, Status};
use crate::primitives::frozen::text::{PersInn, BoxUuid, DateTime};
use crate::sql_models::person::implements::Person;

use crate::client::sql_queries::persons::get::que::sql_get_person_by_inn_function;

pub async fn insert_person_no_sync(
    state: &ClientState,
    person: &Person
) -> Result<Person, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?;

    let prev_person_option = sql_get_person_by_inn_function(&session.local_db, &person.pers_inn)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    let mut prev_person = match prev_person_option {
        Some(p) => p,
        None => person.clone()
    };

    prev_person.metadata.merge(person.metadata.clone());

    let metadata_value: serde_json::Value = serde_json::to_value(&prev_person.metadata)
        .unwrap_or(serde_json::Value::Null);


    sqlx::query_file!(
            "src/client/sql_queries/persons/insert/person_no_sync.sql",
            prev_person.pers_id,
            prev_person.pers_inn,
            metadata_value,
            prev_person.last_update
        ).fetch_optional(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    Ok(prev_person)

}
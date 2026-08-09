use crate::{Status, ClientState, ProcessError};
use crate::primitives::frozen::text::{PersInn, BoxUuid, DateTime};
use crate::sql_models::person::implements::{PersonDto, Person};


pub async fn get_person_by_inn(
    state: &ClientState,
    inn: &PersInn
) -> Result<Option<Person>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

    let inn_str = inn.to_string();

    let person_dto_option = sqlx::query_file_as!(
            PersonDto,
            "src/client/sql_queries/persons/get/by_inn.sql",
            inn_str
        ).fetch_optional(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    let person_dto = match person_dto_option {
        Some(dto) => dto,
        None => return Ok(None)
    };

    match person_dto.try_into() {
        Ok(p) => Ok(Some(p)),
        Err(err) => {
            Err(err.process_err(Status::MappingError, ""))
        }
    }

}
use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{PersInn, BoxUuid, DateTime};
use shared_lib::sql_models::person::implements::{PersonDto, Person};

use crate::state::ClientState;


pub(crate) async fn get_person_by_inn(
    state: &ClientState,
    inn: &PersInn
) -> Result<Option<Person>, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "MISS SESSION IN FUN get_person_by_inn, err = {}", err
            );
            return Err(err);
        }
    };

    let inn_str = inn.to_string();

    let person_dto_option = match sqlx::query_file_as!(
        PersonDto,
        "src/sql_queries/persons/get/by_inn.sql",
        inn_str
    ).fetch_optional(&session.local_db).await {
        Ok(o) => o,
        Err(err) => {
            log::error!(
                "MISS SESSION IN FUN get_person_by_inn, tech_err = {}, locla_err = {}",
                err, Status::SqlQueryWrongLogic
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };

    let person_dto = match person_dto_option {
        Some(dto) => dto,
        None => return Ok(None)
    };

    match person_dto.try_into() {
        Ok(p) => Ok(Some(p)),
        Err(err) => {
            log::error!(
                "FUN get_person_by_inn FAILED BY MAPPING DTO, tech_err = {}, local_err = {}",
                err, Status::MappingError
            );
            Err(Status::MappingError)
        }
    }

    

}
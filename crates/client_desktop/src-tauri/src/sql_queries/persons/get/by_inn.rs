use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{PersInn, BoxUuid, DateTime};
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::sql_models::person::implements::{PersonDto, Person};

use crate::state::ClientState;
use crate::sql_queries::persons::insert::person_sync::insert_person_sync;
use crate::back_api::post_query::post_query_back_api;


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

    let mut person_dto_option = match sqlx::query_file_as!(
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

    if let Some(person_dto) = person_dto_option {
        match person_dto.try_into() {
            Ok(p) => return Ok(Some(p)),
            Err(err) => {
                log::error!(
                "FUN get_person_by_inn FAILED BY MAPPING PERSON, tech_err = {}, local_err = {}",
                err, Status::MappingError
            );
            return Err(Status::MappingError);
            }
        };
    }

    let back_api_response = match post_query_back_api(
        state, 
        state.config.get_inst_client(), 
        ApiRoutes::SqlPersonGetByInn, 
        inn
    ).await {
        Ok(r) => r,
        Err(err) => {
            log::error!(
                "FUN get_person_by_inn FAILED BY QUERY TO BACK API, local_err = {}", err
            );
            return Err(err);
        }
    };

    person_dto_option = match back_api_response.json().await {
        Ok(o) => o,
        Err(err) => {
            log::error!(
                "FUN get_person_by_inn FAILED BY MAPPING PERSON DTO, tech_err = {}, local_err = {}",
                err, Status::MappingError
            );
            return Err(Status::MappingError);
        }
    };

    let person_dto = match person_dto_option {
        Some(p) => p,
        None => return Ok(None)
    };

    let person: Person = match person_dto.try_into() {
        Ok(p) => p,
        Err(err) => {
            log::error!(
                "FUN get_person_by_inn FAILED BY MAPPING PERSON DTO, tech_err = {}, local_err = {}",
                err, Status::MappingError
            );
            return Err(Status::MappingError);
        }
    };

    let _ = insert_person_sync(state, &person).await;

    Ok(Some(person))
}
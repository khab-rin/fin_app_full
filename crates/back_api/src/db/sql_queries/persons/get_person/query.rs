use sqlx::PgPool;

use shared_lib::primitives::frozen::implements::{Inn, BoxUuid, DateTime};
use shared_lib::Status;
use shared_lib::sql_models::person::implements::{Person, PersonDto};
use uuid::Uuid;

use crate::db::sql_queries::persons::get_person::helper::dto_to_person;

pub(crate) async fn get_person_by_inn(
    pool: &PgPool, 
    data: &[Inn]
) -> Result<Vec<Person>, Status> {
    let inn_data: Vec<String> = data.iter().map(|inn| inn.to_string()).collect();

    let persons_dto = sqlx::
        query_file_as!(
            PersonDto,
            "src/db/sql_queries/persons/get_person/get_person_by_inn.sql",
            &inn_data[..]
        ).fetch_all(pool)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                stat_err = ?Status::SqlQueryWrongLogic
            );
        })
        .map_err(|_| Status::SqlQueryWrongLogic)?;
    
    Ok(dto_to_person(persons_dto))

}

pub(crate) async fn get_person_by_id(
    pool: &PgPool, 
    id_data: &[Uuid]
) -> Result<Vec<Person>, Status> {
    let persons_dto = sqlx::
        query_file_as!(
            PersonDto,
            "src/db/sql_queries/persons/get_person/get_person_by_id.sql",
            id_data
        ).fetch_all(pool)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                stat_err = ?Status::SqlQueryWrongLogic
            );
        }).map_err(|_| Status::SqlQueryWrongLogic)?;
    
    Ok(dto_to_person(persons_dto))
}
use crate::{Status, ProcessError};
use crate::primitives::frozen::text::{PersInn, BoxUuid, DateTime};
use crate::sql_models::person::implements::{PersonDto, Person};
use crate::static_data::null_elems::NULLS;


pub async fn sql_get_person_by_inn_function<'t, E>(
    conn: E,
    inn: &PersInn
) -> Result<Option<Person>, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Sqlite>
{
    let person_dto_option = sqlx::query_file_as!(
            PersonDto,
            "src/client/sql_queries/persons/get/que_get_person_by_inn.sql",
            inn
        ).fetch_optional(conn)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;

    let person_dto = match person_dto_option {
        Some(dto) => dto,
        None => return Ok(None)
    };

    match person_dto.try_into() {
        Ok(p) => Ok(Some(p)),
        Err(err) => {
            Err(err.process_err(Status::MappingError, None))
        }
    }
}

pub async fn sql_get_all_persons_function<'t, E>(
	conn: E,
) -> Result<Vec<Person>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Sqlite>
{

	let persons_dto = sqlx::query_file_as!(
		PersonDto,
		"src/client/sql_queries/persons/get/que_get_all_persons.sql"
	).fetch_all(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;

	persons_dto
		.into_iter()
		.filter(|x| x.pers_id != NULLS.id.pers_id)
		.map(|dto| {
			dto.try_into()
			.map_err(|err:serde_json::Error| err.process_err(Status::MappingError, None))
		}).collect()

}
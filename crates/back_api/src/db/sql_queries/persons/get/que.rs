use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::person::implements::{Person, PersonDto};
use shared_lib::primitives::frozen::text::{BoxUuid, PersInn, DateTime};

pub async fn sql_get_person_by_id_function<'t, E>(
	conn: E,
	pers_id: &BoxUuid
) -> Result<Option<Person>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
	let dto_option = sqlx::query_file_as!(
		PersonDto,
		"src/db/sql_queries/persons/get/que_get_person_by_id.sql",
		pers_id as &BoxUuid
	).fetch_optional(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;

	match dto_option {
		Some(dto) => {
			let person = dto.try_into().map_err(|err: serde_json::Error| err.process_err(Status::MappingError, None))?;
			Ok(Some(person))
		},
		None => Ok(None)
	}
}

pub async fn sql_get_person_by_inn_function<'t, E>(
	conn: E,
	pers_inn: &PersInn
) -> Result<Option<Person>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
	let dto_option = sqlx::query_file_as!(
		PersonDto,
		"src/db/sql_queries/persons/get/que_get_person_by_inn.sql",
		pers_inn as &PersInn
	).fetch_optional(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;

	match dto_option {
		Some(dto) => {
			let person = dto.try_into().map_err(|err: serde_json::Error| err.process_err(Status::MappingError, None))?;
			Ok(Some(person))
		},
		None => Ok(None)
	}
}

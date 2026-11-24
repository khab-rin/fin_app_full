
use crate::primitives::frozen::text::{BoxUuid, DateTime, PersInn};
use crate::sql_models::person::implements::{PersonDto, Person};
use crate::{Status, ProcessError};


pub async fn sql_insert_update_person_function<'t, E>(
	conn: E,
	person: &Person,
	is_sinc: i32
) -> Result<Person, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Sqlite>
{
	let metadata_json_value = serde_json::to_value(&person.metadata).map_err(|err: serde_json::Error| err.process_err(Status::MappingError, None) )?;
	let person_dto = sqlx::query_file_as!(
		PersonDto,
		"src/client/sql_queries/persons/add/que_insert_update.sql",
		person.pers_id,
		person.pers_inn,
		metadata_json_value,
		person.last_update,
		is_sinc
	).fetch_one(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;

	let person: Person = person_dto.try_into().map_err(|err: serde_json::Error| err.process_err(Status::MappingError, None))?;
	Ok(person)
}
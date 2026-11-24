use shared_lib::{Status, ProcessError};

use shared_lib::primitives::frozen::text::{BoxUuid, PersInn, DateTime};
use shared_lib::sql_models::person::implements::{Person, PersonDto};

pub async fn sql_insert_update_person_function<'t, E>(
	conn: E,
	person: &Person
) -> Result<Person, Status>
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
	
	let person_dto = sqlx::query_file_as!(
		PersonDto,
		"src/db/sql_queries/persons/add/que_insert_update.sql",
		*person.pers_id,
		*person.pers_inn,
		serde_json::to_value(&person.metadata).unwrap_or_default(),

	).fetch_one(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;

	let person: Person = person_dto
		.try_into()
		.map_err(|err:serde_json::Error| err.process_err(Status::MappingError, None))?;

	Ok(person)
}


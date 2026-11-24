use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{BoxUuid, Date, RubF, DocNum};
use shared_lib::sql_models::operation::account::{Account};
use shared_lib::sql_models::operation::implements::DocType;
use shared_lib::sql_models::operation::implements::Operation;

pub async fn sql_get_operation_ids_by_comp_id_function<'t, E>(
	conn: E,
	comp_id: &BoxUuid
) -> Result<Vec<BoxUuid>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Postgres> 
{	
	sqlx::query_file_scalar!(
		"src/db/sql_queries/operations/get/que_get_operation_ids_by_comp_id.sql",
		comp_id as &BoxUuid
	).fetch_all(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))

}

pub async fn sql_get_operation_by_id_function<'t, E>(
	conn: E,
	oper_id: &BoxUuid
) -> Result<Option<Operation>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
	sqlx::query_file_as!(
		Operation,
		"src/db/sql_queries/operations/get/que_get_operation_by_id.sql",
		oper_id as &BoxUuid
	).fetch_optional(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))
}
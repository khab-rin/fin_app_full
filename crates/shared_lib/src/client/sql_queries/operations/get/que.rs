

use crate::{Status, ProcessError};
use crate::primitives::frozen::text::{BoxUuid, RubF, Date, DocNum};
use crate::sql_models::operation::implements::{Operation, DocType};
use crate::sql_models::operation::account::{Account};
use crate::static_data::null_elems::NULLS;

pub async fn sql_exist_operation_by_id_que<'t, E>(
	conn: E,
	oper_id: &BoxUuid
) -> Result<bool, Status>
where E: sqlx::Executor<'t, Database = sqlx::Sqlite>
{
	sqlx::query_file_scalar!(
		"src/client/sql_queries/operations/get/que_exist_by_id.sql",
		oper_id
	).fetch_one(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))
}

pub async fn sql_get_all_operations_function<'t, E>(
	conn: E
) -> Result<Vec<Operation>, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Sqlite>
{

	let operations: Vec<Operation> = sqlx::query_file_as!(
		Operation,
		"src/client/sql_queries/operations/get/que_get_all_operations.sql"
	).fetch_all(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;

	Ok(operations.into_iter().filter(|x| x.oper_id != NULLS.id.operation_id).collect())

}
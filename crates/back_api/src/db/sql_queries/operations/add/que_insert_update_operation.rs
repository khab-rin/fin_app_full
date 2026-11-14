
use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{BoxUuid, Date, RubF, DocNum};
use shared_lib::sql_models::operation::account::{Account};
use shared_lib::sql_models::operation::implements::DocType;
use shared_lib::sql_models::operation::implements::Operation;

pub async fn que_insert_update_operation_function<'t, E>(
	conn: E,
	operation: &Operation
) -> Result<Operation, Status>
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
	let operation = sqlx::query_file_as!(
		Operation,
		"src/db/sql_queries/operations/add/que_insert_update_operation.sql",
		&operation.oper_id as &BoxUuid, 

		*operation.user_id,
		*operation.comp_id,
		*operation.ctrpty_id,

		*operation.contract_id,

		operation.debet.as_ref().to_string(),
		operation.credit.as_ref().to_string(),
		*operation.amount,
		*operation.oper_date,

		&operation.doc_type as &DocType,
		*operation.doc_num,
		*operation.doc_date,

		operation.is_storno,
		operation.is_del,

		*operation.entr_date
	).fetch_one(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	Ok(operation)
}
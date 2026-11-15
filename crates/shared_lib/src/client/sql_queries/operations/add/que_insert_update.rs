use crate::sql_models::operation::implements::{Operation, DocType};
use crate::{Status, ProcessError};

use crate::primitives::frozen::text::{BoxUuid, RubF, Date, DocNum};
use crate::sql_models::operation::account::Account;


pub async fn sql_insert_update_operation_function<'t, E>(
	conn: E,
	operation: &Operation,
	is_sync: i32
) -> Result<Operation, Status>
where E: sqlx::Executor<'t, Database = sqlx::Sqlite>
{
	let operation = sqlx::query_file_as!(
		Operation,
		"src/client/sql_queries/operations/add/que_insert_update.sql",
		operation.oper_id,
	
		operation.user_id,
		operation.comp_id,
		operation.ctrpty_id,

		operation.contract_id,

		operation.debet,
		operation.credit,
		operation.amount,
		operation.oper_date,

		operation.doc_type,
		operation.doc_num,
		operation.doc_date,

		operation.is_storno,
		operation.is_del,

		operation.entr_date,
		
		is_sync
	).fetch_one(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;


	Ok(operation)
}
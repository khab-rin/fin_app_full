use crate::{Status, ProcessError};
use crate::primitives::frozen::text::{BoxUuid};

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
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))
}
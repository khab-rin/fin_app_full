use crate::{Status, ProcessError};
use crate::sql_models::contracts::implements::Contract;
use crate::primitives::frozen::text::{BoxUuid, DocNum, Date, Currency, RubF, Integ};

pub async fn sql_get_contract_by_contr_id_fun<'t, E>(
	conn: E,
	contr_id: &BoxUuid
) -> Result<Option<Contract>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Sqlite>
{
	sqlx::query_file_as!(
		Contract,
		"src/client/sql_queries/contracts/get/que_get_by_contr_id.sql",
		contr_id
	).fetch_optional(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))
}
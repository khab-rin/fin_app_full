use crate::{Status, ProcessError};
use crate::sql_models::contracts::implements::Contract;
use crate::primitives::frozen::text::{BoxUuid, DocNum, Date, Currency, RubF, Integ};


pub async fn sql_get_contracts_by_comp_ctrpty_id_func<'t, E>(
	conn: E,
	comp_id: &BoxUuid,
	ctrpty_id: &BoxUuid
) -> Result<Vec<Contract>, Status> 
where E: sqlx::Executor <'t, Database = sqlx::Sqlite>
{
	let contracts = sqlx::query_file_as!(
		Vec<Contract>,
		"src/client/sql_queries/"
	)
}
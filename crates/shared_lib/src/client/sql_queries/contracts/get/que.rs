use crate::{Status, ProcessError};
use crate::sql_models::contracts::implements::Contract;
use crate::primitives::frozen::text::{BoxUuid, DocNum, Date, Currency, RubF, Integ};
use crate::static_data::null_elems::NULLS;


pub async fn sql_get_contracts_by_comp_ctrpty_id_func<'t, E>(
	conn: E,
	comp_id: &BoxUuid,
	ctrpty_id: &BoxUuid
) -> Result<Vec<Contract>, Status> 
where E: sqlx::Executor <'t, Database = sqlx::Sqlite>
{
	let contracts = sqlx::query_file_as!(
		Contract,
		"src/client/sql_queries/contracts/get/que_get_by_comp_ctrpty_id.sql",
		comp_id,
		ctrpty_id
	).fetch_all(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	Ok(contracts)

}

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

pub async fn sql_get_all_contracts<'t, E>(
	conn: E,
) -> Result<Vec<Contract>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Sqlite> 
{
	let contracts = sqlx::query_file_as!(
		Contract,
		"src/client/sql_queries/contracts/get/que_get_all_contracts.sql"
	).fetch_all(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?
	.into_iter()
	.filter(|x| x.contract_id != NULLS.id.contract_id)
	.collect();

	Ok(contracts)

}
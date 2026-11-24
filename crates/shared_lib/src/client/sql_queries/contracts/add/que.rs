use sqlx::Sqlite;

use crate::{Status, ProcessError};
use crate::primitives::frozen::text::{BoxUuid, DocNum, Date, Currency, RubF, Integ};
use crate::sql_models::contracts::implements::Contract;

pub async fn sql_insert_update_contract_function<'t, E>(
	conn: E,
	contract: &Contract,
	is_sync: i32
) -> Result<Contract, Status> 
where E: sqlx::Executor<'t, Database = Sqlite>
{
	let contract = sqlx::query_file_as!(
		Contract,
		"src/client/sql_queries/contracts/add/que_insert_update.sql",
		contract.contract_id,
        contract.user_id,
        contract.comp_id,
        contract.ctrpty_id,
        contract.contract_num,
        contract.contract_date,
        contract.title,
        contract.st_date,
        contract.end_date,
        contract.currency,
        contract.total_amount,
        contract.payment_deferral_days,
        contract.is_active,
        contract.descrip,
        contract.entr_date,
        contract.is_del,
		is_sync
	).fetch_one(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;

	Ok(contract)
}
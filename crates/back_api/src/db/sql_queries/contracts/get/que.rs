use shared_lib::sql_models::contracts::implements::Contract;
use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{Integ, BoxUuid, DocNum, Date, Currency, RubF};

pub async fn sql_get_contract_by_id_function<'t, E>(
    conn: E,
    contract_id: &BoxUuid
) -> Result<Option<Contract>, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
    let contract_option = sqlx::query_file_as!(
        Contract,
        "src/db/sql_queries/contracts/get/que_by_id.sql",
        contract_id as &BoxUuid
    ).fetch_optional(conn).await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    Ok(contract_option)
}

pub async fn sql_get_contract_ids_by_comp_id<'t, E>(
	conn: E,
	comp_id: &BoxUuid
) -> Result<Vec<BoxUuid>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
	sqlx::query_file_scalar!(
		"src/db/sql_queries/contracts/get/que_get_contract_ids_by_comp_id.sql",
		comp_id as &BoxUuid
	).fetch_all(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))
}
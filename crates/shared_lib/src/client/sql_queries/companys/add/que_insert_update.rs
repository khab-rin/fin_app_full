use sqlx::Sqlite;

use crate::{Status, ProcessError};
use crate::sql_models::company::implements::{Company};

pub async fn sql_insert_update_company_query<'t, E>(
    conn: E,
    company: &Company
) -> Result<(), Status> 
where E: sqlx::Executor<'t, Database = Sqlite>
{
	let metadata_str = serde_json::to_string(&company.metadata).unwrap_or_else(|_| "{}".to_string());
	sqlx::query_file!(
		"src/client/sql_queries/companys/add/que_insert_update.sql",
		company.comp_id,
		company.comp_inn,
		company.kpp,
		company.comp_type,
		company.comp_status,
		metadata_str,
		company.last_update
	).execute(conn).await
	.map_err(|err| {
		let ext_info = format!("comp_id = {}, comp_inn = {}, kpp = {}", company.comp_id, company.comp_inn, company.kpp);
		err.process_err(Status::SqlQueryWrongLogic, &ext_info)
	})?;


	Ok(())
}
use sqlx::Sqlite;

use crate::{ProcessError, Status, primitives::frozen::text::BoxUuid};

pub async fn sql_del_company_by_id_query<'t, E>(
	conn: E,
	comp_id: &BoxUuid
) -> Result<(), Status> 
where E: sqlx::Executor<'t, Database = Sqlite> 
{
	match sqlx::query_file!(
		"src/client/sql_queries/companys/del/que_by_id.sql",
		comp_id
	).execute(conn).await {
		Ok(_) => Ok(()),
		Err(err) => {
			err.process_err(Status::SqlQueryWrongLogic, "");
			Err(Status::SqlQueryWrongLogic)
		}
	}
}
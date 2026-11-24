use shared_lib::primitives::frozen::text::BoxUuid;
use shared_lib::{Status, ProcessError};


pub(crate) async fn sql_get_comp_id_by_user_id_function<'t, E>(
    conn: E,
    user_id: &BoxUuid
    
) -> Result<Option<BoxUuid>, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
	sqlx::query_file_scalar!(
		"src/db/sql_queries/users/get/que_get_comp_id_by_user_id.sql",
		user_id as &BoxUuid
	).fetch_optional(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))
}
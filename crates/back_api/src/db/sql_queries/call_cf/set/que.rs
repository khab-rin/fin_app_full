use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::BoxUuid;

pub(crate) async fn sql_insert_update_call_function<'t, E>(
    conn: E,
    user_id: &BoxUuid,
    device_id: &BoxUuid,
    external_id: &String
) -> Result<bool, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{

    let insert_res = sqlx::query_file!(
            "src/db/sql_queries/call_cf/set/que_insert_update.sql",
            user_id.as_ref(),
            device_id.as_ref(),
            external_id
        ).fetch_optional(conn)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;


    Ok(insert_res.is_some())
}

use shared_lib::sql_models::id_rules::IdRules;
use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::BoxUuid;


pub(crate) async fn sql_insert_session_fun<'t, E>(
    conn: E,
    user_id: &BoxUuid,
    device_id: &BoxUuid
) -> Result<BoxUuid, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
	let rule = IdRules::Session { user_id: user_id.clone(), device_id: device_id.clone() };
	let session_id = rule.make_stat_id();
	let new_token = BoxUuid::unchecked(uuid::Uuid::new_v4());
    let record = sqlx::query_file!(
            "src/db/sql_queries/sessions/set/que_insert_session.sql",
			session_id as BoxUuid,
            user_id as &BoxUuid,
            device_id as &BoxUuid,
			new_token as BoxUuid
        ).fetch_one(conn)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;
    Ok(record.token) 
}
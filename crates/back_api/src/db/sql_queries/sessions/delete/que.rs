use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{Email, CompInn, PersInn, Kpp};
use shared_lib::service::auth_service::implements::{TokenDeviceData, WarnEmailData};


pub(crate) async fn sql_delete_session_by_token_function<'t, E>(
    conn: E,
    payload: &TokenDeviceData
) -> Result<Vec<WarnEmailData>, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
    let &TokenDeviceData { token, .. } = &payload;
    let row = sqlx::
        query_file_as!(
            WarnEmailData,
            "src/db/sql_queries/sessions/delete/by_token_device.sql",
            token.as_ref(),
        ).fetch_all(conn)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?; 

    Ok(row)
}
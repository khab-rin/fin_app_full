use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{DateTime, BoxUuid};
use shared_lib::service::auth_service::implements::{ExternalDeviceData};

pub(crate) async fn sql_get_user_time_by_device_external_function<'t, E>(
    conn: E,
    data: &ExternalDeviceData
) -> Result<Option<(BoxUuid, DateTime)>, Status> 

where E: sqlx::Executor<'t, Database = sqlx::Postgres>{

    let ExternalDeviceData {device_id, external_id } = data;

    let record_opt = sqlx::query_file!(
        "src/db/sql_queries/call_cf/get/user_id_expires_t_by_device_id_external_id.sql",
        device_id.as_ref(),
        external_id
        ).fetch_optional(conn).await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;
        

    let result = record_opt.map(|row| {
        (row.user_id, row.expires_t) 
    });

    Ok(result)
}
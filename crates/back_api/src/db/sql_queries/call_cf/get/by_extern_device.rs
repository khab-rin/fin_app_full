use std::sync::Arc;

use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{DateTime, BoxUuid};
use shared_lib::service::auth_service::implements::{ExternalDeviceData};

use crate::config::BackApiState;

pub(crate) async fn get_user_time_by_device_external(
    state: &Arc<BackApiState>,
    data: &ExternalDeviceData
) -> Result<Option<(BoxUuid, DateTime)>, Status> {

    let ExternalDeviceData {device_id, external_id } = data;

    let record_opt = sqlx::query_file!(
        "src/db/sql_queries/call_cf/get/by_extern_device.sql",
        device_id.as_ref(),
        external_id
        ).fetch_optional(&state.pool_fast).await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;
        

    let result = record_opt.map(|row| {
        (row.user_id, row.expires_t) 
    });

    Ok(result)
}
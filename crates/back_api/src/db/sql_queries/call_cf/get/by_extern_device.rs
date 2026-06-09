use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{DateTime, BoxUuid};
use shared_lib::service::auth_service::implements::{PhoneDeviceData};

use crate::config::BackApiState;

pub(crate) async fn get_user_time_by_device_external(
    state: &Arc<BackApiState>,
    data: &PhoneDeviceData
) -> Result<(BoxUuid, DateTime), Status> {

    let PhoneDeviceData {device_id, external_id } = data;

    let record = match sqlx::
        query_file!(
            "src/db/sql_queries/call_cf/get/by_extern_device.sql",
            device_id.as_ref(),
            external_id
        ).fetch_one(&state.pool_fast).await {
            Ok(r) => r,
            Err(err) => {
                tracing::error!(
                    device_id = %device_id,
                    external_id = %external_id,
                    tech_err = ?err,
                    local_err = ?Status::SqlQueryWrongLogic,
                    "FUN get_user_time_by_device_external WRONG SQL QUERY call_cf"
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        };

    Ok((record.user_id, record.expires_t))
}
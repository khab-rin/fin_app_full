use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{Email, Inn, Kpp};
use shared_lib::service::auth_service::implements::{RestoreByTokenRequest, WarnEmailData};

use crate::config::BackApiState;

pub(crate) async fn delete_session_by_token(
    state: &Arc<BackApiState>,
    payload: &RestoreByTokenRequest
) -> Result<Vec<WarnEmailData>, Status> {

    let &RestoreByTokenRequest { token, device_id } = &payload;
    let row = sqlx::
        query_file_as!(
            WarnEmailData,
            "src/db/sql_queries/sessions/delete/by_token_device.sql",
            token.as_ref(),
            device_id.as_ref()
        ).fetch_all(&state.pool)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::SqlQueryWrongLogic
            )
        })
        .map_err(|_| Status::SqlQueryWrongLogic)?;
    
    Ok(row)
}
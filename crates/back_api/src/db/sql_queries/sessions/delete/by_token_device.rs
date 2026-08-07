use std::sync::Arc;

use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{Email, CompInn, PersInn, Kpp};
use shared_lib::service::auth_service::implements::{TokenDeviceData, WarnEmailData};

use crate::config::BackApiState;

pub(crate) async fn delete_session_by_token(
    state: &Arc<BackApiState>,
    payload: &TokenDeviceData
) -> Result<Vec<WarnEmailData>, Status> {

    let &TokenDeviceData { token, .. } = &payload;
    let row = sqlx::
        query_file_as!(
            WarnEmailData,
            "src/db/sql_queries/sessions/delete/by_token_device.sql",
            token.as_ref(),
        ).fetch_all(&state.pool_fast)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    
    Ok(row)
}
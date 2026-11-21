use std::sync::Arc;

use shared_lib::{ProcessError, Status};
use shared_lib::service::auth_service::implements::{TokenDeviceData, WarnEmailData};

use crate::config::BackApiState;
use crate::db::sql_queries::sessions::delete::que::sql_delete_session_by_token_function;
use crate::db::service::auth_service::send_warn_mail::send_warn_mail;

pub(crate) async  fn delete_warn_token_device(
    state: &Arc<BackApiState>,
    payload: &TokenDeviceData
) -> Result<Status, Status> {

    let warn_vec: Vec<WarnEmailData> = sql_delete_session_by_token_function(&state.pool_fast, payload).await
        .map_err(|err| err.process_err(err, ""))?;

    for warn_data in warn_vec {
        let state_clone = state.clone();
        tokio::spawn(async move {
            let _ = send_warn_mail(state_clone, warn_data).await;
        });
    }

    Ok(Status::Success)
}
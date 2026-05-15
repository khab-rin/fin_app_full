use std::sync::Arc;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{RestoreByTokenRequest, WarnEmailData};

use crate::config::ApiState;
use crate::db::sql_queries::sessions::delete::by_token_device::delete_session_by_token;
use crate::db::service::auth_service::send_warn_mail::send_warn_mail;

pub(crate) async  fn delete_warn_token_device(
    state: &Arc<ApiState>,
    payload: &RestoreByTokenRequest
) -> Result<Status, Status> {

    let warn_vec: Vec<WarnEmailData> = delete_session_by_token(state, payload).await?;

    for warn_data in warn_vec {
        let state_clone = state.clone();
        tokio::spawn(async move {
            let _ = send_warn_mail(state_clone, warn_data).await;
        });
    }

    Ok(Status::Success)
}
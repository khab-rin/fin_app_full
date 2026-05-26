use std::sync::Arc;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{TokenDeviceData, WarnEmailData};

use crate::config::BackApiState;
use crate::db::sql_queries::sessions::delete::by_token_device::delete_session_by_token;
use crate::db::service::auth_service::send_warn_mail::send_warn_mail;

pub(crate) async  fn delete_warn_token_device(
    state: &Arc<BackApiState>,
    payload: &TokenDeviceData
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
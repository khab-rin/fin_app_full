use shared_lib::Status;
use shared_lib::service::mchd::service::MchdStep;

use crate::state::ClientState;
use crate::service::mchd::check_mchd::{
    check_user_mchd_tax,
    check_user_mchd_home
};

#[tauri::command]
pub async fn cmd_check_user_mchd_tax(
    state: tauri::State<'_, ClientState>
) -> Result<MchdStep, Status> {

    check_user_mchd_tax(&state).await
}


#[tauri::command]
pub async fn cmd_check_user_mchd_home(
    state: tauri::State<'_, ClientState>
) -> Result<MchdStep, Status> {

    check_user_mchd_home(&state).await
}
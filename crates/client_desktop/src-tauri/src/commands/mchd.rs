use shared_lib::Status;
use shared_lib::service::mchd::service::MchdStep;
use shared_lib::service::mchd::tax_mchd::{MchdTaxFields, MchdPowerInfo};

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

#[tauri::command]
pub fn cmd_get_tax_powers(
) -> Vec<MchdTaxFields> {
    MchdTaxFields::get_all_powers()
}

#[tauri::command]
pub fn cmd_get_power_info(
    power: MchdTaxFields
) -> MchdPowerInfo {
    power.get_power_info().clone()
}
use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, NewMchdData};
use shared_lib::service::mchd::tax_mchd::{MchdTaxFields, MchdPowerInfo};

use crate::state::ClientState;
use crate::service::mchd::new_tax_mchd::make_new_tax_mchd;

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
    log::info!("cmd_get_power_info running!");
    MchdTaxFields::get_all_powers()
}

#[tauri::command]
pub fn cmd_get_power_info(
    power: MchdTaxFields
) -> MchdPowerInfo {
    log::info!("cmd_get_power_info running!");
    power.get_power_info().clone()
}

#[tauri::command]
pub async fn cmd_register_tax_mchd(
    state: tauri::State<'_, ClientState>,
    new_mchd_data: NewMchdData
) -> Result<MchdStep, Status> {

    log::info!("cmd_register_tax_mchd running!!!");

    make_new_tax_mchd(&state, &new_mchd_data).await


}
use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, NewMchdData};
use shared_lib::service::mchd::home_mchd_power::{HomeMchdPower, HomePowerInfo};

use crate::state::ClientState;
use crate::service::mchd::tax_mchd::new_tax_mchd::make_new_tax_mchd;
use crate::service::mchd::lend_mchd::lend_mchd::lend_mchd_to_back_api_for_register;


#[tauri::command]
pub fn cmd_get_tax_powers(
) -> Vec<HomeMchdPower> {
    log::info!("cmd_get_tax_powers running!");
    HomeMchdPower::get_all_tax_powers()
}

#[tauri::command]
pub fn cmd_get_btb_powers(
) -> Vec<HomeMchdPower> {
    log::info!("cmd_get_tax_powers running!");
    HomeMchdPower::get_all_btb_powers()
}

#[tauri::command]
pub fn cmd_get_home_powers(
) -> Vec<HomeMchdPower> {
    log::info!("cmd_get_tax_powers running!");
    HomeMchdPower::get_all_home_powers()
}

#[tauri::command]
pub fn cmd_get_power_info(
    power: HomeMchdPower
) -> HomePowerInfo {
    
    power.get_power_info().clone()
}

#[tauri::command]
pub async fn cmd_register_tax_mchd(
    state: tauri::State<'_, ClientState>,
    data: NewMchdData
) -> Result<MchdStep, Status> {

    log::info!("cmd_register_tax_mchd running!!!");

    make_new_tax_mchd(&state, &data).await

}

#[tauri::command]
pub async fn cmd_lend_mchd(
    state: tauri::State<'_, ClientState>,
    xml_file_path: String,
    sig_file_path: String
) -> Result<MchdStep, Status> {

    log::info!("cmd_lend_mchd running!!!");

    lend_mchd_to_back_api_for_register(&state, &xml_file_path, &sig_file_path).await

}
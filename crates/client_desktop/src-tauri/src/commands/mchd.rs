use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, NewMchdData};
use shared_lib::service::mchd::tax_mchd::{MchdTaxFields, MchdPowerInfo};

use crate::state::ClientState;
use crate::service::mchd::tax_mchd::new_tax_mchd::make_new_tax_mchd;


#[tauri::command]
pub fn cmd_get_tax_powers(
) -> Vec<MchdTaxFields> {
    log::info!("cmd_get_tax_powers running!");
    MchdTaxFields::get_all_powers()
}

#[tauri::command]
pub fn cmd_get_power_info(
    power: MchdTaxFields
) -> MchdPowerInfo {
    
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


    Err(Status::Unknown)
}
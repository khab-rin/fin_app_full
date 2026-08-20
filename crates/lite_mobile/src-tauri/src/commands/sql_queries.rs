
use shared_lib::client::sql_queries::contracts::get::contract_by_ids::get_contracts_by_comp_ctrpty_ids;
use shared_lib::sql_models::company::implements::Company;
use shared_lib::sql_models::contracts::implements::{Contract, NewContrData};
use shared_lib::{ClientState, ProcessError, Status};
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::primitives::frozen::text::{RasAcc, Bic, CompInn, Kpp, BoxUuid};

use shared_lib::client::sql_queries::companys::get::bank_acc_by_comp_inn_kpp::get_bank_accs_by_comp_id;
use shared_lib::client::sql_queries::companys::add::new_company::add_company_by_inn_cpp_acc;
use shared_lib::client::sql_queries::companys::get::by_inn_kpp::get_company_by_inn_kpp;
use shared_lib::client::sql_queries::contracts::add::new_contract::make_new_contract;

#[tauri::command]
pub async fn cmd_add_comp_bank_acc(
    state: tauri::State<'_, ClientState>,
    comp_inn: Option<CompInn>,
    kpp: Option<Kpp>,
    bic: Option<Bic>,
    ras_acc: Option<RasAcc>
) -> Result<Vec<RasBicAcc>, Status> {

    let ras_bic_acc = if let (Some(b), Some(r)) = (bic, ras_acc) {
        let acc = match RasBicAcc::new(b, r) {
            Ok(r) => r,
            Err(err) => {
                return Err(err.process_err(err, ""));
            }
        };
        Some(acc)
    } else {
        None
    };

    let company_option = add_company_by_inn_cpp_acc(
            &state, 
            &comp_inn, 
            &kpp, 
            &ras_bic_acc)
        .await
        .map_err(|err| err.process_err(err, ""))?; 

    match company_option {
        Some(c) => Ok(c.metadata.bank_acc),
        None => Ok(vec!())
    }

}


#[tauri::command]
pub async fn cmd_get_comp_bank_accs(
    state: tauri::State<'_, ClientState>,
    comp_inn: Option<CompInn>,
    kpp: Option<Kpp> 
) -> Result<Vec<RasBicAcc>, Status> {

    get_bank_accs_by_comp_id(&state, &comp_inn, &kpp).await
    
}


#[tauri::command]
pub async fn cmd_get_comp_by_inn_kpp(
    state: tauri::State<'_, ClientState>,
    comp_inn: CompInn,
    kpp: Kpp
) -> Result<Option<Company>, Status> {


    log::info!("cmd_get_comp_by_inn_kpp running");
    
    get_company_by_inn_kpp(&state, &comp_inn, &kpp).await
}

#[tauri::command]
pub async fn cmd_add_new_contract(
    state: tauri::State<'_, ClientState>,
    data: NewContrData
) -> Result<Vec<Contract>, Status> {
    let ctrpty_id = data.ctrpty_id.clone();

    make_new_contract(&state, data).await
        .map_err(|err| err.process_err(err, ""))?;

    get_contracts_by_comp_ctrpty_ids(&state, &ctrpty_id).await

}

#[tauri::command]
pub async fn cmd_get_contracts_by_ctrpty_id(
    state: tauri::State<'_, ClientState>,
    ctrpty_id: BoxUuid
) -> Result<Vec<Contract>, Status> {

    get_contracts_by_comp_ctrpty_ids(&state, &ctrpty_id).await

}
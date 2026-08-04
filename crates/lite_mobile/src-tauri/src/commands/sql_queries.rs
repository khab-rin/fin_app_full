
use shared_lib::{Status, ClientState};
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::primitives::frozen::text::{RasAcc, Bic, CompInn, Kpp};

use shared_lib::client::operation::helper::add_bank_acc_by_inn_kpp;
use shared_lib::client::sql_queries::companys::get::bank_acc_by_comp_inn_kpp::get_bank_accs_by_comp_id;

#[tauri::command]
pub async fn cmd_add_comp_bank_acc(
    state: tauri::State<'_, ClientState>,
    comp_inn: Option<CompInn>,
    kpp: Option<Kpp>,
    bic: Option<Bic>,
    ras_acc: Option<RasAcc>
) -> Result<Vec<RasBicAcc>, Status> {

    log::info!("cmd_add_comp_bank_acc running");

    let ras_bic_acc = if let (Some(b), Some(r)) = (bic, ras_acc) {
        let acc = match RasBicAcc::new(b, r) {
            Ok(r) => r,
            Err(err) => {
                log::error!(
                    "local_err = {:?}, FUN cmd_input_own_ras_bic_acc FAILED BY MAPPING RasBicAcc", err
                );
                return Err(err);
            }
        };
        Some(acc)
    } else {
        None
    };

    add_bank_acc_by_inn_kpp(&state, &comp_inn, &kpp, &ras_bic_acc).await
}


#[tauri::command]
pub async fn cmd_get_comp_bank_accs(
    state: tauri::State<'_, ClientState>,
    comp_inn: Option<CompInn>,
    kpp: Option<Kpp> 
) -> Result<Vec<RasBicAcc>, Status> {

    get_bank_accs_by_comp_id(&state, &comp_inn, &kpp).await
    
}


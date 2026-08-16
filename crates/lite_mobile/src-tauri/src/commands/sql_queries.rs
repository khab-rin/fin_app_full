
use shared_lib::sql_models::company::implements::Company;
use shared_lib::{ClientState, ProcessError, Status};
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::primitives::frozen::text::{RasAcc, Bic, CompInn, Kpp};

use shared_lib::client::sql_queries::companys::get::bank_acc_by_comp_inn_kpp::get_bank_accs_by_comp_id;
use shared_lib::client::sql_queries::companys::add::new_company::add_company_by_inn_cpp_acc;
use shared_lib::client::sql_queries::companys::get::by_inn_kpp::get_company_by_inn_kpp;

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

pub async fn cmd_get_comp_by_inn_kpp(
    state: tauri::State<'_, ClientState>,
    comp_inn: CompInn,
    kpp: Kpp
) -> Result<Option<Company>, Status> {

    get_company_by_inn_kpp(&state, &comp_inn, &kpp).await

}

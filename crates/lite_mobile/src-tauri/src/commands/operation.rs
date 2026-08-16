
use shared_lib::primitives::frozen::text::{CompInn, Kpp};
use shared_lib::sql_models::company::implements::Company;
use shared_lib::{Status, ClientState};
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::sql_models::operation::service::OperationStep;


use crate::service::operation::make_bank_statement_operations;


#[tauri::command]
pub async fn cmd_load_bank_statement(
    state: tauri::State<'_, ClientState>,
    ras_bic_acc: RasBicAcc,
    path: String
) -> Result<OperationStep, Status> {
    make_bank_statement_operations(&state, &ras_bic_acc, &path).await
}




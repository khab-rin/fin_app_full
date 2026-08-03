use shared_lib::{Status, ClientState};
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::sql_models::operation::service::OperationStep;



#[tauri::command]
pub async fn cmd_load_bank_statement(
    state: tauri::State<'_, ClientState>,
    ras_bic_acc: RasBicAcc,
    path: String
) -> Result<OperationStep, Status> {
    Err(Status::Unknown)
}


use shared_lib::sql_models::operation::implements::DocType;
use shared_lib::{Status, ClientState};
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::sql_models::operation::service::OperationStep;

use shared_lib::primitives::frozen::macros::RussEnumName; 


use crate::service::operation::make_bank_statement_operations;


#[tauri::command]
pub async fn cmd_load_bank_statement(
    state: tauri::State<'_, ClientState>,
    ras_bic_acc: RasBicAcc,
    path: String
) -> Result<OperationStep, Status> {
    make_bank_statement_operations(&state, &ras_bic_acc, &path).await
}

#[tauri::command]
pub async fn cmd_get_doc_type_russ_name
(
    state: tauri::State<'_, ClientState>,
    doc_type: DocType
) -> Result<String, Status> {
    

    
    Ok(doc_type.russian_enum_name())
}




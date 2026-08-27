

use shared_lib::primitives::frozen::text::{BoxUuid, DocNum, RubF, Date};
use shared_lib::sql_models::operation::implements::{DocType, make_oper_id, Operation};
use shared_lib::{ClientState, ProcessError, Status};
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::sql_models::operation::service::{OperationInfo, OperationStep};

use shared_lib::primitives::frozen::macros::RussEnumName; 
use shared_lib::sql_models::operation::account::Account;
use shared_lib::client::sql_queries::operations::get::exist_id_by_id::get_exist_id_by_id;
use shared_lib::client::sql_queries::operations::add::many::add_new_operations;

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
    doc_type: DocType
) -> Result<String, Status> {
    
    Ok(doc_type.russian_enum_name())
}

#[tauri::command]
pub fn cmd_is_accounts_compatible(
    left_acc: Account,
    rigth_acc: Account
) -> bool {

    for acc in left_acc.get_correspondents() {
        if *acc == rigth_acc {
            return true;
        }
    }
    false
}

#[tauri::command]
pub async fn cmd_is_operation_exist(
    state: tauri::State<'_, ClientState>,
    doc_num: Option<DocNum>, 
    oper_date: Option<Date>, 
    amount: Option<RubF>, 
    ctrpty_id: Option<BoxUuid>
) -> Result<(BoxUuid, bool), Status> {

    let check_oper_id = make_oper_id(&doc_num, &oper_date, &amount, &ctrpty_id);

    match get_exist_id_by_id(&state, &check_oper_id).await
        .map_err(|err| err.process_err(err, ""))? {
            Some(id) => Ok((id, true)),
            None => Ok((check_oper_id, false))
        }
}

#[tauri::command]
pub fn cmd_get_acc_info(
	state: tauri::State<'_, ClientState>,
	account: Account
) -> Result<String, Status> {
	Ok(account.info())
}

#[tauri::command]
pub async  fn cmd_process_operations(
	state: tauri::State<'_, ClientState>,
	option_operations: Vec<Option<Operation>>
) -> Result<OperationStep, Status> {
	let operations: Vec<Operation> = option_operations.into_iter().flatten().collect();

	let count = operations.len() as i32;

	if let Err(err) = add_new_operations(&state, operations).await {
		err.process_err(err, "");
		return Ok(OperationStep::TryLater { text: OperationInfo::ClientApiSystemError });
	}

	let step = OperationStep::ProcessSuccess { text: OperationInfo::StatementSuccess, count };

	Ok(step)
}



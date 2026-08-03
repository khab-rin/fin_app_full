
use shared_lib::{Status, ClientState};
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::primitives::svelte_validate::SvelteValidator;
use shared_lib::client::operation::statement_parser::parser::parse_statement;


#[tauri::command]
pub async fn cmd_process_bank_statement(
    state: tauri::State<'_, ClientState>,
    ras_bic_acc: RasBicAcc,
    path: String
) -> Result<(), Status> {

    log::info!("cmd_process_bank_statement running");

    match parse_statement(&state, &ras_bic_acc, &path).await {
        Ok(_) => {
            Ok(())
        }
        Err(err) => Err(err)
    }
}



#[tauri::command]
pub fn cmd_validate_field(
    type_value: SvelteValidator,
    value: String
) -> Result<bool, Status> {
    type_value.validate_svelte_field(&value)
}

#[tauri::command]
pub async fn get_own_bank_accs(
    state: tauri::State<'_, ClientState> 
) -> Result<Vec<RasBicAcc>, Status> {
    

    Err(Status::Unknown)
}









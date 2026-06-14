use shared_lib::Status;
use crate::state::ClientState;
use shared_lib::primitives::svelte_validate::SvelteValidator;
use crate::service::process::bank_statement::proceed::process_statement;


#[tauri::command]
pub async fn cmd_process_bank_statement(
    state: tauri::State<'_, ClientState>,
    path: String
) -> Result<(), Status> {

    log::info!("cmd_process_bank_statement running");

    match process_statement(&state, path).await {
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









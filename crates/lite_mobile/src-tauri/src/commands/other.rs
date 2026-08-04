use shared_lib::Status;
use shared_lib::primitives::svelte_validate::SvelteValidator;



#[tauri::command]
pub fn cmd_validate_field(
    type_value: SvelteValidator,
    value: String
) -> Result<bool, Status> {
    type_value.validate_svelte_field(&value)
}

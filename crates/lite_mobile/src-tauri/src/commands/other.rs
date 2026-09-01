use shared_lib::primitives::frozen::text::{BoxUuid, Date};
use shared_lib::{ClientState, Status, ProcessError};
use shared_lib::primitives::svelte_validate::SvelteValidator;


#[tauri::command]
pub fn cmd_get_today(
) -> Result<Date, Status> {
	let res = Date::unchecked(chrono::Local::now().date_naive());
	Ok(res)
}


#[tauri::command]
pub async fn cmd_get_user_comp_ids(
	state: tauri::State<'_, ClientState>
) -> Result<(BoxUuid, BoxUuid), Status> {

	let session = state.get_session().await
		.map_err(|err| err.process_err(err, ""))?;

	let user_id = session.session_user.user.user_id.clone();
	let comp_id = session.session_user.company.comp_id.clone();

	Ok((user_id, comp_id))
}


#[tauri::command]
pub fn cmd_validate_field(
    type_value: SvelteValidator,
    value: String
) -> Result<bool, Status> {
    type_value.validate_svelte_field(&value)
}



use shared_lib::{ClientState, Status};
use tauri::Emitter;

#[tauri::command]
pub async fn cmd_sync_data_base(
	state: tauri::State<'_, ClientState>
) -> Result<Status, Status> {
	state.app_handle.emit("синхронизация контрагентов", 0).ok();

	state.app_handle.emit("синхронизация физ. лиц", 25).ok();

	state.app_handle.emit("синхронизация договоров", 50).ok();

	state.app_handle.emit("синхронизация операций", 75).ok();


	state.app_handle.emit("синхронизация завершена", 100).ok();

	Ok(Status::Success)
}



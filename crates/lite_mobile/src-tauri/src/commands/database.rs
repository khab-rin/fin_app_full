use tauri::Emitter;

use shared_lib::{ClientState, Status, ProcessError};
use shared_lib::service::database::service::{DatabaseInfo, DatabaseStep};

use shared_lib::client::sql_queries::companys::sync::sync_all_companys;

#[tauri::command]
pub async fn cmd_sync_data_base(
	state: tauri::State<'_, ClientState>
) -> Result<DatabaseStep, Status> {
	state.app_handle.emit("синхронизация контрагентов", 0).ok();


	state.app_handle.emit("синхронизация физ. лиц", 25).ok();

	state.app_handle.emit("синхронизация договоров", 50).ok();

	state.app_handle.emit("синхронизация операций", 75).ok();


	state.app_handle.emit("синхронизация завершена", 100).ok();

	let step = sync_all_companys(&state).await?;
	if let DatabaseStep::SyncSuccess { .. } = step {
		return Ok(DatabaseStep::Loading { text: DatabaseInfo::SqlSyncSuccess  });
	}

	Ok(DatabaseStep::TryLater { text: DatabaseInfo::Notning })
}
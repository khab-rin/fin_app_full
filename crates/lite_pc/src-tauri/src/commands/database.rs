use tauri::Emitter;

use shared_lib::{ClientState, ProcessError, Status};
use shared_lib::service::database::service::{DatabaseInfo, DatabaseStep};
use shared_lib::service::back_communication::implements::ProgressBar;

use shared_lib::client::sql_queries::companys::sync::sync_all_companys;
use shared_lib::client::sql_queries::persons::sync::sync_all_persons;
use shared_lib::client::sql_queries::contracts::sync::sync_all_contracts;
use shared_lib::client::sql_queries::operations::sync::sync_all_operations;




#[tauri::command]
pub async fn cmd_sync_data_base(
	state: tauri::State<'_, ClientState>
) -> Result<DatabaseStep, Status> {

	let mut failed_result = DatabaseStep::TryLater {text: DatabaseInfo::ClientServiceError};

	let emit_progress = |label: &'static str, percent: u8| {
		let a: Result<(), tauri::Error> = state.app_handle.emit("db_sync_progress", ProgressBar {label, percent});
	};

	emit_progress("Синхронизация контрагентов...", 0);

	if let Err(err) = sync_all_companys(&state).await {
		err.process_err(err,"");
		failed_result.process_info(&err);
        return Ok(failed_result);
	}

	emit_progress("Синхронизация физ. лиц...", 25);

	if let Err(err) = sync_all_persons(&state).await {
		err.process_err(err,"");
		failed_result.process_info(&err);
		return Ok(failed_result);
	}

	emit_progress("Синхронизация договоров...", 50);

	if let Err(err) = sync_all_contracts(&state).await {
		err.process_err(err,"");
		failed_result.process_info(&err);
		return Ok(failed_result);
	}

	emit_progress("Синхронизация операций...", 75);

	if let Err(err) = sync_all_operations(&state).await {
		err.process_err(err,"");
		failed_result.process_info(&err);
		return Ok(failed_result);
	}

	emit_progress("Синхронизация завершена", 100);

	Ok(DatabaseStep::Loading { text: DatabaseInfo::SqlSyncSuccess  })
}
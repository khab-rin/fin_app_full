use tauri::Emitter;

use shared_lib::{ClientState, ProcessError, Status};
use shared_lib::service::database::service::{DatabaseInfo, DatabaseStep};
use shared_lib::service::back_communication::implements::ProgressBar;
use shared_lib::service::mchd::home_mchd_power::{HomeMchdPower::H1110, HomeMchdPower::H1111};

use shared_lib::client::sql_queries::companys::sync::sync_all_companys;
use shared_lib::client::sql_queries::persons::sync::sync_all_persons;
use shared_lib::client::sql_queries::contracts::sync::sync_all_contracts;
use shared_lib::client::sql_queries::operations::sync::sync_all_operations;
use shared_lib::client::mchd::show_powers::check_access;


#[tauri::command]
pub async fn cmd_sync_data_base(
	state: tauri::State<'_, ClientState>
) -> Result<DatabaseStep, Status> {

	match check_access(&state, vec![H1110, H1111]).await {
		Ok(true) => {},
		Ok(false) => { return Ok(DatabaseStep::TryLater { text: DatabaseInfo::AccessDenied });}
		Err(err) => {
			err.process_err(err, None);
			return Ok(DatabaseStep::from_err(&err));
		}
	}

	let emit_progress = |label: &'static str, percent: u8| {
		let _ = state.app_handle.emit("db_sync_progress", ProgressBar {label, percent});
	};

	emit_progress("Синхронизация контрагентов...", 0);

	if let Err(err) = sync_all_companys(&state).await {
		err.process_err(err, None);
		return Ok(DatabaseStep::from_err(&err));
	}

	emit_progress("Синхронизация физ. лиц...", 25);

	if let Err(err) = sync_all_persons(&state).await {
		err.process_err(err, None);
		return Ok(DatabaseStep::from_err(&err));
	}

	emit_progress("Синхронизация договоров...", 50);

	if let Err(err) = sync_all_contracts(&state).await {
		err.process_err(err, None);
		return Ok(DatabaseStep::from_err(&err));
	}

	emit_progress("Синхронизация операций...", 75);

	if let Err(err) = sync_all_operations(&state).await {
		err.process_err(err, None);
		return Ok(DatabaseStep::from_err(&err));
	}

	emit_progress("Синхронизация завершена", 100);

	Ok(DatabaseStep::Loading { text: DatabaseInfo::SqlSyncSuccess  })
}
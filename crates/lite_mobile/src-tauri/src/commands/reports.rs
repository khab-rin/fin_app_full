use shared_lib::{ClientState, Status, ProcessError};
use shared_lib::service::reports::service::FnsReportType;

#[tauri::command]
pub async fn cmd_get_all_all_fns_report_types(
) -> Result<Vec<FnsReportType>, Status> {
	Ok(FnsReportType::get_all_fns_report_types())
}
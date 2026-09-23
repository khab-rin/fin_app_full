use shared_lib::primitives::frozen::text_base::Digits4_4;
use shared_lib::{ClientState, Status, ProcessError};
use shared_lib::service::reports::service::{FnsReportType, ReportStep};

use shared_lib::client::reports::fns::decl_usn_6::make_decl_6_files;
use shared_lib::client::reports::fns::decl_usn_15::make_decl_15_files;
use shared_lib::client::reports::fns::notif_usn_6::make_notif_usn_6_files;
use shared_lib::client::reports::fns::notif_usn_15::make_notif_15_files;


#[tauri::command]
pub async fn cmd_get_all_fns_report_types(
) -> Result<Vec<FnsReportType>, Status> {
	Ok(FnsReportType::get_all_fns_report_types())
}

#[tauri::command]
pub async fn cmd_make_fns_report_files(
	state: tauri::State<'_, ClientState>,
	report_type: FnsReportType,
	year: i32,
	quat: i32,
	fns_code: Digits4_4
) -> Result<ReportStep, Status> {

	match report_type {
		FnsReportType::UsnNotifSix => {
			return make_notif_usn_6_files(&state, year, quat, fns_code).await.map_err(|err| err.process_err(err, ""))
		},
		FnsReportType::UsnNotifFifteen => {
			return make_notif_15_files(&state, year, quat, fns_code).await.map_err(|err| err.process_err(err, ""))
		}
		FnsReportType::UsnDeclSix => {
			return make_decl_6_files(&state, year, quat, fns_code).await.map_err(|err| err.process_err(err, ""))
		}
		FnsReportType::UsnDeclFifteen => {
			return make_decl_15_files(&state, year, quat, fns_code).await.map_err(|err| err.process_err(err, ""))
		}
	}
}
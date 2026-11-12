use shared_lib::{ProcessError, Status};
use shared_lib::service::reports::service::{ReportStep, ReportInfo};
use shared_lib::sql_models::company::implements::Company;

use crate::config::BackApiState;
use crate::db::sql_queries::companys::get::que_option_by_id::get_company_option_by_id;
use crate::db::sql_queries::companys::add::que_insert_update::sql_insert_update_company_function;
use crate::db::parsers::dadata::get_new_company::get_dadata_company;

pub async fn sql_company_sync(
	state: &BackApiState,
	load: &ReportStep
) -> Result<ReportStep, Status> {

	let mut failed_result = ReportStep::TryLater { text: ReportInfo::BackApiError };
	
	let local_companys = match load {
		ReportStep::SyncCompanysToServer {companys,} => {
			companys
		},
		_ => {
			Status::Tech.process_err(Status::SystemLogicErr, "");
			return Ok(failed_result);
		}
	};

	let mut tx = match state.pool_fast.acquire().await {
		Ok(t) => t,
		Err(err) => {
			let _ = err.process_err(Status::SqLitePoolErr, "ext_info");
			return Ok(failed_result);
		}
	};

	let mut del_companys: Vec<Company> = vec![];
	let mut server_companys: Vec<Company> = vec![];


	for local_company in local_companys {
		let comp_id = &local_company.comp_id;
		let server_company_option = match get_company_option_by_id(&mut *tx, comp_id).await {
			Ok(Some(o)) => Some(o),
			Ok(None) => {
				let comp_inn = &local_company.comp_inn;
				let kpp = &local_company.kpp;
				match get_dadata_company(state, comp_inn, kpp).await {
					Ok(Some(c)) => Some(c),
					Ok(None) => None,
					Err(err) => {
						err.process_err(err, "");
						return Ok(failed_result);
					}
				}
			}
			Err(err) => {
				err.process_err(err, "");
				return Ok(failed_result);
			}
		};
		

		if let Some(mut server_company) = server_company_option {
			server_company.merge_acc(local_company);
			if let Err(err) = sql_insert_update_company_function(&mut *tx, &server_company).await {
				if err == Status::SqlSyncError && let ReportStep::TryLater {text, ..} = &mut failed_result {
					*text = ReportInfo::SqlNeedSync;
				}
				return Ok(failed_result);
			}
			server_companys.push(server_company);
		} else {
			del_companys.push(local_company.clone());
		}

	}

	Ok(ReportStep::SyncCompanysToClient { del_companys, server_companys })

}
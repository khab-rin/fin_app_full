
use crate::client::sql_queries::companys::add::que_insert_update::sql_insert_update_company_query;
use crate::{ClientState, ProcessError, Status};
use crate::service::api_routes::implements::ApiRoutes;
use crate::service::reports::service::{ReportStep, ReportInfo};
use crate::sql_models::company::implements::Company;

use crate::client::sql_queries::companys::get::all::get_all_companys;
use crate::client::sql_queries::companys::del::que_by_id::sql_del_company_by_id_query;
use crate::client::back_api::post_query::post_query_back_api;

pub async fn sync_all_companys(
	state: &ClientState
) -> Result<ReportStep, Status> {

	let mut failed_result = ReportStep::TryLater { text: ReportInfo::ClientServiceError };

	let session = match state.get_session().await {
		Ok(s) => s,
		Err(err) => {
			err.process_err(err, "");
			return Ok(failed_result);
		}
	};

	let mut local_companys = match get_all_companys(state).await {
		Ok(a) => a,
		Err(err) => {
			err.process_err(err, "");
			return Ok(failed_result);
		}
	};

	let mut tx = match session.local_db.acquire().await {
		Ok(t) => t,
		Err(err) => {
			err.process_err(Status::SystemErr, "");
			return Ok(failed_result);
		}
	};

	while !local_companys.is_empty() {
		let mut companys: Vec<Company> = vec![];
		while companys.len() < 30 && let Some(c) = local_companys.pop() {
			companys.push(c);
		}
		let step_to_serv = ReportStep::SyncCompanysToServer { companys };
		let response = match post_query_back_api(state, state.config.get_std_client(), ApiRoutes::SqlSync, &step_to_serv).await {
			Ok(r) => r,
			Err(err) => {
				if err == Status::ConnectProblems && let ReportStep::TryLater{text} = &mut failed_result {
					*text = ReportInfo::ConnectionError;
				}
				return Ok(failed_result);
			}
		};

		let step_to_client: ReportStep = match response.json().await {
			Ok(s) => s,
			Err(err) => {
				err.process_err(Status::MappingError, "");
				return Ok(failed_result);
			}
		};

		if let ReportStep::SyncCompanysToClient { del_companys, server_companys } = step_to_client {
			for del_company in del_companys {
				if let Err(err) = sql_del_company_by_id_query(&mut *tx, &del_company.comp_id).await {
					err.process_err(err, "");
					return Ok(failed_result);
				}
			}

			for server_company in server_companys {
				if let Err(err) =  sql_insert_update_company_query(&mut *tx, &server_company).await {
					err.process_err(err, "");
					return Ok(failed_result);
				}
			}

		} else {
			Status::Tech.process_err(Status::SystemLogicErr, "");
			return Ok(failed_result);
		}
	}

	Ok(ReportStep::SyncSuccess {text: ReportInfo::Notning})

}
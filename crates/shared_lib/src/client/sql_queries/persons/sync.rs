use crate::{Status, ClientState, ProcessError};
use crate::service::reports::service::{ReportInfo, ReportStep};
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::back_api::post_query::post_query_back_api;
use crate::client::sql_queries::persons::get::que::sql_get_all_persons_function;

pub async fn sync_all_persons(
	state: &ClientState
) -> Result<ReportStep, Status> {
	let failed_result = ReportStep::TryLater { text: ReportInfo::ClientServiceError };

	let session = match state.get_session().await {
		Ok(s) => s,
		Err(err) => {
			err.process_err(err, "");
			return Ok(failed_result);
		}
	};

	let local_persons = match sql_get_all_persons_function(&session.local_db).await {
		Ok(l) => l,
		Err(err) => {
			err.process_err(err, "");
			return Ok(failed_result);
		}
	};

	let step_to_serv = ReportStep::SyncPersons { 
		text: ReportInfo::Notning, 
		persons: local_persons, 
		user_id: session.session_user.user.user_id.clone()
	};

	let response = match post_query_back_api(
		state, 
		state.config.get_sql_long(), 
		ApiRoutes::SqlSync, 
		&step_to_serv
	).await {
		Ok(r) => r,
		Err(err) => {
			err.process_err(err, "ext_info");
			return Ok(failed_result);
		}
	}; 

	if response.status().is_success() {
		Ok(ReportStep::SyncSuccess { text: ReportInfo::Notning })
	} else {
		Ok(failed_result)
	}

}
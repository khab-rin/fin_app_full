use crate::{Status, ClientState, ProcessError};
use crate::service::database::service::{DatabaseInfo, DatabaseStep};
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::back_api::post_query::post_query_back_api;
use crate::client::sql_queries::persons::get::que::sql_get_all_persons_function;

pub async fn sync_all_persons(
	state: &ClientState
) -> Result<DatabaseStep, Status> {
	let mut failed_result = DatabaseStep::TryLater { text: DatabaseInfo::ClientServiceError };

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

	let step_to_serv = DatabaseStep::SyncPersons { 
		text: DatabaseInfo::Notning, 
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
			failed_result.process_info(&err);
            return Ok(failed_result);
		}
	}; 

	if response.status().is_success() {
		Ok(DatabaseStep::SyncSuccess { text: DatabaseInfo::Notning })
	} else {
		Ok(failed_result)
	}

}
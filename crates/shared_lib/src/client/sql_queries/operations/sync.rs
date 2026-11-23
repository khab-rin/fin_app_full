use crate::{Status, ProcessError, ClientState};

use crate::service::database::service::{DatabaseInfo, DatabaseStep};
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::back_api::post_query::post_query_back_api;
use crate::client::sql_queries::operations::get::que::sql_get_all_operations_function;
use crate::client::sql_queries::operations::add::que::sql_insert_update_operation_function;

pub async fn sync_all_operations(
	state: &ClientState
) -> Result<DatabaseStep, Status> {

	let failed_result: DatabaseStep = DatabaseStep::TryLater { text: DatabaseInfo::ClientServiceError };

	let session = match state.get_session().await {
		Ok(s) => s,
		Err(err) => {
			err.process_err(err, "");
			return Ok(failed_result);
		}
	};

	let mut tx = match session.local_db.begin().await {
		Ok(c) => c,
		Err(err) => {
			err.process_err(Status::SqlPoolErr, "");
			return Ok(failed_result);
		}
	};

	let local_operations = match sql_get_all_operations_function(&mut *tx).await {
		Ok(l) => l,
		Err(err) => {
			err.process_err(err, "");
			return Ok(failed_result)
		}
	};

	let step_to_serv = DatabaseStep::SyncOperations { 
		text: DatabaseInfo::Notning, 
		operations: local_operations, 
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
			return Ok(DatabaseStep::from_err(&err));
		}
	};

	let step_to_client:DatabaseStep = match response.json().await {
		Ok(s) => s,
		Err(err) => {
			err.process_err(Status::MappingError, "");
			return Ok(failed_result);
		}
	};

	if let DatabaseStep::SyncOperations { operations, .. } = step_to_client {
		for operation in operations.iter() {
			if let Err(err) = sql_insert_update_operation_function(
				&mut *tx, operation, 1
			).await {
				err.process_err(err, "");
				return Ok(failed_result);
			}
		}
	}

	match tx.commit().await {
		Ok(_) => Ok(DatabaseStep::SyncSuccess { text: DatabaseInfo::Notning }),
		Err(err) => {
			err.process_err(Status::MappingError, "");
			Ok(failed_result)
		}
	}
}
use crate::{Status, ProcessError, ClientState};
use crate::sql_models::operation::implements::{Operation, OperationDto};
use crate::sql_models::operation::service::{OperationStep, OperationInfo};
use crate::service::mchd::home_mchd_power::HomeMchdPower;
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::mchd::show_powers::check_access;
use crate::client::back_api::post_query::post_query_back_api;
use crate::client::sql_queries::operations::add::que::sql_insert_update_operation_function;

pub async fn process_cmd_process_operations_function(
	state: &ClientState,
	option_operations: Vec<Option<OperationDto>>
) -> Result<OperationStep, Status> {

	let failed_result = OperationStep::TryLater { text: OperationInfo::ClientApiSystemError };

	match check_access(state, vec![
		HomeMchdPower::H210,
		HomeMchdPower::H110,
		HomeMchdPower::H510,
		HomeMchdPower::H810,
	]).await {
        Ok(true) => {},
        Ok(false) => {
            return Ok(OperationStep::TryLater { text: OperationInfo::AccessDenied })
        },
        Err(err) => {
            err.process_err(err, None);
			return Ok(failed_result);
        }
    }

	let session = match state.get_session().await {
		Ok(s) => s,
		Err(err) => {
			err.process_err(err, None);
			return Ok(failed_result);
		}
	};

	let operations: Vec<Operation> = option_operations.into_iter().flatten().map(|x| x.into()).collect();

	let count = operations.len();

	let mut tx = match session.local_db.begin().await {
		Ok(c) => c,
		Err(err) => {
			err.process_err(Status::SqlCommitErr, None);
			return Ok(failed_result);
		}
	};

	for operation in operations.iter() {
		if let Err(err) = sql_insert_update_operation_function(&mut *tx, operation, 1). await {
			err.process_err(Status::SqlQueryWrongLogic, None);
			return Ok(OperationStep::from_err(&err));
		}
	}

	if let Err(err) = tx.commit().await {
		err.process_err(Status::SqlCommitErr, None);
		return Ok(OperationStep::from_err(&Status::SqlCommitErr));
	}

	let response = match post_query_back_api(
		state, 
		state.config.get_sql_long(), 
		ApiRoutes::SqlOperationAddMany,
		&operations 
	).await{
		Ok(r) => r,
		Err(err) =>  {
			err.process_err(err, None);
			return Ok(OperationStep::from_err(&err));
		}		
	};

	if !response.status().is_success() {
		return Ok(OperationStep::from_err(&Status::SqlSyncError));
	}

	Ok(OperationStep::ProcessSuccess { text: OperationInfo::Nothing, count: count as i32})

}
use crate::{Status, ProcessError, ClientState, LL};
use crate::sql_models::operation::implements::{Operation, OperationDto};
use crate::sql_models::operation::service::{OperationStep, OperationInfo};
use crate::service::mchd::home_mchd_power::HomeMchdPower;

use crate::client::mchd::show_powers::check_access;
use crate::client::sql_queries::operations::add::que_insert_update::sql_insert_update_operation_function;

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
            err.process_err(err, "");
			return Ok(failed_result);
        }
    }

	let session = match state.get_session().await {
		Ok(s) => s,
		Err(err) => {
			err.process_err(err, "");
			return Ok(failed_result);
		}
	};

	let operations: Vec<Operation> = option_operations.into_iter().flatten().map(|x| x.into()).collect();

	let count = operations.len();

	let mut conn = match session.local_db.begin().await {
		Ok(c) => c,
		Err(err) => {
			err.process_err(Status::SqlCommitErr, "");
			return Ok(failed_result);
		}
	};

	for operation in operations {
		if let Err(err) = sql_insert_update_operation_function(&mut *conn, &operation, 0). await {
			err.process_err(Status::SqlQueryWrongLogic, "");
			return Ok(failed_result);
		}
	}

	let step = OperationStep::ProcessSuccess { text: OperationInfo::Nothing, count: count as i32};

	Ok(step)
}
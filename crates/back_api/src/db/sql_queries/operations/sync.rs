use std::collections::HashSet;


use shared_lib::sql_models::operation::implements::Operation;
use shared_lib::{Status, ProcessError, LL};
use shared_lib::service::database::service::{DatabaseStep, DatabaseInfo};
use shared_lib::primitives::frozen::text::BoxUuid;


use crate::config::BackApiState;

use crate::db::sql_queries::users::get::que::sql_get_comp_id_by_user_id_function;
use crate::db::sql_queries::operations::get::que::{
	sql_get_operation_ids_by_comp_id_function,
	sql_get_operation_by_id_function
};
use crate::db::sql_queries::operations::add::que::sql_insert_update_operation_function;

pub async fn sync_all_operations(
	state: &BackApiState,
	local_step: &DatabaseStep
) -> Result<DatabaseStep, Status> {

	let mut failed_result = DatabaseStep::TryLater { text: DatabaseInfo::BackApiError };

	let (local_operations, user_id) = if let DatabaseStep::SyncOperations { operations, user_id, ..} = local_step {
		(operations, user_id)
	} else {
		Status::log(LL::Error, "WRONG STEP IN sync_all_operations FUNCTION");
		return Ok(failed_result);
	};

	let mut tx = match state.pool_fast.begin().await {
		Ok(t) => t,
		Err(err) => {
			err.process_err(Status::SqlPoolErr, "");
			return Ok(failed_result);
		}
	};

	let comp_id = match sql_get_comp_id_by_user_id_function(&mut *tx, user_id).await {
		Ok(Some(c)) => c,
		Ok(None) => {
			if let DatabaseStep::TryLater { text } = &mut failed_result {
				*text = DatabaseInfo::MissUser;
			}
			Status::log(LL::Error, "MISS USER IN sync_all_operations FUNCTION");
			return Ok(failed_result);
		},
		Err(err) => {
			err.process_err(err, "");
			failed_result.process_info(&err);
            return Ok(failed_result); 
		}
	};

	let mut all_server_operation_ids: HashSet<BoxUuid> = match sql_get_operation_ids_by_comp_id_function(&mut *tx, &comp_id).await {
		Ok(ids_vec) => ids_vec.into_iter().collect(),
		Err(err) => {
			err.process_err(Status::SqlPoolErr, "");
			failed_result.process_info(&err);
            return Ok(failed_result); 
		}
	};

	let mut server_operations: Vec<Operation> = vec![];

	for operation in local_operations {
		if let Err(err) = sql_insert_update_operation_function(&mut *tx, operation).await {
			err.process_err(err, "");
			failed_result.process_info(&err);
            return Ok(failed_result); 
		}
		all_server_operation_ids.remove(&operation.oper_id);
	}

	for server_operation_id in all_server_operation_ids {
		let server_operation_option = sql_get_operation_by_id_function(&mut *tx, &server_operation_id).await
			.map_err(|err| err.process_err(err, ""))?;
		if let Some(server_operation) = server_operation_option {
			server_operations.push(server_operation);
		}
	}

	match tx.commit().await {
		Ok(_) => Ok(DatabaseStep::SyncOperations { text: DatabaseInfo::Notning, operations: server_operations, user_id: user_id.clone() }),
		Err(err) => {
			err.process_err(Status::SqlPoolErr, "");
			Ok(failed_result)
		}
	}

}
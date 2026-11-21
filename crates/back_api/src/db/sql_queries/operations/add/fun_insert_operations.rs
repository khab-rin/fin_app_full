use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::operation::implements::Operation;

use crate::config::BackApiState;
use crate::db::sql_queries::operations::add::que::sql_insert_update_operation_function;


pub async fn insert_operations_function(
	state: &BackApiState,
	operations: Vec<Operation>
) -> Result<(), Status> {

	let mut tx = state.pool_long.begin().await
		.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	for operation in operations.iter() {
		sql_insert_update_operation_function(&mut *tx, operation).await
			.map_err(|err| err.process_err(err, ""))?;
	}

	tx.commit().await.map_err(|err| err.process_err(Status::SqlCommitErr, ""))

}
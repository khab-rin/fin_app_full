use crate::{ClientState, Status, ProcessError};
use crate::sql_models::operation::implements::Operation;
use crate::service::api_routes::implements::ApiRoutes;
use crate::client::back_api::post_query::post_query_back_api;

pub async fn add_new_operations(
	state: &ClientState,
	operations: Vec<Operation>
) -> Result<(), Status> {
	let session = state.get_session().await
		.map_err(|err| err.process_err(err, ""))?;

	
	post_query_back_api(
		state, 
		state.config.get_sql_long(), 
		ApiRoutes::SqlOperationsAddMany, 
		&operations).await
		.map_err(|err| err.process_err(err, ""))?;


	let mut tx = session.local_db.begin().await
		.map_err(|err| err.process_err(Status::SqLitePoolErr, ""))?;


	for oper in operations.iter() {
		sqlx::query_file!(
			"src/client/sql_queries/operations/add/new_operation.sql",
			oper.oper_id,
			oper.user_id,
			oper.comp_id,
			oper.ctrpty_id,
			oper.contract_id,
			oper.debet,
			oper.credit,
			oper.amount,
			oper.oper_date,
			oper.doc_type,
			oper.doc_num,
			oper.doc_date,
			oper.is_storno,
			oper.is_del,
			oper.entr_date
		).execute(&mut *tx).await
		.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;
	}

	tx.commit().await.map_err(|err| err.process_err(Status::SqliteCommitErr, ""))?;

	Ok(())
}
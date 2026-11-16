use shared_lib::static_data::null_elems::NULLS;
use shared_lib::{Status, ProcessError};

use crate::config::BackApiState;
use crate::db::sql_queries::companys::add::que_insert_update::sql_insert_update_company_function;
use crate::db::sql_queries::persons::add::que_insert_update::sql_insert_update_person_function;
use crate::db::sql_queries::users::add::que_insert_update_user::sql_insert_update_user_function;
use crate::db::sql_queries::contracts::add::que_insert_update_contract::sql_insert_update_contract_function;
use crate::db::sql_queries::operations::add::que_insert_update_operation::que_insert_update_operation_function;

pub async fn make_null_postgress_elements(
	state: &BackApiState
) -> Result<(), Status> {
	

	let mut conn = state.pool_fast.begin().await
		.map_err(|err| err.process_err(Status::SqlCommitErr, ""))?;

	sql_insert_update_company_function(&mut *conn, &NULLS.model.company).await
		.map_err(|err| err.process_err(err, ""))?;
	
	sql_insert_update_person_function(&mut *conn, &NULLS.model.person).await
		.map_err(|err| err.process_err(err, ""))?;

	sql_insert_update_user_function(&mut *conn, &NULLS.model.user_data).await
		.map_err(|err| err.process_err(err, ""))?;

	sql_insert_update_contract_function(&mut *conn, &NULLS.model.contract).await
		.map_err(|err| err.process_err(err, ""))?;

	que_insert_update_operation_function(&mut *conn, &NULLS.model.operation).await
		.map_err(|err| err.process_err(err, ""))?;

	conn.commit().await.map_err(|err| err.process_err(Status::SqlCommitErr, ""))?;

	Ok(())
}

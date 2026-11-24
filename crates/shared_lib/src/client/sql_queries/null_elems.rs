use crate::static_data::null_elems::NULLS;
use crate::{Status, ProcessError};
use crate::service::auth_service::general::ActiveSession;

use crate::client::sql_queries::companys::add::que_insert_update::sql_insert_update_company_query;
use crate::client::sql_queries::persons::add::que_insert_update::sql_insert_update_person_function;
use crate::client::sql_queries::contracts::add::que::sql_insert_update_contract_function;
use crate::client::sql_queries::operations::add::que::sql_insert_update_operation_function;

pub async fn make_null_sqlite_elements(
	session: &ActiveSession
) -> Result<(), Status> {

	let mut conn = session.local_db.begin().await.map_err(|err| err.process_err(Status::SqlCommitErr, None))?;

	sql_insert_update_company_query(&mut *conn, &NULLS.model.company).await
		.map_err(|err| err.process_err(err, None))?;

	sql_insert_update_person_function(&mut *conn, &NULLS.model.person, 1).await
		.map_err(|err| err.process_err(err, None))?;

	sql_insert_update_contract_function(&mut *conn, &NULLS.model.contract, 1).await
		.map_err(|err| err.process_err(err, None))?;

	sql_insert_update_operation_function(&mut *conn, &NULLS.model.operation, 1).await
		.map_err(|err| err.process_err(err, None))?;

	conn.commit().await.map_err(|err| err.process_err(Status::SqlCommitErr, None))?;

	Ok(())
}


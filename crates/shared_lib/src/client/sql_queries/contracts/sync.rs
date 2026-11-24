use crate::{ClientState, Status, ProcessError};
use crate::service::api_routes::implements::ApiRoutes;
use crate::service::database::service::{DatabaseStep, DatabaseInfo};

use crate::client::sql_queries::companys::add::que_insert_update::sql_insert_update_company_query;
use crate::client::sql_queries::contracts::get::que::sql_get_all_contracts;
use crate::client::sql_queries::contracts::add::que::sql_insert_update_contract_function;
use crate::client::back_api::post_query::post_query_back_api;

pub async fn sync_all_contracts(
	state: &ClientState,
) -> Result<DatabaseStep, Status> {
	let failed_result = DatabaseStep::TryLater { text: DatabaseInfo::ClientServiceError };

	let session = match state.get_session().await {
		Ok(s) => s,
		Err(err) => {
			err.process_err(err, None);
			return Ok(failed_result);
		}
	};

	let local_contracts = match sql_get_all_contracts(&session.local_db).await{
		Ok(a) => a,
		Err(err) => {
			err.process_err(err, None);
			return Ok(failed_result);
		}
	};

	let step_to_serv = DatabaseStep::SyncContracts { 
		text: DatabaseInfo::Notning, 
		contracts: local_contracts, 
		companys: vec![],
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
			err.process_err(err, None);
			return Ok(DatabaseStep::from_err(&err));
		}
	};

	let step_to_client:DatabaseStep = match response.json().await {
		Ok(s) => s,
		Err(err) => {
			let detailed_err = format!("{:?}", err);
			err.process_err(Status::MappingError, Some(&detailed_err));
			return Ok(failed_result);
		}
	};

	let mut tx = match session.local_db.begin().await {
		Ok(c) => c,
		Err(err) => {
			err.process_err(Status::SqlPoolErr, None);
			return Ok(failed_result);
		}
	};

	if let DatabaseStep::SyncContracts { contracts, companys, .. } = step_to_client {
		for company in companys {
			if let Err(err) = sql_insert_update_company_query(&mut *tx, &company).await {
				err.process_err(err, None);
				return Ok(DatabaseStep::from_err(&err));
			}
		}
		for contract in contracts.iter() {
			if let Err(err) = sql_insert_update_contract_function(
				&mut *tx, contract, 1
			).await {
				err.process_err(err, None);
				return Ok(DatabaseStep::from_err(&err));
			}
		}
	}

	match tx.commit().await {
		Ok(_) => Ok(DatabaseStep::SyncSuccess { text: DatabaseInfo::Notning }),
		Err(err) => {
			err.process_err(Status::SqlPoolErr, None);
			Ok(failed_result)
		}
	}
}
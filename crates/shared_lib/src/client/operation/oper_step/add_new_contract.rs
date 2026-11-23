use crate::{ClientState, Status, ProcessError, LL};

use crate::sql_models::contracts::implements::{NewContrData};
use crate::sql_models::operation::service::{OperationInfo, OperationStep};
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::sql_queries::contracts::add::fun_new_contract::make_new_contract_function;
use crate::client::sql_queries::contracts::get::que::{
	sql_get_contract_by_contr_id_fun,
	sql_get_contracts_by_comp_ctrpty_id_func
};

use crate::client::sql_queries::contracts::add::que::sql_insert_update_contract_function;
use crate::client::back_api::post_query::post_query_back_api;

pub async fn process_cmd_add_new_contract(
	state: &ClientState,
	new_contr_data: NewContrData
) -> Result<OperationStep, Status> {

	let failed_result = OperationStep::TryLater { text: OperationInfo::ClientApiSystemError };

	let new_contract = match make_new_contract_function(state, new_contr_data).await {
		Ok(c) => c,
		Err(err) => {
			err.process_err(err, "");
			return Ok(OperationStep::from_err(&err));
		}
	};

	let session = match state.get_session().await {
		Ok(s) => s, 
		Err(err) => {
			err.process_err(err, "");
			return Ok(failed_result);
		}
	};

	let exist_contract = match sql_get_contract_by_contr_id_fun(&session.local_db, &new_contract.contract_id).await {
		Ok(opt) => opt,
		Err(err) => {
			err.process_err(err, "");
			return Ok(OperationStep::from_err(&err));
		}
	};

	let mut all_contracts = match sql_get_contracts_by_comp_ctrpty_id_func(&session.local_db, &new_contract.comp_id, &new_contract.ctrpty_id).await {
		Ok(c) => c,
		Err(err) => {
			err.process_err(err, "");
			return Ok(OperationStep::from_err(&err));
		}
	};

	if exist_contract.is_some() {
		return Ok(OperationStep::AddContractSuccess { contracts: all_contracts, text: OperationInfo::DuplicateContract});
	}

	let response = match post_query_back_api(
		state,
		state.config.get_inst_client(),
		ApiRoutes::SqlContractAddNew,
		&new_contract
	).await {
		Ok(r) => r, 
		Err(err) => {
			err.process_err(err, "");
			return Ok(OperationStep::from_err(&err));
		}
	};

	if !response.status().is_success() {
		Status::log(LL::Warn, &format!("ApiRoutes::SqlContractAddNew FAILED, contract = {:?}", new_contract));
		return Ok(failed_result);
	}

	if let Err(err) = sql_insert_update_contract_function(&session.local_db, &new_contract, 1).await {
		err.process_err(err, "");
		return Ok(OperationStep::from_err(&err));
	}

	all_contracts.push(new_contract);
	
	Ok(OperationStep::AddContractSuccess { contracts: all_contracts, text: OperationInfo::AddContractSuccess})

}
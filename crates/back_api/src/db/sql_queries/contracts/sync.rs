use shared_lib::primitives::frozen::text::BoxUuid;
use shared_lib::{Status, ProcessError, LL};
use shared_lib::service::database::service::{DatabaseStep, DatabaseInfo};
use shared_lib::sql_models::contracts::implements::Contract;
use shared_lib::sql_models::company::implements::Company;


use crate::config::BackApiState;
use crate::db::sql_queries::users::get::que::sql_get_comp_id_by_user_id_function;
use crate::db::sql_queries::contracts::add::que::sql_insert_update_contract_function;
use crate::db::sql_queries::companys::get::que::get_company_option_by_id;
use crate::db::sql_queries::contracts::get::que::{
	sql_get_contract_ids_by_comp_id,
	sql_get_contract_by_id_function
};

pub async fn sync_all_contracts(
	state: &BackApiState,
	local_step: &DatabaseStep
) -> Result<DatabaseStep, Status> {
	let mut failed_result = DatabaseStep::TryLater { text: DatabaseInfo::BackApiError };

	let (local_contracts, user_id) = if let DatabaseStep::SyncContracts { contracts, user_id, .. } = local_step {
		(contracts, user_id)
	} else {
		Status::log(LL::Error, "WRONG STEP IN sync_all_contracts function");
		return Ok(failed_result)
	};

	let mut tx = match state.pool_fast.begin().await {
		Ok(c) => c,
		Err(err) => {
			err.process_err(Status::SqlPoolErr, None);
			return Ok(failed_result);
		}
	};

	let comp_id = match sql_get_comp_id_by_user_id_function(&mut *tx, user_id).await {
		Ok(Some(c)) => c,
		Ok(None) => {
			if let DatabaseStep::TryLater { text } = &mut failed_result {
				*text = DatabaseInfo::MissUser
			}
			return Ok(failed_result);
		}
		Err(err) => {
			err.process_err(err, None);
			return Ok(DatabaseStep::from_err(&err));
		}
	};

	let mut serv_contract_ids: std::collections::HashSet<BoxUuid> = match sql_get_contract_ids_by_comp_id(&mut *tx, &comp_id).await{
		Ok(contr_vec) => contr_vec.into_iter().collect(),
		Err(err) => {
			err.process_err(err, None);
			return Ok(DatabaseStep::from_err(&err));
		}
	};

	for local_contract in local_contracts {
		let local_contract_id = &local_contract.contract_id;
		serv_contract_ids.remove(local_contract_id);
		if let Err(err) = sql_insert_update_contract_function(&mut *tx, local_contract).await {
			err.process_err(err, None);
			return Ok(DatabaseStep::from_err(&err));
		}
	}

	let mut server_contracts: Vec<Contract> = vec![];
	let mut server_companys: Vec<Company> = vec![];
		
	for serv_contract_id in serv_contract_ids {
		let server_contract = match sql_get_contract_by_id_function(&mut *tx, &serv_contract_id).await {
			Ok(Some(c)) => c,
			Ok(None) => {continue;},
			Err(err) => {
				err.process_err(err, None);
				return Ok(DatabaseStep::from_err(&err)); 
			}
		};
		server_contracts.push(server_contract);
	}

	for contract in server_contracts.iter() {
		let comp_id = &contract.ctrpty_id;
		match get_company_option_by_id(&mut *tx, comp_id).await {
			Ok(Some(c)) => server_companys.push(c),
			Ok(None) => {},
			Err(err) => {
				err.process_err(err, None);
				return Ok(DatabaseStep::from_err(&err));
			}
		}
	}

	if let Err(err) = tx.commit().await {
		err.process_err(Status::SqlCommitErr, None);
		return Ok(failed_result);
	}

	Ok(DatabaseStep::SyncContracts { 
		text: DatabaseInfo::Notning, 
		contracts: server_contracts,
		companys: server_companys, 
		user_id: user_id.clone() })
}
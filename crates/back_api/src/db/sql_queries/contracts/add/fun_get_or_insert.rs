use shared_lib::{Status, ProcessError, LL};
use shared_lib::sql_models::contracts::implements::Contract;


use crate::config::BackApiState;
use crate::db::sql_queries::contracts::get::que::sql_get_contract_by_id_function;
use crate::db::sql_queries::contracts::add::que::sql_insert_update_contract_function;


pub(crate) async fn insert_contract_if_not_exist_function(
    state: &BackApiState,
    contract: Contract
) -> Result<Contract, Status> {

    let contr_option = sql_get_contract_by_id_function(&state.pool_fast, &contract.contract_id).await
        .map_err(|err| err.process_err(err, ""))?;

    if let Some(contract) = contr_option {
		Status::log(LL::Debug, &format!("{:?}", contract.contract_id));
        return Ok(contract);
    }

	sql_insert_update_contract_function(&state.pool_fast, &contract).await
		.map_err(|err| err.process_err(err, ""))
   
}
use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::contracts::implements::Contract;
use shared_lib::primitives::frozen::text::{Integ,BoxUuid, DocNum, Date, Currency, RubF};

use crate::config::BackApiState;
use crate::db::sql_queries::contracts::get::by_contr_id::get_contract_by_contract_id;


pub(crate) async fn add_contract(
    state: &BackApiState,
    contract: Contract
) -> Result<Contract, Status> {

    let contr_option = get_contract_by_contract_id(state, &contract.contract_id).await
        .map_err(|err| err.process_err(err, ""))?;

    if let Some(contract) = contr_option {
        return Ok(contract);
    }

    let contract: Contract = sqlx::query_file_as!(
        Contract,
        "src/db/sql_queries/contracts/add/new_contr.sql",
        contract.contract_id.as_ref(),
        &contract.user_id.as_ref(),
        contract.comp_id.as_ref(),
        contract.ctrpty_id.as_ref(),
        contract.contract_num.as_ref(),
        contract.contract_date.as_ref(),
        contract.title.as_str(),
        contract.st_date.as_ref(),
        contract.end_date.as_ref(),
        contract.currency.as_str(),
        contract.total_amount.as_ref(),
        contract.payment_deferral_days.as_ref(),
        contract.is_active.as_ref(),
        contract.descrip.as_str(),
        contract.entr_date.as_ref(),
        contract.is_del.as_ref()
    ).fetch_one(&state.pool_fast).await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;



    const SQL_QUERY: &str = include_str!("/home/khabipovrinat/dev/fin_app_full/crates/back_api/src/db/sql_queries/contracts/add/new_contr.sql");

    let contract: Contract = sqlx::query_as(SQL_QUERY)
        .bind(&contract.contract_id)
        .bind(contract.user_id)
        .bind(contract.comp_id)
        .bind(contract.ctrpty_id)
        .bind(contract.contract_num)
        .bind(contract.contract_date)
        .bind(contract.title)
        .bind(contract.st_date)
        .bind(contract.end_date)
        .bind(contract.currency)
        .bind(contract.total_amount)
        .bind(contract.payment_deferral_days)
        .bind(contract.is_active)
        .bind(contract.descrip)
        .bind(contract.entr_date)
        .bind(contract.is_del)
        .fetch_one(&state.pool_fast).await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;
            

    Ok(contract)
    
}
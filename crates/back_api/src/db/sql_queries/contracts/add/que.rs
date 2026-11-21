use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::contracts::implements::Contract;
use shared_lib::primitives::frozen::text::{Integ,BoxUuid, DocNum, Date, Currency, RubF};


pub(crate) async fn sql_insert_update_contract_function<'t, E>(
    conn: E,
    contract: &Contract
) -> Result<Contract, Status> 
where E: sqlx::Executor<'t, Database=sqlx::Postgres>
{

    let contract: Contract = sqlx::query_file_as!(
        Contract,
        "src/db/sql_queries/contracts/add/que_insert_update_contract.sql",
        contract.contract_id.as_ref(),
        contract.user_id.as_ref(),
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
        &contract.is_active,
        contract.descrip.as_str(),
        contract.entr_date.as_ref(),
        &contract.is_del
    ).fetch_one(conn).await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;    

    Ok(contract)
    
}
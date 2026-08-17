use shared_lib::sql_models::contracts::implements::Contract;
use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{BoxUuid, DocNum, Date, Currency, RubF};

use crate::config::BackApiState;


pub(crate) async fn get_contract_by_contract_id(
    state: &BackApiState,
    contract_id: BoxUuid
) -> Result<Option<Contract>, Status> {

    let contract_option = sqlx::query_file_as!(
        Contract,
        "src/db/sql_queries/contracts/get/by_contr_id.sql",
        contract_id.as_ref()
    ).fetch_optional(&state.pool_fast).await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    Ok(contract_option)
}
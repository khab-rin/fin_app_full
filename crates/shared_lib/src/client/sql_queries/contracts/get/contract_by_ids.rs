use crate::{Status, ClientState, ProcessError};
use crate::primitives::frozen::text::{Integ, Date, RubF, BoxUuid, DocNum, Currency};
use crate::sql_models::contracts::implements::Contract;

pub async fn get_contracts_by_comp_ctrpty_ids(
    state: &ClientState,
    ctrpty_id: &BoxUuid
) -> Result<Vec<Contract>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

    let comp_id = session.session_user.company.comp_id.clone();

    sqlx::query_file_as!(
            Contract,
            "src/client/sql_queries/contracts/get/by_comp_ctrpty_ids.sql",
            comp_id,
            ctrpty_id
        ).fetch_all(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))

}

pub async fn get_contract_by_contr_id(
    state: &ClientState,
    contr_id: &BoxUuid
) -> Result<Option<Contract>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?;


    sqlx::query_file_as!(
            Contract,
            "src/client/sql_queries/contracts/get/by_contr_id.sql",
            contr_id
        ).fetch_optional(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))

}
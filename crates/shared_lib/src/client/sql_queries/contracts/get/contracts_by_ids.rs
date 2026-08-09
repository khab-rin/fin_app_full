use crate::{Status, ClientState, ProcessError};
use crate::primitives::frozen::text::{Date, DateTime, RubF, BoxUuid};
use crate::sql_models::contracts::implements::Contract;

pub async fn get_contracts_by_comp_ctrpty_ids(
    state: &ClientState,
    comp_id: &BoxUuid,
    ctrpty_id: &BoxUuid
) -> Result<Vec<Contract>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

    let var1 = comp_id.as_ref();
    let var2 = ctrpty_id.as_ref();

    sqlx::query_file_as!(
            Contract,
            "src/client/sql_queries/contracts/get/contracts_by_ids.sql",
            var1,
            var2
        ).fetch_all(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))

}
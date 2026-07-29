use shared_lib::Status;
use shared_lib::primitives::frozen::text::{Date, DateTime, RubF, BoxUuid};
use shared_lib::sql_models::contracts::implements::Contract;

use crate::state::ClientState;

pub async fn get_contracts_by_comp_ctrpty_ids(
    state: &ClientState,
    comp_id: &BoxUuid,
    ctrpty_id: &BoxUuid
) -> Result<Vec<Contract>, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN get_contracts_by_comp_ctrpty_ids FAILED BY MISS SASSION", err
            );
            return Err(err);
        }
    };

    let var1 = comp_id.as_ref();
    let var2 = ctrpty_id.as_ref();

    match sqlx::query_file_as!(
        Contract,
        "src/sql_queries/contracts/get/contracts_by_ids.sql",
        var1,
        var2
    ).fetch_all(&session.local_db).await {
        Ok(r) => Ok(r),
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN get_contracts_by_comp_ctrpty_ids FAILED BY MISS SASSION", 
                err, Status::SqlQueryWrongLogic
            );
            Err(Status::SqlQueryWrongLogic)
        } 
    }
}
use std::collections::HashSet;

use crate::primitives::frozen::text::BoxUuid;
use crate::{ClientState, Status, ProcessError};
use crate::sql_models::operation::implements::OperationRaw;

pub async fn get_exist_ids_by_ids(
    state: &ClientState,
    operations: &[OperationRaw]
) -> Result<HashSet<BoxUuid>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 


    let all_check_ids: HashSet<BoxUuid> = operations
        .iter()
        .map(|x| x.oper_id.clone())
        .collect();

    let json_ids = serde_json::to_string(&all_check_ids)
        .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?; 

    let json_ids_str = &json_ids;

    let exist_ids: HashSet<BoxUuid> = sqlx::query_file_scalar!(
            "src/client/sql_queries/operations/get/exist_ids_by_ids.sql",
            json_ids_str
        ).fetch_all(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?
        .into_iter()
        .flatten()
        .collect(); 


    Ok(exist_ids)
}
use std::collections::HashSet;

use crate::{ClientState, Status, ProcessError};
use crate::sql_models::operation::implements::OperationRaw;

pub async fn get_ext_ids_by_ext_id(
    state: &ClientState,
    operations: &[OperationRaw]
) -> Result<HashSet<i64>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 


    let all_ext_ids: HashSet<i64> = operations
        .iter()
        .filter_map(|x| x.external_id)
        .collect();

    let json_ids = serde_json::to_string(&all_ext_ids)
        .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?; 

    let json_ids_str = &json_ids;

    let exist_ext_ids: HashSet<i64> = sqlx::query_file_scalar!(
            "src/client/sql_queries/operations/get/ext_ids_by_ext_ids.sql",
            json_ids_str
        ).fetch_all(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?
        .into_iter()
        .collect(); 


    Ok(exist_ext_ids)
}
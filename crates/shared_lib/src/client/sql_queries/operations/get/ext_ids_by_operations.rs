use std::collections::HashSet;

use crate::{ClientState, Status};
use crate::sql_models::operation::implements::OperationRaw;

pub async fn get_ext_ids_by_ext_id(
    state: &ClientState,
    operations: &[OperationRaw]
) -> Result<HashSet<i64>, Status> {

    let mut result: HashSet<i64> = HashSet::new();

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN get_ext_ids_by_ext_id FAILED BY MISS SESSION", err
            );
            return Err(err);
        }
    };

    let all_ext_ids: HashSet<i64> = operations.iter().map(|x| x.external_id).collect();

    let json_ids = match serde_json::to_string(&all_ext_ids) {
        Ok(a) => a,
        Err(err) =>  {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN get_ext_ids_by_ext_id FAILED BY serde_json::to_string",
                err, Status::MappingError
            );
            return Err(Status::MappingError);
        }
    };

    let json_ids_str = &json_ids;

    let exist_ext_ids: HashSet<i64> = match sqlx::query_file_scalar!(
        "src/client/sql_queries/operations/get/ext_ids_by_ext_ids.sql",
        json_ids_str
    ).fetch_all(&session.local_db).await {
        Ok(r) => r.into_iter().collect(),
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN get_ext_ids_by_ext_id FAILED BY WRONG SQL QUERY LOGIC",
                err, Status::SqlQueryWrongLogic
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };


    Ok(exist_ext_ids)
}
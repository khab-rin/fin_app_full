
use crate::primitives::frozen::text::BoxUuid;
use crate::{ClientState, Status, ProcessError};

pub async fn get_exist_id_by_id(
    state: &ClientState,
    check_oper_id: &BoxUuid
) -> Result<Option<BoxUuid>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 


    let exist_oper_id = sqlx::query_file_scalar!(
        "src/client/sql_queries/operations/get/exist_id_by_id.sql",
        check_oper_id
    ).fetch_optional(&session.local_db).await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    Ok(exist_oper_id)
}
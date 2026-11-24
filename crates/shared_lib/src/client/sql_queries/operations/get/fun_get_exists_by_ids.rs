use crate::{ClientState, ProcessError, Status};
use crate::primitives::frozen::text::BoxUuid;

use crate::client::sql_queries::operations::get::que::sql_exist_operation_by_id_que;

pub async fn get_exists_operations_by_ids_function(
	state: &ClientState,
	ids: &[BoxUuid]
) -> Result<Vec<BoxUuid>, Status> {

	let session = state.get_session().await
		.map_err(|err| err.process_err(err, None))?;

	let mut res: Vec<BoxUuid> = vec![];

	let mut conn = session.local_db.acquire().await
		.map_err(|err| err.process_err(Status::SqlPoolErr, None))?;

	for id in ids {
		if sql_exist_operation_by_id_que(&mut *conn, id).await.map_err(|err| err.process_err(err, None))? {
			res.push(id.clone());
		}
	}

	Ok(res)
}
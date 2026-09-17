use crate::{Status, ProcessError, ClientState};

use crate::service::reports::service::{QuartCummulAmnt, QuartDates};
use crate::primitives::frozen::text::RubF;


pub async fn get_quater_cummul_incomes_usn(
	state: &ClientState,
	dates: &QuartDates
) -> Result<QuartCummulAmnt, Status> {
	
	let session = state.get_session().await
		.map_err(|err| err.process_err(err, ""))?;

	sqlx::query_file_as!(
		QuartCummulAmnt,
		"src/client/sql_queries/operations/get/reports/fns/usn_incomes.sql",
		dates.start,
		dates.q1,
		dates.q2,
		dates.q3,
		dates.q4
	).fetch_one(&session.local_db).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))

}
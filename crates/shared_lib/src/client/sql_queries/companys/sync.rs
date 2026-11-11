use crate::{ClientState, ProcessError, Status};
use crate::primitives::frozen::text::{BoxUuid, CompInn, Kpp, CompType, CompStatus, DateTime};
use crate::service::reports::service::{ReportStep, ReportInfo};
use crate::sql_models::company::implements::{Company, CompanyDto};

pub async fn sync_all_companys(
	state: &ClientState
) -> Result<ReportStep, Status> {

	let failed_result: Result<ReportStep, Status> = Ok(ReportStep::TryLater { text: ReportInfo::ClientServiceError });

	let session = match state.get_session().await {
		Ok(s) => s,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};


	let local_companys_dto = sqlx::query_file_as!(
		CompanyDto,
		"src/client/sql_queries/companys/get/all.sql"
	).fetch_all(&session.local_db).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	let local_companys: Vec<Company> = local_companys_dto
		.into_iter()
		.map(|x| 
			x
			.try_into()
			.map_err(|e: serde_json::Error| e.process_err(Status::MappingError, "")))
		.collect::<Result<Vec<Company>, Status>>()?;

	

	Err(Status::Unknown)
}
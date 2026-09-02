use crate::{Status, ClientState, ProcessError};
use crate::primitives::frozen::text::{Kpp, CompInn, BoxUuid, CompStatus, CompType, DateTime};
use crate::sql_models::company::implements::{Company, CompanyDto};


pub async fn get_all_companys(
    state: &ClientState,
) -> Result<Vec<Company>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

    
    let companys_dto = sqlx::query_file_as!(
            CompanyDto,
            "src/client/sql_queries/companys/get/all.sql",
        ).fetch_all(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    let companys: Vec<Company> = companys_dto
		.into_iter()
		.map(|x| 
			x
			.try_into()
			.map_err(|err: serde_json::Error| err.process_err(Status::MappingError, "")))
		.collect::<Result<Vec<Company>, Status>>()?;

	Ok(companys)
}
use shared_lib::primitives::frozen::text::{CompInn, BoxUuid, DateTime, Kpp, CompType, CompStatus};
use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};

use crate::config::BackApiState;


pub(crate) async fn get_company_by_userid(
    state: &BackApiState,
    user_id: &BoxUuid
) -> Result<Option<Company>, Status> {

    let company_dto_option = sqlx::query_file_as!(
        CompanyDto,
        "src/db/sql_queries/companys/get/company_by_userid.sql",
        user_id.as_ref()
    ).fetch_optional(&state.pool_fast).await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    let company_dto = match company_dto_option {
        Some(d) => d,
        None => return Ok(None)
    };

    let company: Company = company_dto
        .try_into()
        .map_err(|err: serde_json::Error| err.process_err(Status::SqlQueryWrongLogic, ""))?; 


    Ok(Some(company))
}
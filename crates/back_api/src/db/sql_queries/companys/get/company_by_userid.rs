use shared_lib::primitives::frozen::implements::{CompInn, BoxUuid, DateTime, Kpp, CompType, CompStatus};
use shared_lib::Status;
use shared_lib::sql_models::company::implements::{Company, CompanyDto};

use crate::config::BackApiState;


pub(crate) async fn get_company_by_userid(
    state: &BackApiState,
    user_id: &BoxUuid
) -> Result<Option<Company>, Status> {

    let company_dto_option = match sqlx::query_file_as!(
        CompanyDto,
        "src/db/sql_queries/companys/get/company_by_userid.sql",
        user_id.as_ref()
    ).fetch_optional(&state.pool_fast).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::SqlQueryWrongLogic,
                "FUN get_company_by_userid FAILED BY SQL QUERY"
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };

    let company_dto = match company_dto_option {
        Some(d) => d,
        None => return Ok(None)
    };

    let company: Company = match company_dto.try_into() {
        Ok(p) => p,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN get_company_by_userid FAIELD BY DTO MAPPING"
            );
            return Err(Status::MappingError);
        }
    };


    Ok(Some(company))
}
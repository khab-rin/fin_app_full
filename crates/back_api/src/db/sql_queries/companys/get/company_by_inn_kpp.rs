use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{CompInn, Kpp, BoxUuid, CompStatus, CompType, DateTime};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};

use crate::config::BackApiState;

pub(crate) async fn get_company_by_inn_kpp(
    state: &BackApiState,
    inn: &CompInn,
    kpp: &Kpp

) -> Result<Option<Company>, Status> {

    let dto_option = sqlx::query_file_as!(
        CompanyDto,
        "src/db/sql_queries/companys/get/company_by_inn_kpp.sql",
        inn.as_ref(),
        kpp.as_ref()
    ).fetch_optional(&state.pool_fast)
    .await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    let dto = match dto_option {
        Some(d) => d,
        None => return Ok(None)
    };

    let company = dto
        .try_into()
        .map_err(|err: serde_json::Error| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    Ok(Some(company))


}
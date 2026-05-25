use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{Inn, Kpp, BoxUuid, CompStatus, CompType, DateTime};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};

use crate::config::BackApiState;

pub(crate) async fn get_company_by_inn_kpp(
    state: &Arc<BackApiState>,
    inn: &Inn,
    kpp: &Kpp

) -> Result<Option<Company>, Status> {

    let dto_option = match sqlx::query_file_as!(
        CompanyDto,
        "src/db/sql_queries/companys/get/company_by_inn_kpp.sql",
        inn.as_ref(),
        kpp.as_ref()
    ).fetch_optional(&state.pool)
    .await {
        Ok(opt_d) => opt_d,
        Err(err) => {
            tracing::error!(
                err = ?err,
                local_err = ?Status::SqlQueryWrongLogic,
                failed_data = ?(inn, kpp),
                "FUN get_companys_by_inn_kpp FAILED BY SQL QUERY GET COMPANY"
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };

    let dto = match dto_option {
        Some(d) => d,
        None => return Ok(None)
    };

    match dto.try_into() {
        Ok(company) => Ok(Some(company)),
        Err(err) => {
            tracing::error!(
                err = ?err,
                failed_data = ?(inn, kpp),
                "FUN get_companys_by_inn_kpp FAILED BY MAPPING COMPANY"
            );
            Err(err)
        }
    }

}
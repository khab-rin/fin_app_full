use std::sync::Arc;

use shared_lib::Status;
use shared_lib::sql_models::company::implements::{Company, CompanyDto};
use shared_lib::primitives::frozen::implements::{BoxUuid, CompInn, Kpp, CompStatus, CompType, DateTime};

use crate::config::BackApiState;


pub(crate) async fn add_company(
    state: &Arc<BackApiState>,
    company: &Company
) -> Result<Company, Status> {
    let company_dto_option = match sqlx::
        query_file_as!(
            CompanyDto,
            "src/db/sql_queries/companys/add/company.sql",
            company.comp_inn.as_ref(),
            company.kpp.as_ref(),
            company.comp_type.as_str(),
            company.comp_status.as_str(),
            serde_json::to_value(&company.metadata).unwrap_or_default()
        ).fetch_optional(&state.pool).await {
            Ok(dto) => dto,
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::SqlQueryWrongLogic,
                    "FUN add_company FAILED BY WRONG QUERY LOGIC"
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        };

    let company_dto = match company_dto_option {
        Some(c) => c,
        None => {
            tracing::error!(
                    local_err = ?Status::SqlQueryWrongLogic,
                    "FUN add_company GET NONE BY SQL QUERY GET company BY company"
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };

    match company_dto.try_into() {
        Ok(company) => Ok(company),
        Err(err) => {
            tracing::error!(
                err = ?err,
                "FUN add_company GET NONE BY SQL QUERY GET company BY company"
            );
            Err(Status::SqlQueryWrongLogic)
        }
    }


}
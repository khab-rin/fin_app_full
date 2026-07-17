use std::sync::Arc;

use shared_lib::Status;
use shared_lib::sql_models::company::implements::{Company, CompanyDto};
use shared_lib::primitives::frozen::implements::{BoxUuid, CompInn, Kpp, CompStatus, CompType, DateTime};

use crate::config::BackApiState;
use crate::db::sql_queries::companys::get::company_by_inn_kpp::get_company_by_inn_kpp;


pub(crate) async fn add_company(
    state: &BackApiState,
    new_company: &Company
) -> Result<Company, Status> {

    let exist_company_option = match get_company_by_inn_kpp(
            state, 
            &new_company.comp_inn, 
            &new_company.kpp).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN add_company FAILED BY FUN get_company_by_inn_kpp"
            );
            return Err(err);
        }
    };

    let company = match exist_company_option {
        Some(mut c) => {
            c.comp_status = new_company.comp_status.clone();
            c.comp_type = new_company.comp_type.clone();
            c.metadata.bank_acc.extend(new_company.metadata.bank_acc.clone());

            c
        }
        None => new_company.clone()
    };


    let company_dto = match sqlx::
        query_file_as!(
            CompanyDto,
            "src/db/sql_queries/companys/add/company.sql",
            company.comp_inn.as_ref(),
            company.kpp.as_ref(),
            company.comp_type.as_str(),
            company.comp_status.as_str(),
            serde_json::to_value(&company.metadata).unwrap_or_default()
        ).fetch_one(&state.pool_fast).await {
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
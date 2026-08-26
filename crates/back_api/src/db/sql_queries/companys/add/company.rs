use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};
use shared_lib::primitives::frozen::text::{BoxUuid, CompInn, Kpp, CompStatus, CompType, DateTime};

use crate::config::BackApiState;
use crate::db::sql_queries::companys::get::company_by_inn_kpp::get_company_by_inn_kpp;


pub(crate) async fn add_company(
    state: &BackApiState,
    new_company: &Company
) -> Result<Company, Status> {

    let exist_company_option = get_company_by_inn_kpp(
        state, 
        &new_company.comp_inn, 
        &new_company.kpp)
        .await
        .map_err(|err| err.process_err(err, ""))?; 

    let company = match exist_company_option {
        Some(mut c) => {
            c.comp_status = new_company.comp_status.clone();
            c.comp_type = new_company.comp_type.clone();
            c.metadata.bank_acc.extend(new_company.metadata.bank_acc.clone());

            c
        }
        None => new_company.clone()
    };


    let company_dto = sqlx::query_file_as!(
            CompanyDto,
            "src/db/sql_queries/companys/add/company.sql",
			company.comp_id.as_ref(),
            company.comp_inn.as_ref(),
            company.kpp.as_ref(),
            company.comp_type.as_str(),
            company.comp_status.as_str(),
            serde_json::to_value(&company.metadata).unwrap_or_default()
        ).fetch_one(&state.pool_fast)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;
    
    let company: Company = company_dto
        .try_into()
        .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;

    Ok(company)

}
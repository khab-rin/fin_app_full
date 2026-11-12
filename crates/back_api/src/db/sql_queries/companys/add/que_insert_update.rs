use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};
use shared_lib::primitives::frozen::text::{BoxUuid, CompInn, Kpp, CompStatus, CompType, DateTime};


pub(crate) async fn sql_insert_update_company_function<'t, E>(
    conn: E,
    company: &Company
) -> Result<Company, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Postgres>

{
    let company_dto = sqlx::query_file_as!(
            CompanyDto,
            "src/db/sql_queries/companys/add/que_insert_update.sql",
			*company.comp_id,
            *company.comp_inn,
            *company.kpp,
            company.comp_type.as_str(),
            company.comp_status.as_str(),
            serde_json::to_value(&company.metadata).unwrap_or_default()
        ).fetch_one(conn)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;
    
    let company: Company = company_dto
        .try_into()
        .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;

    Ok(company)

}
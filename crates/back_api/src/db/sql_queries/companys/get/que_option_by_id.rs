use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{CompInn, Kpp, BoxUuid, CompStatus, CompType, DateTime};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};


pub(crate) async fn get_company_option_by_id<'t, E>(
    conn: E,
    comp_id: &BoxUuid
) -> Result<Option<Company>, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{

    let dto_option = sqlx::query_file_as!(
        CompanyDto,
        "src/db/sql_queries/companys/get/que_option_by_id.sql",
        **comp_id,
    ).fetch_optional(conn)
    .await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	match dto_option {
		Some(dto) => {
			let company: Company = dto.try_into().map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;
			Ok(Some(company))
		},
		None => Ok(None)
	}
}
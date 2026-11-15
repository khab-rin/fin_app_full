use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};
use shared_lib::primitives::frozen::text::{CompInn, Kpp, BoxUuid, CompType, CompStatus, DateTime};


pub async fn sql_get_company_by_inn_kpp_function<'t, E>(
	conn: E,
	comp_inn: &CompInn,
	kpp: &Kpp
) -> Result<Option<Company>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Postgres>
{
	let dto_option = sqlx::query_file_as!(
		CompanyDto,
		"src/db/sql_queries/companys/get/que_get_by_inn_kpp.sql",
		comp_inn as &CompInn,
		kpp as &Kpp
	).fetch_optional(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	if let Some(dto) = dto_option {
		let company: Company = dto.try_into().map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;
		Ok(Some(company))
	} else {
		Ok(None)
	}
}
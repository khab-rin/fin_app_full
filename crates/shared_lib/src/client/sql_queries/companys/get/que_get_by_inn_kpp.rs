
use crate::{Status, ProcessError};
use crate::primitives::frozen::text::{Kpp, CompInn, BoxUuid, CompStatus, CompType, DateTime};
use crate::sql_models::company::implements::{Company, CompanyDto};


pub async fn sql_get_company_by_inn_kpp_function<'t, E>(
    conn: E,
    comp_inn: &CompInn,
	kpp: &Kpp
) -> Result<Option<Company>, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Sqlite>
{

	let dto_option = sqlx::query_file_as!(
		CompanyDto,
		"src/client/sql_queries/companys/get/que_get_by_inn_kpp.sql",
		comp_inn,
		kpp
	).fetch_optional(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	if let Some(dto) = dto_option {
		Ok(Some(dto.try_into().map_err(|err:serde_json::Error| err.process_err(Status::MappingError, ""))?))
	} else {
		Ok(None)
	}
    
}

use crate::{Status, ProcessError};
use crate::static_data::null_elems::NULLS;
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
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;

	if let Some(dto) = dto_option {
		Ok(Some(dto.try_into().map_err(|err:serde_json::Error| err.process_err(Status::MappingError, None))?))
	} else {
		Ok(None)
	}
    
}

pub async fn sql_get_company_ids_by_type_function<'t, E>(
	conn: E,
	comp_type: &CompType
) -> Result<Vec<BoxUuid>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Sqlite> 
{
	let res = sqlx::query_file_scalar!(
		"src/client/sql_queries/companys/get/que_ids_by_comp_type.sql",
		comp_type
	).fetch_all(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?
	.into_iter()
	.filter(|x| x != NULLS.id.comp_id)
	.collect();

	Ok(res)

}


pub async fn sql_get_all_companys_function<'t, E>(
    conn: E,
) -> Result<Vec<Company>, Status> 
where E: sqlx::Executor<'t, Database = sqlx::Sqlite>
{
	sqlx::query_file_as!(
		CompanyDto,
		"src/client/sql_queries/companys/get/que_get_all_companys.sql",
	).fetch_all(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?
	.into_iter()
	.filter(|x| x.comp_id != NULLS.id.comp_id)
	.map(|x| {
		x.try_into()
		.map_err(|err:serde_json::Error| err.process_err(Status::MappingError, None))
	}).collect()

}

pub async fn sql_get_by_inn_function<'t, E>(
	conn: E,
	comp_inn: &CompInn
) -> Result<Vec<Company>, Status>
where E: sqlx::Executor<'t, Database = sqlx::Sqlite>
{

	let dtos = sqlx::query_file_as!(
		CompanyDto,
		"src/client/sql_queries/companys/get/que_get_by_inn.sql",
		comp_inn
	).fetch_all(conn).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, None))?;

	dtos.into_iter().map(|x| x
		.try_into()
		.map_err(|err: serde_json::Error| err.process_err(Status::MappingError, None))
	).collect()

}
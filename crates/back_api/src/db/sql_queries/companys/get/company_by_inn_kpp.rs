use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{CompInn, Kpp, BoxUuid, CompStatus, CompType, DateTime};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};

use crate::config::BackApiState;
use crate::db::parsers::dadata::inn_kpp_query::parse_company_by_inn_kpp;
use crate::db::sql_queries::companys::add::company::add_company;

pub(crate) async fn get_company_by_inn_kpp(
    state: &BackApiState,
    comp_inn: &CompInn,
    kpp: &Kpp

) -> Result<Option<Company>, Status> {

    let dto_option = sqlx::query_file_as!(
        CompanyDto,
        "src/db/sql_queries/companys/get/company_by_inn_kpp.sql",
        comp_inn,
        kpp
    ).fetch_optional(&state.pool_fast)
    .await
    .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	if let Some(dto) = dto_option {
		let company = dto
			.try_into()
			.map_err(|err: serde_json::Error| err.process_err(Status::SqlQueryWrongLogic, ""))?;
		return Ok(Some(company));
	}

    let company_option = parse_company_by_inn_kpp(state, comp_inn, kpp)
		.await
		.map_err(|err| err.process_err(err, ""))?;

	let company = if let Some(c) = company_option {
		c
	} else {
		return Ok(None)
	};

	add_company(state, &company).await.map_err(|err| err.process_err(err, ""))?;

	Ok(Some(company))

}
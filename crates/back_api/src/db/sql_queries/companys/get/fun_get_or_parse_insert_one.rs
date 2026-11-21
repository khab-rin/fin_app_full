use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::company::implements::{Company};
use shared_lib::primitives::frozen::text::{CompInn, Kpp};

use crate::config::BackApiState;

use crate::db::parsers::dadata::get_new_company::get_dadata_company;
use crate::db::sql_queries::companys::get::que::sql_get_company_by_inn_kpp_function;
use crate::db::sql_queries::companys::add::que::sql_insert_update_company_function;

pub async fn get_or_parse_insert_one_company_function(
	state: &BackApiState,
	comp_inn: &CompInn,
	kpp: &Kpp
) -> Result<Option<Company>, Status> {

	let server_company = sql_get_company_by_inn_kpp_function(
		&state.pool_fast,
		comp_inn,
		kpp
	).await.map_err(|err| err.process_err(err, ""))?;

	if let Some(c) = server_company { return Ok(Some(c));}

	let dadata_company = get_dadata_company(state, comp_inn, kpp).await
		.map_err(|err| err.process_err(err, ""))?;

	if let Some(c) = dadata_company {
		sql_insert_update_company_function(&state.pool_fast, &c).await
			.map_err(|err| err.process_err(err, ""))?;
		return Ok(Some(c));
	}

	Ok(None)
}
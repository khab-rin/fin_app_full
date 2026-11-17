use crate::{Status, ProcessError, ClientState};
use crate::sql_models::company::implements::{Company, CompCrateData};
use crate::primitives::frozen::text::{CompInn, Kpp};
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::back_api::post_query::post_query_back_api;
use crate::client::sql_queries::companys::get::que_get_by_inn_kpp::sql_get_company_by_inn_kpp_function;
use crate::client::sql_queries::companys::add::que_insert_update::sql_insert_update_company_query;

pub async fn get_or_parse_insert_company_function(
	state: &ClientState,
	comp_inn: &CompInn,
	kpp: &Kpp
) -> Result<Option<Company>, Status> {

	let session = state.get_session().await
		.map_err(|err| err.process_err(err, ""))?;

	let exist_local_company = sql_get_company_by_inn_kpp_function(&session.local_db, comp_inn, kpp).await
		.map_err(|err| err.process_err(err, ""))?;

	if exist_local_company.is_some() {
		return Ok(exist_local_company);
	}

	let data = CompCrateData{comp_inn: comp_inn.clone(), kpp: kpp.clone()};

	let response = post_query_back_api(
			state, 
			state.config.get_sql_fast(),
			ApiRoutes::SqlCompanyGetOrParseInsertOne, 
			&data)
		.await
		.map_err(|err| err.process_err(err, ""))?;

	let result: Option<Company> = response.json().await
		.map_err(|err| err.process_err(Status::MappingError, ""))?;

	if let Some(comp) = &result {
		sql_insert_update_company_query(&session.local_db, comp).await
			.map_err(|err| err.process_err(err, ""))?;
	}

	Ok(result)

}
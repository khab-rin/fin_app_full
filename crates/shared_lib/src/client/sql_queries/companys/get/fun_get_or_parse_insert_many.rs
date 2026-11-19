use crate::{Status, ProcessError, ClientState};
use crate::sql_models::company::implements::{Company, CompCrateData};
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::back_api::post_query::post_query_back_api;
use crate::client::sql_queries::companys::get::que::sql_get_company_by_inn_kpp_function;
use crate::client::sql_queries::companys::add::que_insert_update::sql_insert_update_company_query;

pub async fn get_or_parse_insert_companys_function(
	state: &ClientState,
	pairs: Vec<CompCrateData>
) -> Result<(), Status> {

	let session = state.get_session().await
		.map_err(|err| err.process_err(err, ""))?;

	let mut conn = session.local_db.acquire().await
		.map_err(|err| err.process_err(Status::SqlPoolErr, ""))?;

	let mut new_pairs: Vec<CompCrateData> = vec![];

	for pair in pairs {
		let local_comp_option = sql_get_company_by_inn_kpp_function(&mut *conn, &pair.comp_inn, &pair.kpp)
			.await.map_err(|err| err.process_err(err, ""))?;
		if local_comp_option.is_some() {continue;}
		new_pairs.push(pair);
	}

	drop(conn);

	let mut new_companys: Vec<Company> = vec![];

	while !new_pairs.is_empty() {
		let mut pairs_chunk: Vec<CompCrateData> = vec![];
		while pairs_chunk.len() < 30 && let Some(pair) = new_pairs.pop() {
			pairs_chunk.push(pair);
		}
		let response = post_query_back_api(
			state, 
			state.config.get_sql_long(), 
			ApiRoutes::SqlCompanyGetOrParseInsertMany, 
			&pairs_chunk
		).await.map_err(|err| err.process_err(err, ""))?;

		let server_companys: Vec<Company> = response.json().await
			.map_err(|err| err.process_err(Status::MappingError, ""))?;

		new_companys.extend(server_companys);
	}

	let mut tx = session.local_db.begin().await
		.map_err(|err| err.process_err(Status::SqlPoolErr, ""))?;

	for new_company in new_companys {
		sql_insert_update_company_query(&mut *tx, &new_company).await
			.map_err(|err| err.process_err(err, ""))?;
	}

	tx.commit().await.map_err(|err| err.process_err(Status::SqlCommitErr, ""))

}
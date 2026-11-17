use shared_lib::{Status, ProcessError, LL};
use shared_lib::sql_models::company::implements::{CompCrateData, Company};


use crate::config::BackApiState;

use crate::db::sql_queries::companys::get::que_get_by_inn_kpp::sql_get_company_by_inn_kpp_function;
use crate::db::sql_queries::companys::add::que_insert_update::sql_insert_update_company_function;
use crate::db::parsers::dadata::get_new_company::get_dadata_company;

pub async fn get_or_parse_insert_company_many_function(
	state: &BackApiState,
	data: Vec<CompCrateData>
) -> Result<Vec<Company>, Status> {

	let mut server_companys: Vec<Company> = vec![];
	let mut new_companys: Vec<Company> = vec![];
	let mut new_keys: Vec<CompCrateData> = vec![];

	let mut conn = state.pool_fast.acquire().await
		.map_err(|err| err.process_err(Status::SqlPoolErr, ""))?;

	for pair in data {
		let server_company_option = sql_get_company_by_inn_kpp_function(
			&mut *conn, &pair.comp_inn, &pair.kpp
		).await.map_err(|err| err.process_err(err, ""))?;
		if let Some(c) = server_company_option {
			server_companys.push(c);
		} else {
			new_keys.push(pair);
		}
	}

	for key in new_keys {
		let new_company_option = get_dadata_company(state, &key.comp_inn, &key.kpp)
			.await.map_err(|err| err.process_err(err, ""))?;
		if let Some(new_company) = new_company_option {
			sql_insert_update_company_function(&mut *conn, &new_company).await
				.map_err(|err| err.process_err(err, ""))?;
			new_companys.push(new_company);
		} else {
			Status::log(LL::Info, &format!("Company not founded in datata, comp_inn = {:?}, kpp = {:?}", key.comp_inn, key.kpp));
		}
	}

	server_companys.extend(new_companys);

	Ok(server_companys)
}
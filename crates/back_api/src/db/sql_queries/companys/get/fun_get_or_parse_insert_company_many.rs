use futures::future::try_join_all;

use shared_lib::{Status, ProcessError, LL};
use shared_lib::sql_models::company::implements::{CompCrateData, Company};


use crate::config::BackApiState;

use crate::db::sql_queries::companys::get::que::sql_get_company_by_inn_kpp_function;
use crate::db::sql_queries::companys::add::que::sql_insert_update_company_function;
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
	drop(conn);

	let mut dadata_tasks = Vec::with_capacity(new_keys.len());

	for key in new_keys {
		let future = async move {
			let new_company_option = get_dadata_company(state, &key.comp_inn, &key.kpp)
				.await.map_err(|err| err.process_err(err, ""))?;
			if new_company_option.is_none() {
				Status::log(LL::Info, &format!("Company not founded in datata, comp_inn = {:?}, kpp = {:?}", key.comp_inn, key.kpp));
			}
			Ok::<_, Status>(new_company_option)
		};
		dadata_tasks.push(future);
	}

	let mut dadata_companys = try_join_all(dadata_tasks).await
		.map_err(|err| err.process_err(err, ""))?;

	while let Some(comp_option) = dadata_companys.pop() {
		if let Some(new_company) = comp_option {
			new_companys.push(new_company);
		}
	}

	let mut tx = state.pool_fast.begin().await.map_err(|err| err.process_err(Status::SqlPoolErr, ""))?;

	for new_company in new_companys.iter() {
		sql_insert_update_company_function(&mut *tx, new_company).await
			.map_err(|err| err.process_err(err, ""))?;
	}

	tx.commit().await.map_err(|err| err.process_err(Status::SqlCommitErr, ""))?;

	server_companys.extend(new_companys);

	Ok(server_companys)
}
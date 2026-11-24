use std::collections::HashMap;
use futures::future::try_join_all;

use shared_lib::{ProcessError, Status};
use shared_lib::service::database::service::{DatabaseStep, DatabaseInfo};
use shared_lib::sql_models::company::implements::Company;
use shared_lib::primitives::frozen::text::{CompInn, Kpp};

use crate::config::BackApiState;
use crate::db::sql_queries::companys::get::que::{get_company_option_by_id};
use crate::db::sql_queries::companys::add::que::sql_insert_update_company_function;
use crate::db::parsers::dadata::get_new_company::get_dadata_company;

pub async fn sql_company_sync(
	state: &BackApiState,
	load: &DatabaseStep
) -> Result<DatabaseStep, Status> {

	let failed_result = DatabaseStep::TryLater { text: DatabaseInfo::BackApiError };
	
	let local_companys = match load {
		DatabaseStep::SyncCompanysToServer {companys,} => {
			companys
		},
		_ => {
			Status::Tech.process_err(Status::SystemLogicErr, None);
			return Ok(failed_result);
		}
	};

	let mut conn = match state.pool_fast.acquire().await {
		Ok(t) => t,
		Err(err) => {
			err.process_err(Status::SqlPoolErr, None);
			return Ok(failed_result);
		}
	};

	let mut update_companys: HashMap<(CompInn, Kpp), Company> = HashMap::new();
	let mut del_companys: Vec<Company> = vec![];
	let mut server_companys: Vec<Company> = vec![];
	let mut dadata_futures = vec![];

	for local_company in local_companys {
		let serv_comp_option = get_company_option_by_id(&mut *conn, &local_company.comp_id).await
			.map_err(|err| err.process_err(err, None))?;
		match serv_comp_option {
			Some(mut server_company) => {
				server_company.merge_acc(local_company);
				server_companys.push(server_company);
			},
			None => {
				let comp_inn = &local_company.comp_inn;
				let kpp = &local_company.kpp;
				let future = async move {
					let new_company_option = get_dadata_company(state, comp_inn, kpp)
						.await.map_err(|err| err.process_err(err, None))?;
					Ok::<_, Status>(((comp_inn.clone(), kpp.clone()), new_company_option))
				};
				dadata_futures.push(future);
				update_companys.insert((comp_inn.clone(), kpp.clone()), local_company.clone());
			}
		}
	}

	drop(conn);

	let mut dadata_companys = match try_join_all(dadata_futures).await {
		Ok(res) => res,
		Err(err) => {
			err.process_err(err, None);
			return Ok(DatabaseStep::from_err(&err));
		}
	};

	while let Some((key, dadata_company_option)) = dadata_companys.pop() {
		if let Some(local_company) = update_companys.remove(&key) {
			if let Some(mut dadata_company) = dadata_company_option {
				dadata_company.merge_acc(&local_company);
				server_companys.push(dadata_company);
			} else {
				del_companys.push(local_company);
			}
		} else if let Some(dadata_company) = dadata_company_option {
			server_companys.push(dadata_company);
		}
	}

	let mut tx = match state.pool_fast.begin().await {
		Ok(t) => t,
		Err(err) => {
			err.process_err(Status::SqlPoolErr, None);
			return Ok(failed_result);
		}
	};

	for server_company in server_companys.iter() {
		if let Err(err) = sql_insert_update_company_function(&mut *tx, server_company).await {
			err.process_err(err, None);
			return Ok(DatabaseStep::from_err(&err));
		}
	}

	match tx.commit().await {
		Ok(_) => Ok(DatabaseStep::SyncCompanysToClient { del_companys, server_companys }),
		Err(err) => {
			err.process_err(Status::SqlCommitErr, None);
			Ok(failed_result)
		}
	}
}
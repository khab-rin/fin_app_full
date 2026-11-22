use crate::{Status, ProcessError, ClientState};
use crate::sql_models::company::implements::Company;
use crate::primitives::frozen::text::{CompInn, Kpp};

use crate::client::sql_queries::companys::get::que::sql_get_by_inn_function;


pub async fn try_get_by_inn_kpp_then_inn_function(
	state: &ClientState,
	comp_inn: &CompInn,
	kpp: &Kpp
) -> Result<Option<Company>, Status> {


	let session = state.get_session().await
		.map_err(|err| err.process_err(err, ""))?;


	let all_companys = sql_get_by_inn_function(
		&session.local_db, comp_inn).await
		.map_err(|err| err.process_err(err, ""))?;
	
	// for (i, company) in all_companys.iter().enumerate() {
	// 	Status::log(LL::Debug, &format!("\nnum = {}\n, name = {:?}\n, type = {:?}\n, que_inn = {}\n, que_kpp = {}\n, list_inn = {}\n, list_kpp = {}\n, metadata_kpp = {:?}\n",
	// 		i,
	// 		company.metadata.comp_name, 
	// 		company.metadata.branch_type,
	// 		comp_inn,
	// 		kpp,
	// 		company.comp_inn,
	// 		company.kpp,
	// 		company.metadata.kpp
		
	// 	));
	// }

	for company in all_companys.iter() {
		if company.kpp == kpp {
			return Ok(Some(company.clone()));
		}
	}

	for company in all_companys.iter() {

		if let Some(br_type) = &company.metadata.branch_type && br_type.as_ref() == "MAIN" {
			// Status::log(LL::Debug, &format!("equal, type = {:?}", br_type.as_ref()));
			return Ok(Some(company.clone()));
		} else {
			// Status::log(LL::Debug, &format!("not equal, type = {:?}", company.metadata.branch_type));
		}
	}

	if let Some(company) = all_companys.first() {
		return Ok(Some(company.clone()));
	}

	Ok(None)
}
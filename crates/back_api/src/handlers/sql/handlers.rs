use std::sync::Arc;

use axum::{extract::State, Json};

use shared_lib::service::reports::service::ReportStep;
use shared_lib::{ProcessError, Status};
use shared_lib::primitives::frozen::text::PersInn;
use shared_lib::sql_models::operation::implements::Operation;
use shared_lib::sql_models::person::implements::Person;
use shared_lib::sql_models::company::implements::{Company, CompCrateData};
use shared_lib::sql_models::contracts::implements::Contract;

use crate::config::BackApiState;
use crate::db::sql_queries::persons::get::person_by_inn::get_person_by_inn;
use crate::db::sql_queries::companys::get::fun_get_or_parse_insert_company_many;
use crate::db::sql_queries::contracts::add::new_contr::add_contract;
use crate::db::sql_queries::operations::add::add_many::add_operations_many;
use crate::db::sql_queries::companys::sync::sql_company_sync;
use crate::db::sql_queries::companys::get::fun_get_or_parse_insert_one::get_or_parse_insert_one_company_function;
use crate::db::sql_queries::companys::get::fun_get_or_parse_insert_company_many::get_or_parse_insert_company_many_function;


pub(crate) async fn get_person_by_inn_handler(
    State(state): State<Arc<BackApiState>>,
    Json(pers_inn): Json<PersInn>
) -> Result<Json<Option<Person>>, Status> {

    let res = get_person_by_inn(&state, &pers_inn).await?;

    Ok(Json(res))

}

pub async fn get_or_parse_insert_company_one_handler(
	State(state): State<Arc<BackApiState>>,
	Json(data): Json<CompCrateData>
) -> Result<Json<Option<Company>>, Status> {

	let CompCrateData { comp_inn, kpp } = data;

	let res = get_or_parse_insert_one_company_function(&state, &comp_inn, &kpp).await
		.map_err(|err| err.process_err(err, ""))?;

	Ok(Json(res))
}

pub async fn get_or_parse_insert_company_many_handler(
	State(state): State<Arc<BackApiState>>,
	Json(data): Json<Vec<CompCrateData>>
) -> Result<Json<Vec<Company>>, Status> {

	let res = get_or_parse_insert_company_many_function(
		&state, data
	).await.map_err(|err| err.process_err(err, ""))?;

	Ok(Json(res))

}

pub async fn sql_new_companys_handler(
    State(state): State<Arc<BackApiState>>,
    Json(payload): Json<Vec<CompCrateData>>
) -> Result<Json<Vec<Company>>, Status> {
 
    let res = get_or_parse_insert_company_many_function(&state, payload).await?;

    Ok(Json(res))
}

pub async fn sql_add_new_contract_handler(
    State(state): State<Arc<BackApiState>>,
    Json(data): Json<Contract>
) -> Result<Json<Contract>, Status> {

    let contr = add_contract(&state, data).await?;

    Ok(Json(contr))
}

pub async fn sql_operations_add_many_handler(
	State(state): State<Arc<BackApiState>>,
	Json(data): Json<Vec<Operation>>
) -> Result<(), Status> {

	add_operations_many(&state, data).await
		.map_err(|err| err.process_err(err, ""))?;

	Ok(())

}

pub async fn sql_sync_handler(
	State(state): State<Arc<BackApiState>>,
	Json(data): Json<ReportStep>
) -> Result<Json<ReportStep>, Status> {

	match &data {
		ReportStep::SyncCompanysToServer { .. } => {
			let res = sql_company_sync(&state, &data).await
				.map_err(|err| err.process_err(err, ""))?;
			Ok(Json(res))
		},
		_ => Err(Status::Unknown)
	}

}
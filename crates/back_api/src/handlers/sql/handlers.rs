use std::sync::Arc;

use axum::{extract::State, Json};

use shared_lib::service::database::service::{DatabaseInfo, DatabaseStep};
use shared_lib::{ProcessError, Status, LL};
use shared_lib::primitives::frozen::text::PersInn;
use shared_lib::sql_models::operation::implements::Operation;
use shared_lib::sql_models::person::implements::Person;
use shared_lib::sql_models::company::implements::{Company, CompCrateData};
use shared_lib::sql_models::contracts::implements::Contract;

use crate::config::BackApiState;
use crate::db::sql_queries::persons::get::que::sql_get_person_by_inn_function;
use crate::db::sql_queries::contracts::add::fun_get_or_insert::insert_contract_if_not_exist_function;
use crate::db::sql_queries::operations::add::fun_insert_operations::insert_operations_function;
use crate::db::sql_queries::companys::sync::sql_company_sync;
use crate::db::sql_queries::persons::sync::sync_all_persons;
use crate::db::sql_queries::contracts::sync::sync_all_contracts;
use crate::db::sql_queries::operations::sync::sync_all_operations;
use crate::db::sql_queries::companys::get::fun_get_or_parse_insert_one::get_or_parse_insert_one_company_function;
use crate::db::sql_queries::companys::get::fun_get_or_parse_insert_company_many::get_or_parse_insert_company_many_function;

pub(crate) async fn get_person_by_inn_handler(
    State(state): State<Arc<BackApiState>>,
    Json(pers_inn): Json<PersInn>
) -> Result<Json<Option<Person>>, Status> {

    let res = sql_get_person_by_inn_function(&state.pool_fast, &pers_inn).await?;

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

    let contr = insert_contract_if_not_exist_function(&state, data).await?;

    Ok(Json(contr))
}

pub async fn sql_operations_add_many_handler(
	State(state): State<Arc<BackApiState>>,
	Json(data): Json<Vec<Operation>>
) -> Result<(), Status> {

	insert_operations_function(&state, data).await
		.map_err(|err| err.process_err(err, ""))?;

	Ok(())
}

pub async fn sql_sync_handler(
	State(state): State<Arc<BackApiState>>,
	Json(data): Json<DatabaseStep>
) -> Result<Json<DatabaseStep>, Status> {

	let failed_result = DatabaseStep::TryLater { text: DatabaseInfo::BackApiError };

	match &data {
		DatabaseStep::SyncCompanysToServer { .. } => {
			match sql_company_sync(&state, &data).await {
				Ok(res) => Ok(Json(res)),
				Err(err) => {
					err.process_err(err, "");
					Ok(Json(DatabaseStep::from_err(&err)))
				}
			}	
		},
		DatabaseStep::SyncPersons { .. } => {
			match sync_all_persons(&state, data).await {
				Ok(res) => Ok(Json(res)),
				Err(err) => {
					err.process_err(err, "");
					Ok(Json(DatabaseStep::from_err(&err)))
				}
			}
		},
		DatabaseStep::SyncContracts { .. } => {
			match sync_all_contracts(&state, &data).await {
				Ok(res) => Ok(Json(res)),
				Err(err) => {
					err.process_err(err, "");
					Ok(Json(DatabaseStep::from_err(&err)))
				}
			}
		},
		DatabaseStep::SyncOperations { .. } => {
			match sync_all_operations(&state, &data).await {
				Ok(res) => Ok(Json(res)),
				Err(err) => {
					err.process_err(err, "");
					Ok(Json(DatabaseStep::from_err(&err)))
				}
			}
		},
		_ => {
			Status::log(LL::Error, "WRONG DatabaseStep in sql_sync_handler FUNCTION");
			Ok(Json(failed_result))
		}
	}

}
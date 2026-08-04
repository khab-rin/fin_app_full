use std::sync::Arc;

use axum::{extract::State, Json};

use shared_lib::Status;
use shared_lib::primitives::frozen::text::PersInn;
use shared_lib::sql_models::person::implements::Person;
use shared_lib::sql_models::company::implements::{Company, CompCrateData};

use crate::config::BackApiState;
use crate::db::sql_queries::persons::get::person_by_inn::get_person_by_inn;
use crate::db::sql_queries::companys::add::sync_companys::update_companys;


pub(crate) async fn get_person_by_inn_handler(
    State(state): State<Arc<BackApiState>>,
    Json(pers_inn): Json<PersInn>
) -> Result<Json<Option<Person>>, Status> {
    
    tracing::info!("get_person_by_inn_handler running!");

    let res = get_person_by_inn(&state, &pers_inn).await?;

    Ok(Json(res))

}

pub async fn sql_new_companys_handler(
    State(state): State<Arc<BackApiState>>,
    Json(mut payload): Json<Vec<CompCrateData>>
) -> Result<Json<Vec<Company>>, Status> {
 
    let res = update_companys(&state, payload).await?;

    Ok(Json(res))

}
use std::sync::Arc;

use axum::{extract::State, Json};

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::PersInn;
use shared_lib::sql_models::person::implements::Person;


use crate::config::BackApiState;
use crate::db::sql_queries::persons::get::person_by_inn::get_person_by_inn;


pub(crate) async fn get_person_by_inn_handler(
    State(state): State<Arc<BackApiState>>,
    Json(pers_inn): Json<PersInn>
) -> Result<Json<Option<Person>>, Status> {
    
    let res = get_person_by_inn(&state, &pers_inn).await?;

    Ok(Json(res))

}
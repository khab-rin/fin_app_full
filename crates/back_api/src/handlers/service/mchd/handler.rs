use std::sync::Arc;

use axum::{Json, extract::State};

use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, RegisterMchdData};
use shared_lib::primitives::frozen::implements::CompInn;

use crate::config::BackApiState;
use crate::db::service::mchd::register_mchd::register_mchd;
use crate::db::service::mchd::get_ifns::get_json_data;

pub(crate) async fn register_mchd_hadler(
    State(state): State<Arc<BackApiState>>,
    Json(payload): Json<RegisterMchdData>
) -> Result<Json<MchdStep>, Status> {

    let res = register_mchd(&state, &payload).await?;

    Ok(Json(res))

}

pub(crate) async fn get_ifns_handler(
    State(state): State<Arc<BackApiState>>,
    Json(playload): Json<CompInn>
) -> Result<Json<String>, Status> {

    let res = get_json_data(&state, &playload, ).await?;
    Ok(Json(res))
}
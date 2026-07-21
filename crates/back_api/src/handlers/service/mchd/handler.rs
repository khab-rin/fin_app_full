use std::sync::Arc;

use axum::{Json, extract::State};

use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, RegisterMchdData};
use shared_lib::primitives::frozen::implements::BoxUuid;

use crate::config::BackApiState;
use crate::db::service::mchd::register_mchd::register_mchd;
use crate::db::service::mchd::show_powers::show_powers;

pub(crate) async fn register_mchd_hadler(
    State(state): State<Arc<BackApiState>>,
    Json(payload): Json<RegisterMchdData>
) -> Result<Json<MchdStep>, Status> {

    let res = register_mchd(&state, &payload).await?;

    Ok(Json(res))

}

pub(crate) async fn show_powers_handler(
    State(state): State<Arc<BackApiState>>,
    Json(user_id): Json<BoxUuid>
) -> Result<Json<MchdStep>, Status> {

    let res = show_powers(&state, &user_id).await?;


    Ok(Json(res))
}

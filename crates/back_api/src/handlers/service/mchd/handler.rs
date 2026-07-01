use std::sync::Arc;

use axum::{Json, extract::State};

use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, RegisterMchdData};

use crate::config::BackApiState;
use crate::db::service::mchd::register_mchd::register_mchd;

pub(crate) async fn register_mchd_hadler(
    State(state): State<Arc<BackApiState>>,
    Json(payload): Json<RegisterMchdData>
) -> Result<Json<MchdStep>, Status> {

    let res = register_mchd(&state, &payload).await?;

    Ok(Json(res))

}
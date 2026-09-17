use std::sync::Arc;

use axum::{Json, extract::State};

use shared_lib::{ProcessError, Status};
use shared_lib::service::mchd::home_mchd_power::HomeMchdPower;
use shared_lib::service::mchd::service::{MchdStep, RegisterMchdData};
use shared_lib::primitives::frozen::text::BoxUuid;
use shared_lib::service::reports::service::VerifyPowersResult;

use crate::config::BackApiState;
use crate::db::service::mchd::register_mchd::register_mchd;
use crate::db::service::mchd::show_powers::show_powers;
use crate::db::service::mchd::verify_user_power::verify_user_power_exist;

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

pub(crate) async fn verify_user_power_exist_handler(
    State(state): State<Arc<BackApiState>>,
    Json((user_id, power)): Json<(BoxUuid, HomeMchdPower)>,
) -> Result<Json<VerifyPowersResult>, Status> {

    let res = verify_user_power_exist(&state, user_id, power)
		.await.map_err(|err| err.process_err(err, ""))?;
		

    Ok(Json(res))
}

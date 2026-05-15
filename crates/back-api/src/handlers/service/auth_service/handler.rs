use axum::{Json, extract::State};
use axum_typed_multipart::TypedMultipart;
use std::sync::Arc;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{RegisterResponse, RestoreByTokenRequest};

use crate::config::ApiState;
use crate::db::service::auth_service::by_device_token::get_user;
use crate::db::service::auth_service::registration::register_new_user;
use crate::models::registration::RegistrationRequest;


pub async fn restore_user_by_token_handler(
    State(state): State<Arc<ApiState>>, 
    Json(payload): Json<RestoreByTokenRequest>
) -> Result<Json<RegisterResponse>, Status> {

    let res = get_user(&state, &payload).await?;
    Ok(Json(res))
}

pub async fn register_user_by_crypto_handler(
    State(state): State<Arc<ApiState>>,
    TypedMultipart(payload): TypedMultipart<RegistrationRequest>
) -> Result<Json<RegisterResponse>, Status> {

    let res = register_new_user(&state, payload).await?;

    Ok(Json(res))
}
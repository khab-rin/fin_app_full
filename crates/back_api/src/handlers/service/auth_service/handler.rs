use axum::{Json, extract::State};
use std::sync::Arc;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    TokenDeviceData,
    RegistrationData,
    AuthStep,
    PasswordData
};

use crate::config::BackApiState;
use crate::db::service::auth_service::by_device_token::get_user;
use crate::db::service::auth_service::by_password::restore_session_by_passord;
use crate::db::service::auth_service::registration::register_new_user;


pub async fn restore_by_token_handler(
    State(state): State<Arc<BackApiState>>, 
    Json(payload): Json<TokenDeviceData>
) -> Result<Json<AuthStep>, Status> {

    let res = get_user(&state, &payload).await?;
    Ok(Json(res))
}

pub async fn register_user_by_crypto_handler(
    State(state): State<Arc<BackApiState>>,
    Json(payload): Json<RegistrationData>
) -> Result<Json<AuthStep>, Status> {

    let res = register_new_user(&state, payload).await?;

    Ok(Json(res))
}

pub async fn restore_by_password_handler(
    State(state) : State<Arc<BackApiState>>,
    Json(payload): Json<PasswordData>
) -> Result<Json<AuthStep>, Status> {
    let res = restore_session_by_passord(&state, &payload).await?;

    Ok(Json(res))
}
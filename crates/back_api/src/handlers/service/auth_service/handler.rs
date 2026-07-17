use std::sync::Arc;

use axum::{Json, extract::State};

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    TokenDeviceData,
    RegistrationData,
    AuthStep,
    PasswordDataClient,
    ExternalDeviceData, RegInitData
};

use crate::config::BackApiState;
use crate::db::service::auth_service::by_device_token::restore_session_by_token;
use crate::db::service::auth_service::by_password::restore_session_by_passord;
use crate::db::service::auth_service::registration::register_new_user;
use crate::db::service::auth_service::by_tel_call::make_session_by_tel_call;
use crate::db::service::auth_service::init_user::init_user;


pub async fn init_user_handler(
    State(state) : State<Arc<BackApiState>>,
    Json(payload) : Json<RegInitData>
) -> Result<Json<AuthStep>, Status> {

    let res = init_user(&state, &payload).await?;
    
    Ok(Json(res))
}

pub async fn make_session_by_tell_call_handler(
    State(state) : State<Arc<BackApiState>>,
    Json(payload) : Json<ExternalDeviceData>
) -> Result<Json<AuthStep>, Status> {
    let res = make_session_by_tel_call(&state, &payload).await?;

    Ok(Json(res))
}


pub async fn register_user_by_crypto_handler(
    State(state): State<Arc<BackApiState>>,
    Json(payload): Json<RegistrationData>
) -> Result<Json<AuthStep>, Status> {

    tracing::debug!("register_user_by_crypto_handler Running!!!!!!");

    let res = register_new_user(&state, payload).await?;

    Ok(Json(res))
}


pub async fn restore_by_password_handler(
    State(state) : State<Arc<BackApiState>>,
    Json(payload): Json<PasswordDataClient>
) -> Result<Json<AuthStep>, Status> {
    let res = restore_session_by_passord(&state, &payload).await?;

    Ok(Json(res))
}

pub async fn restore_by_token_handler(
    State(state): State<Arc<BackApiState>>, 
    Json(payload): Json<TokenDeviceData>
) -> Result<Json<AuthStep>, Status> {

    let res = restore_session_by_token(&state, &payload).await?;
    Ok(Json(res))
}






use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    AuthInfo, AuthStep, PasswordDataClientShort, RegFilesPathData, RegInitData
};
use shared_lib::primitives::frozen::implements_base::String1_50;
use shared_lib::primitives::frozen::implements::BoxUuid;

use crate::state::ClientState;
use crate::service::auth_service::helper::get_device_id;
use crate::service::auth_service::restore_by_nick::restore_session_by_nick;
use crate::service::auth_service::restore_by_password::restore_by_password;
use crate::service::auth_service::nick_data::get_nick_names;
use crate::service::auth_service::register_step1::register_step1;
use crate::service::auth_service::register_step2::register_step2;
use crate::service::auth_service::make_session_by_tell_call::make_session_by_tel_call;

#[tauri::command]
pub fn cmd_get_device_id() -> Result<BoxUuid, Status> {

    log::debug!("cmd_get_device_id running");

    get_device_id()
}


#[tauri::command]
pub fn cmd_get_nick_names(
    state: tauri::State<'_, ClientState>
) -> Result<Vec<String>, Status> {

    log::debug!("cmd_get_nick_names running");

    get_nick_names(&state)
}




#[tauri::command]
pub async  fn cmd_is_state_active_fast(
    state: tauri::State<'_, ClientState>
) -> Result<bool, Status> {

    log::debug!("cmd_is_state_active_fast running!!!!");

    let session = state.session.lock().await;
    Ok(session.is_some())
}


#[tauri::command]
pub async  fn cmd_is_state_active_init(
    state: tauri::State<'_, ClientState>
) -> Result<AuthStep, Status> {

    log::info!("cmd_is_state_active_init running!!!!");

    let session_ref = state.session.lock().await;
    
    if let Some(ref session) = *session_ref {
        match session.local_db.acquire().await {
            Ok(_) => Ok(AuthStep::SuccessShort { }),
            Err(err) => {
                log::error!(
                    "tech_err = {}, local_err = {}",
                    err, Status::SystemErr
                );
                Ok(AuthStep::Loading { text: AuthInfo::LoadingInfo})
            }
        }
    } else {
        Ok(AuthStep::Loading { text: AuthInfo::LoadingInfo})
    }
}

#[tauri::command]
pub async fn cmd_logout(
    state: tauri::State<'_, ClientState>
) -> Result<(), Status> {

    log::debug!("cmd_logout running!!!!");

    let mut session_ref = state.session.lock().await;
    *session_ref = None;
    Ok(())
}


#[tauri::command]
pub async  fn cmd_register_step1(
    state: tauri::State<'_, ClientState>,
    data: RegInitData
) -> Result<AuthStep, Status> {

    log::debug!("cmd_register_step1 running");

    let res = register_step1(&state, &data).await;

    res
}


#[tauri::command]
pub async  fn cmd_register_step2(
    state: tauri::State<'_, ClientState>,
    data: RegFilesPathData
) -> Result<AuthStep, Status> {

    log::debug!("cmd_register_step2 running");

    let res = register_step2(&state, &data).await;

    res
}


#[tauri::command]
pub async fn cmd_session_by_nick(
    state: tauri::State<'_, ClientState>,
    nick: String
) -> Result<AuthStep, Status> {

    log::info!("cmd_session_by_nick running!!!!");

    match restore_session_by_nick(&state, &nick).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(AuthStep::TryLater { text: AuthInfo::ClientApiSystemError })
    }
}


#[tauri::command]
pub async fn cmd_session_by_password(
    state: tauri::State<'_, ClientState>,
    data: PasswordDataClientShort
) -> Result<AuthStep, Status> {

    log::debug!("cmd_session_by_password running!!!!");

    restore_by_password(&state, &data).await
}


#[tauri::command]
pub async fn cmd_session_by_tel_call(
    state: tauri::State<'_, ClientState>,
    external_id: String,
) -> Result<AuthStep, Status> {
    
    let res = make_session_by_tel_call(&state, &external_id).await?;
    
    Ok(res)

}
use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    AuthStep, IngoingData, PasswordDataClientShort, SvelteRegistrationData, TextInfo
};
use shared_lib::service::auth_service::client_state::NickData;
use shared_lib::primitives::frozen::implements_base::String1_50;

use crate::state::ClientState;
use shared_lib::primitives::svelte_validate::SvelteValidator;
use crate::service::auth_service::restore_by_nick::restore_session_by_nick;
use crate::service::auth_service::restore_by_password::restore_by_password;
use crate::service::process::bank_statement::proceed::process_statement;
use crate::service::auth_service::nick_data::get_nicknames;
use crate::service::auth_service::ingoing_data::make_ingoing_doc;
use crate::service::auth_service::registration::register_user;
use crate::service::auth_service::make_session_by_tell_call::make_session_by_tel_call;

#[tauri::command]
pub async fn cmd_process_bank_statement(
    state: tauri::State<'_, ClientState>,
    path: String
) -> Result<(), Status> {

    log::info!("cmd_process_bank_statement running");

    match process_statement(&state, path).await {
        Ok(_) => {
            Ok(())
        }
        Err(err) => Err(err)
    }
}


#[tauri::command]
pub async  fn cmd_is_state_active_init(
    state: tauri::State<'_, ClientState>
) -> Result<AuthStep, Status> {

    log::debug!("cmd_is_state_active_init running!!!!");

    log::debug!("{:?}", state.config.sqlite_options.duration);

    let session_ref = state.session.lock().await;
    
    if let Some(ref session) = *session_ref {
        match session.local_db.acquire().await {
            Ok(_) => Ok(AuthStep::SuccessShort {  }),
            Err(err) => {
                log::error!(
                    "tech_err = {}, local_err = {}",
                    err, Status::SystemErr
                );
                Ok(AuthStep::Loading { text: TextInfo::LoadingInfo})
            }
        }
    } else {
        Ok(AuthStep::Loading { text: TextInfo::LoadingInfo})
    }
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
pub async fn cmd_logout(
    state: tauri::State<'_, ClientState>
) -> Result<(), Status> {

    log::debug!("cmd_logout running!!!!");

    let mut session_ref = state.session.lock().await;
    *session_ref = None;
    Ok(())
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
pub async fn cmd_session_by_nick(
    state: tauri::State<'_, ClientState>,
    nick: String1_50
) -> Result<AuthStep, Status> {

    log::info!("cmd_session_by_nick running!!!!");

    match restore_session_by_nick(&state, &nick).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError })
    }
}

#[tauri::command]
pub fn cmd_validate_field(
    type_value: SvelteValidator,
    value: String
) -> Result<bool, Status> {
    type_value.validate_svelte_field(&value)
}


#[tauri::command]
pub fn cmd_get_nick_names(
    state: tauri::State<'_, ClientState>
) -> Result<NickData, Status> {

    log::debug!("cmd_get_nick_names running");

    get_nicknames(&state)
}

#[tauri::command]
pub async  fn cmd_make_ingoing_doc(
    state: tauri::State<'_, ClientState>,
    data: IngoingData
) -> Result<Vec<u8>, Status> {

    log::debug!("cmd_make_ingoing_doc running");

    let res = make_ingoing_doc(&state, &data).await;
    if res.is_err() {
        log::debug!("cmd_make_ingoing_doc failed");
    }

    res
}

#[tauri::command]
pub async fn cmd_register_user(
    state: tauri::State<'_, ClientState>,
    data: SvelteRegistrationData
) -> Result<AuthStep, Status> {

    log::debug!("cmd_register_user running");
    
    let res = register_user(&state, data).await;

    res
}

#[tauri::command]
pub async fn cmd_session_by_tel_call(
    state: tauri::State<'_, ClientState>,
    external_id: String,
    nick: String1_50
) -> Result<AuthStep, Status> {
    
    let res = make_session_by_tel_call(&state, &external_id, &nick).await?;
    
    Ok(res)

}
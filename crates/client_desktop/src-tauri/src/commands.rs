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

#[tauri::command]
pub async fn cmd_process_bank_statement(
    state: tauri::State<'_, ClientState>,
    path: String
) -> Result<(), Status> {

    match process_statement(&state, path).await {
        Ok(res) => {
            std::println!("ОБЩИЙ_РЕЗУЛЬТАТ!!!");
            std::println!("{:?}", res.companys_curt);
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

    std::println!("{:?}", state.config.sqlite_options.duration);

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

    let session = state.session.lock().await;
    Ok(session.is_some())
}

#[tauri::command]
pub async fn cmd_logout(
    state: tauri::State<'_, ClientState>
) -> Result<(), Status> {

    let mut session_ref = state.session.lock().await;
    *session_ref = None;
    Ok(())
}

#[tauri::command]
pub async fn cmd_session_by_password(
    state: tauri::State<'_, ClientState>,
    data: PasswordDataClientShort
) -> Result<AuthStep, Status> {
    restore_by_password(&state, &data).await
}

#[tauri::command]
pub async fn cmd_session_by_nick(
    state: tauri::State<'_, ClientState>,
    nick: String1_50
) -> Result<AuthStep, Status> {
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
    if res.is_err() {
        log::debug!("cmd_register_user failed");
    }

    res
}
use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    PasswordDataShort, 
    AuthStep
};

use shared_lib::primitives::frozen::implements::{Inn, Kpp};
use shared_lib::service::auth_service::client_state::NickData;

use crate::state::ClientState;
use crate::service::auth_service::restore_by_nick::restore_session_by_nick;
use crate::service::auth_service::restore_by_password::restore_by_password;
use crate::service::process::bank_statement::proceed::process_statement;
use crate::service::auth_service::nick_data::get_nicknames;

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
) -> Result<bool, Status> {

    let session_ref = state.session.lock().await;
    if let Some(ref session) = *session_ref {
        match session.local_db.acquire().await {
            Ok(_) => Ok(true),
            Err(err) => {
                log::error!(
                    "tech_err = {}, local_err = {}",
                    err, Status::SystemErr
                );
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async  fn is_state_active_fast(
    state: tauri::State<'_, ClientState>
) -> Result<bool, Status> {

    let session = state.session.lock().await;
    Ok(session.is_some())
}

#[tauri::command]
pub async fn logout(
    state: tauri::State<'_, ClientState>
) -> Result<(), Status> {

    let mut session_ref = state.session.lock().await;
    *session_ref = None;
    Ok(())
}

#[tauri::command]
pub async fn get_state(
    state: tauri::State<'_, ClientState>,
    data: PasswordDataShort
) -> Result<AuthStep, Status> {
    restore_by_password(&state, &data).await
}

#[tauri::command]
pub async fn cmd_auth_restore_nick(
    state: tauri::State<'_, ClientState>,
    nick: String
) -> Result<AuthStep, Status> {
    match restore_session_by_nick(&state, &nick).await {
        Ok(res) => Ok(res),
        Err(err) => Ok(AuthStep::TryLater { status: err })
    }
}

#[tauri::command]
pub fn cmd_get_nick_names(
    state: tauri::State<'_, ClientState>
) -> Result<NickData, Status> {
    get_nicknames(&state)
}
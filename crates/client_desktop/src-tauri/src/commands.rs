use shared_lib::Status;
use shared_lib::service::auth_service::implements::{AppState, AuthData};
use shared_lib::primitives::frozen::implements::{Inn, Kpp};

use crate::service::auth_service::login::login;
use crate::service::process::bank_statement::proceed::process_statement;

#[tauri::command]
pub async fn cmd_process_bank_statement(
    state: tauri::State<'_, AppState>,
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
pub(crate) async  fn is_state_active(
    state: tauri::State<'_, AppState>
) -> Result<bool, Status> {

    let session_ref = state.session.lock().await;
    if let Some(ref session) = *session_ref {
        match session.local_db.acquire().await {
            Ok(_) => Ok(true),
            Err(err) => {
                log::error!(
                    "tech_err = {}, local_err = {}",
                    err, Status::AuthClientCommandIsStateActiveDbErr
                );
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub(crate) async fn logout(
    state: tauri::State<'_, AppState>
) -> Result<(), Status> {

    let mut session_ref = state.session.lock().await;
    *session_ref = None;
    Ok(())
}

#[tauri::command]
pub(crate) async fn get_state(
    state: tauri::State<'_, AppState>,
    pers_inn: Inn, 
    password: String, 
    comp_inn: Inn, 
    kpp: Kpp) -> Result<Status, Status> {
        let auth_data = AuthData {pers_inn, password, comp_inn, kpp};

        login(&state, auth_data).await
}
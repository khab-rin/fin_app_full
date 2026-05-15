use shared_lib::service::auth_service::implements::*;
use shared_lib::Status;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::implements::{AuthData, SessionUserToken};

use crate::service::auth_service::helper::get_device_id;


pub async fn register_user(auth_data: AuthData, state: &AppState) -> Result<(), Status> {
    let AuthData { pers_inn, password, comp_inn, kpp } = auth_data;


    if password.len() < 8 { return Err(Status::AuthShartPassword) }

    let device_id = get_device_id()?;

    let payload = serde_json::json!({
        "user_inn": pers_inn,
        "password": password,
        "company_inn": comp_inn,
        "kpp": kpp,
        "device_id": device_id
    });

    let api_url = format!(
        "{}/{}", 
        state.api_url.trim_end_matches('/'), 
        ApiRoutes::Register.get_path().trim_start_matches('/'));
        
    let response = state
        .client
        .post(&api_url)
        .json(&payload)
        .send()
        .await
        .inspect_err(|err| {
            log::error!(
                "tech_err = {}, stat_err = {}",
                err, Status::AuthSendQuery
            )
        }).map_err(|_| Status::AuthSendQuery)?;
        
    
    if response.status().is_success() {
        let server_data: SessionUserToken  = response
            .json()
            .await
            .inspect_err(|err| {
                log::error!(
                    "tech_err = {}, stat_err = {}",
                    err, Status::AuthMapRegisterResponse
                )
            }).map_err(|_| Status::AuthMapRegisterResponse)?;
        
        
        
        let mut session_guard = state
            .session.lock().await;

        
        // *session_guard = Some(ActiveSession { 
        //     user: server_data.user, 
        //     local_db: Config::make_base_url(&company_inn), 
        //     token: server_data.token })
        
    } else {
        return Err(Status::Unknown);
    }

    

    Err(Status::Unknown)
}
 



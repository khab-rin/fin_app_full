use shared_lib::Status;
use shared_lib::service::auth_service::client_state::{ClientState, UserLogInfo};
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::implements::{
    AuthStep, 
    RegisterResponse, 
};

use crate::service::auth_service::helper::get_device_id;
use crate::config::init_session;

pub(crate) async fn restore_session_by_nik(
    state: &ClientState,
    nik: &String
) -> Result<RegisterResponse, Status> {

    let device_id = match get_device_id() {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN restore_session_by_nik FAILED BY FUN get_device_id, err = {:?}", err
            );
            return Err(err);
        }
    };

    let failed_result = RegisterResponse {
        device_id: device_id.clone(),
        step: AuthStep::TryLater {}
    };

    let entry = match keyring::Entry::new(&state.app_name, nik) {
        Ok(e) => e,
        Err(err) => {
            log::error!(
                "FUN restore_session_by_nik FAILED BY keyring::Entry::new, teck_err = {:?}, local_err = {:?}", 
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    let json_data = match entry.get_password() {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN restore_session_by_nik FAILED BY entry.get_password, teck_err = {:?}, local_err = {:?}", 
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    let user_log_data: UserLogInfo = match serde_json::from_str(&json_data) {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN restore_session_by_nik FAILED BY mapping UserLogInfo, teck_err = {:?}, local_err = {:?}", 
                err, Status::MappingError
            );
            return Ok(failed_result);
        }
    };

    let token = user_log_data.token;

    let payload = serde_json::json!({
        "token": token,
        "device_id": device_id,
    });

    let back_api_url = format!("{}/{}",
        state.api_url.trim_end_matches('/'),
        ApiRoutes::AuthRestoreToken.get_path().trim_start_matches('/')
    );

    let response = match state
        .client
        .post(&back_api_url)
        .json(&payload)
        .send()
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!(
                    "FUN restore_session_by_nik FAILED BY POST QUERY TO BACK API, teck_err = {:?}, local_err = {:?}",
                    err, Status::QueryPostRequestErr 
                );
                return Ok(failed_result);
            }
        };

    if !response.status().is_success() {
        log::error!(
            "FUN restore_session_by_nik FAILED BY POST QUERY TO BACK API, local_err = {:?}",
            Status::QueryPostRequestErr 
        );
        return Ok(failed_result);
    }

    let register_response: RegisterResponse = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN restore_session_by_nik FAILED BY POST QUERY TO BACK API, err = {:?}, local_err = {:?}",
                err, Status::MappingError
            );
            return Ok(failed_result);
        }
    };

    match register_response.step {
        AuthStep::SuccessFull {session_user_token} =>{
            match init_session(state, session_user_token.as_ref()).await {
                Ok(_) => return Ok(RegisterResponse { device_id, step: AuthStep::SuccessShort {} }),
                Err(_) => return Err(Status::SystemErr)
            }   
        },
        _ => {}
    }

    Ok(register_response)
}
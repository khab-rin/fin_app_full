use shared_lib::Status;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::implements::{
    AuthStep, 
    TokenDeviceData,
    TextInfo
};

use crate::service::auth_service::helper::{
    get_device_id,
    get_keyring_data
};
use crate::state::{init_session, ClientState};

pub(crate) async fn restore_session_by_nick(
    state: &ClientState,
    nick: &str
) -> Result<AuthStep, Status> {

    let device_id = match get_device_id() {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN restore_session_by_nick FAILED BY FUN get_device_id, err = {:?}", err
            );
            return Ok(AuthStep::TryLater {text: TextInfo::ClientApiSystemError});
        }
    };

    

    let user_log_data = match get_keyring_data(state, nick) {
        Ok(None) => return Ok(AuthStep::NeedPassword {text: TextInfo::NewUserInSystem}),
        Ok(Some(u)) => u,
        Err(err) => return Err(err)
    };


    let token = user_log_data.token;

    let token_device_data = TokenDeviceData {
        token, device_id
    };


    let back_api_url = format!("{}/{}",
        state.config.back_api_url().trim_end_matches('/'),
        ApiRoutes::AuthRestoreToken.get_path().trim_start_matches('/')
    );

    let response = match state
        .config
        .get_inst_client()
        .post(&back_api_url)
        .headers(state.config.back_api_header().clone())
        .json(&token_device_data)
        .send()
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!(
                    "FUN restore_session_by_nick FAILED BY POST QUERY TO BACK API, teck_err = {:?}, local_err = {:?}",
                    err, Status::QueryPostRequestErr 
                );
                return Ok(AuthStep::TryLater {text: TextInfo::ClientApiQueryError});
            }
        };

    if !response.status().is_success() {
        let back_err = response
            .json::<Status>()
            .await
            .unwrap_or(Status::Unknown);

        log::error!(
            "FUN restore_session_by_nick FAILED BY POST QUERY TO BACK API. Backend error code: {}, local_err = {:?}",
            back_err, Status::BackApiError 
        );
        return Ok(AuthStep::TryLater {text: TextInfo::BackApiError});
    }

    let auth_step: AuthStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN restore_session_by_nick FAILED BY POST QUERY TO BACK API, err = {:?}, local_err = {:?}",
                err, Status::MappingError
            );
            return Ok(AuthStep::TryLater {text: TextInfo::ClientApiSystemError});
        }
    };

    if let AuthStep::SuccessFull {session_user_token} = &auth_step {
        match init_session(state, session_user_token.as_ref()).await {
            Ok(_) => return Ok(AuthStep::SuccessShort {}),
            Err(_) => return Err(Status::SystemErr)
        }   
    }

    Ok(auth_step)
}
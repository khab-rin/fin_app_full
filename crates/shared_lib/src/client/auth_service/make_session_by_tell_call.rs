use crate::{Status, ClientState, ProcessError};
use crate::service::api_routes::implements::ApiRoutes;
use crate::service::auth_service::implements::{
    AuthStep, 
    ExternalDeviceData,
    AuthInfo
};

use crate::service::auth_service::client_state::init_session;
use crate::client::back_api::post_query::post_query_back_api;
use crate::client::auth_service::helper::{
    get_device_id,
    write_new_user_info_to_device
};

pub async fn make_session_by_tel_call(
    state: &ClientState,
    external_id: &str,
) -> Result<AuthStep, Status> {

    let failed_result = Ok(AuthStep::TryLater { text: AuthInfo::ClientApiSystemError });

    let device_id = match get_device_id() {
        Ok(d) => d,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    let external_device_data = ExternalDeviceData {
        external_id: external_id.to_string(),
        device_id
    };

    let response = match post_query_back_api(
            state, 
            state.config.get_std_client(), 
            ApiRoutes::AuthRestoreTellCall, 
            &external_device_data).await {
        Ok(r) => r,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    let auth_step: AuthStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            err.process_err(Status::MappingError, "");
            return failed_result;
        }
    };

    let session_token = match auth_step {
        AuthStep::SuccessFull { session_user_token } => session_user_token,
        _ => return Ok(auth_step)
    };

    match write_new_user_info_to_device(state, &session_token) {
        Ok(_) => {},
        Err(err) => {
            err.process_err(Status::MappingError, "");
            return failed_result;
        }
    }

    match init_session(state, session_token.as_ref()).await {
        Ok(_) => Ok(AuthStep::SuccessShort {  }),
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    }
}
use shared_lib::Status;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::implements::{
    AuthStep, 
    ExternalDeviceData,
    AuthInfo
};

use crate::state::{ClientState, init_session};
use crate::back_api::post_query::post_query_back_api;
use crate::service::auth_service::helper::get_device_id;
use crate::service::auth_service::helper::write_new_user_info_to_device;

pub(crate) async fn make_session_by_tel_call(
    state: &ClientState,
    external_id: &str,
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: AuthInfo::ClientApiSystemError };

    let device_id = match get_device_id() {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN make_session_by_tel_call FAILED BY FUN get_device_id, err = {:?}", err
            );
            return Ok(AuthStep::TryLater {text: AuthInfo::ClientApiSystemError});
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
            log::error!("FUN make_session_by_tel_call FAILED BY FUN post_query_back_api, err = {}", err);
            return Ok(failed_result)
        }
    };

    let auth_step: AuthStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN restore_by_password FAILED BY POST QUERY TO BACK API, err = {:?}, local_err = {:?}",
                err, Status::MappingError
            );
            return Ok(AuthStep::TryLater {text: AuthInfo::ClientApiSystemError});
        }
    };

    let session_token = match auth_step {
        AuthStep::SuccessFull { session_user_token } => session_user_token,
        _ => return Ok(auth_step)
    };

    match write_new_user_info_to_device(state, &session_token) {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "FUN register_step2 FAILED BY MAPPING RESPONSE, err = {:?}, local_err = {:?}",
                err, Status::MappingError
            );
            return Ok(failed_result);
        }
    }

    match init_session(state, session_token.as_ref()).await {
        Ok(_) => Ok(AuthStep::SuccessShort {  }),
        Err(err) => {
            log::error!("FUN register_user FAILED BY init_session, err = {}",err);
            Ok(failed_result)
        }
    }
}
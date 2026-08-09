use crate::{Status, ClientState, ProcessError};
use crate::service::api_routes::implements::ApiRoutes;
use crate::service::auth_service::implements::{
    AuthStep, 
    TokenDeviceData,
    AuthInfo
};
use crate::client::back_api::post_query::post_query_back_api;
use crate::client::auth_service::helper::get_device_id;
use crate::client::auth_service::key_ring::get_keyring_token;
use crate::client::auth_service::nick_data::get_nick_data_by_nick;
use crate::service::auth_service::client_state::init_session;

pub async fn restore_session_by_nick(
    state: &ClientState,
    nick: &String
) -> Result<AuthStep, Status> {

    let failed_result = Ok(AuthStep::TryLater {text: AuthInfo::ClientApiSystemError});

    let device_id = match get_device_id() {
        Ok(d) => d,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    let nick_data_option = match get_nick_data_by_nick(state, nick) {
        Ok(o) => o,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    let nick_data = match nick_data_option {
        Some(d) => d,
        None => return Ok(AuthStep::Password { text: AuthInfo::MissToken })
    };

    let key_ = format!("{}{}{}", nick_data.pers_inn, nick_data.comp_inn, nick_data.kpp);

    let token_option = match get_keyring_token(state, &key_) {
        Ok(u) => u,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    let token = match token_option {
        Some(t) => t,
        None =>  {
            return Ok(AuthStep::Password { text: AuthInfo::MissToken });
        }
    };

    let token_device_data = TokenDeviceData {
        token, device_id
    };

    let response = match post_query_back_api(
            state, 
            state.config.get_std_client(), 
            ApiRoutes::AuthRestoreToken, 
            &token_device_data).await {
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

    if let AuthStep::SuccessFull {session_user_token} = &auth_step {
        match init_session(state, session_user_token.as_ref()).await {
            Ok(_) => return Ok(AuthStep::SuccessShort {}),
            Err(err) => {
                err.process_err(err, "");
                return failed_result;
            }
        }
    }

    Ok(auth_step)
}
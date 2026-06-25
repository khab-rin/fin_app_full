use shared_lib::Status;
use shared_lib::primitives::frozen::implements_base::String1_50;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::implements::{
    AuthStep, 
    TokenDeviceData,
    TextInfo
};

use crate::back_api::post_query::post_query_back_api;
use crate::service::auth_service::helper::get_device_id;
use crate::service::auth_service::key_ring::get_keyring_data;
use crate::service::auth_service::nick_data::get_nick_data_by_nick;
use crate::state::{init_session, ClientState};

pub(crate) async fn restore_session_by_nick(
    state: &ClientState,
    nick: &String1_50
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

    let nick_data_option = match get_nick_data_by_nick(state, nick) {
        Ok(o) => o,
        Err(err) => {
            log::error!(
                "FUN restore_session_by_nick FAILED BY FUN get_nick_data_by_nick, local_err = {}", err
            );
            return Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError });
        }
    };

    let nick_data = match nick_data_option {
        Some(d) => d,
        None => return Ok(AuthStep::NeedPassword { text: TextInfo::MissToken })
    };

    let key_ = format!("{}{}{}", nick_data.pers_inn, nick_data.comp_inn, nick_data.kpp);

    let user_log_data = match get_keyring_data(state, &key_) {
        Ok(u) => u,
        Err(err) => {
            log::error!(
                "FUN restore_session_by_nick FAILED BY FUN get_keyring_data, local_err = {}", err
            );
            return Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError });
        }
    };

    let token = match user_log_data.token {
        Some(t) => t,
        None =>  {
            return Ok(AuthStep::NeedPassword { text: TextInfo::MissToken });
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
            log::error!(
                "FUN restore_session_by_nick FAILED BY FUN post_query_back_api, local_err = {}", err
            );
            return Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError });
        }
    };

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
            Err(err) => {
                log::error!(
                    "FUN restore_session_by_nick FAILED BY FUN init_session, local_err = {}", err
                );
                return Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError });
            }
        }
    }

    Ok(auth_step)
}
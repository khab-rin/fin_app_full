use shared_lib::Status;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::implements::{
    AuthStep, 
    ExternalDeviceData,
    TextInfo
};
use shared_lib::service::auth_service::client_state::{NickData, UserLogInfo};
use shared_lib::primitives::frozen::implements_base::String1_50;

use crate::state::{ClientState, init_session};
use crate::back_api::post_query::post_query_back_api;
use crate::service::auth_service::helper::get_device_id;
use crate::service::auth_service::key_ring::write_keyring_data;
use crate::service::auth_service::nick_data::add_nick_data;

pub(crate) async fn make_session_by_tel_call(
    state: &ClientState,
    external_id: &str,
    nick: &String1_50
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: TextInfo::ClientApiSystemError };

    let device_id = match get_device_id() {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN make_session_by_tel_call FAILED BY FUN get_device_id, err = {:?}", err
            );
            return Ok(AuthStep::TryLater {text: TextInfo::ClientApiSystemError});
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
            return Ok(AuthStep::TryLater {text: TextInfo::ClientApiSystemError});
        }
    };

    let success_result = match auth_step {
        AuthStep::SuccessFull { session_user_token } => session_user_token,
        _ => return Ok(auth_step)
    };


    let log_info = UserLogInfo {
        init_file_hash: "".to_string(),
        token: Some(success_result.token.clone())
    };

    let sur_name = success_result.user.person.metadata.fio.sur_name.clone();


    let comp_name = success_result.user.company.metadata.comp_name.as_ref() // <-- добавили .as_ref() для внешнего Option
        .and_then(|c| c.short_egrul_name.as_ref())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Неизвестная компания".to_string());
    
    let nick = format!("{}_{}", sur_name, comp_name);


    let nick_data = NickData {
        nick,
        pers_inn: success_result.user.person.pers_inn.clone(),
        comp_inn: success_result.user.company.comp_inn.clone(),
        kpp: success_result.user.company.kpp.clone()
    };

    let key_ = format!("{}{}{}", nick_data.pers_inn, nick_data.comp_inn, nick_data.kpp);


    match write_keyring_data(state, &key_, &log_info) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN make_session_token_by_tel_call FAILED by writing UserLogInfo, err = {}", err);
        }
    }

    match add_nick_data(state, &nick_data) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN make_session_token_by_tel_call FAILED BY FUN add_nick_data, err = {}", err);
            return Ok(failed_result);
        }
    }

    match init_session(state, &success_result).await {
        Ok(_) => Ok(AuthStep::SuccessShort {}),
        Err(err) => {
            log::error!("FUN restore_by_password FAILED BY init_session, err = {}",err);
            Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError })
        }
    }
}
use shared_lib::Status;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::implements::{
    PasswordDataClient,
    PasswordDataClientShort,
    AuthStep,
    TextInfo
};
use shared_lib::service::auth_service::client_state::NickData;

use crate::state::{ClientState, init_session};
use crate::back_api::post_query::post_query_back_api;
use crate::service::auth_service::helper::get_device_id;
use crate::service::auth_service::nick_data::add_nick_data;
use crate::service::auth_service::key_ring::{write_keyring_data, get_keyring_data};


pub(crate) async fn restore_by_password(
    state: &ClientState,
    data: &PasswordDataClientShort
) -> Result<AuthStep, Status> {

    let key_ = format!("{}{}{}", data.pers_inn, data.comp_inn, data.kpp);

    let mut user_log_info = match get_keyring_data(state, &key_) {
        Ok(i) => i,
        Err(err) => {
            log::error!("FUN restore_by_password FAILED BY FUN get_keyring_data, err = {}", err);
            return Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError });
        }
    };

    let device_id = match get_device_id() {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN restore_by_password FAILED BY FUN get_device_id, err = {:?}", err
            );
            return Err(err);
        }
    };

    let password_hash = blake3::hash(data.password.clone().as_bytes()).to_hex().to_string();

    let password_data = PasswordDataClient {
        password: password_hash,
        device_id,
        pers_inn: data.pers_inn.clone(),
        comp_inn: data.comp_inn.clone(),
        kpp: data.kpp.clone()
    };

    let response = match post_query_back_api(
            state, 
            state.config.get_std_client(), 
            ApiRoutes::AuthRestorePassword, 
            &password_data).await {
        Ok(r) => r,
        Err(err) => {
            log::error!("FUN restore_by_password FAILED BY FUN post_query_back_api, err = {}", err);
            return Ok(AuthStep::TryLater { text: TextInfo::BackApiError});
        }
    };


    let auth_step:AuthStep = match response.json().await {
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
        AuthStep::SuccessFull {session_user_token} => session_user_token,
        _ => return Ok(auth_step)
    };

    user_log_info.token = Some(success_result.token.clone());

    match write_keyring_data(state, &key_, &user_log_info) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN restore_by_password FAILED by writing UserLogInfo, err = {}", err);
            return Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError });
        }
    }

    let sur_name = success_result.user.person.metadata.fio.sur_name.clone();


    let comp_name = success_result.user.company.metadata.comp_name.as_ref() // <-- добавили .as_ref() для внешнего Option
        .and_then(|c| c.short_egrul_name.as_ref())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Неизвестная компания".to_string());
    
    let nick = format!("{}_{}", sur_name, comp_name);

    let nick_data = NickData {
        nick,
        pers_inn: data.pers_inn.clone(),
        comp_inn: data.comp_inn.clone(),
        kpp: data.kpp.clone()
    };

    match add_nick_data(state, &nick_data) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN restore_by_password FAILED BY FUN add_nick_data, err = {}", err);
            return Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError });
        }
    }

    match init_session(state, success_result.as_ref()).await {
        Ok(_) => Ok(AuthStep::SuccessShort {}),
        Err(err) => {
            log::error!("FUN restore_by_password FAILED BY init_session, err = {}",err);
            Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError })
        }
    }

    
}
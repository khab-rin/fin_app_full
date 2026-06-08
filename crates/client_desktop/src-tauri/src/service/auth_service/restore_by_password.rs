use shared_lib::Status;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::implements::{
    PasswordData,
    PasswordDataShort,
    AuthStep,
    TextInfo
};
use shared_lib::service::auth_service::client_state::UserLogInfo;


use crate::state::{ClientState, init_session};
use crate::service::auth_service::helper::{get_device_id, write_log_info};


pub(crate) async fn restore_by_password(
    state: &ClientState,
    data: &PasswordDataShort
) -> Result<AuthStep, Status> {

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

    let password_data = PasswordData {
        password: password_hash,
        device_id,
        pers_inn: data.pers_inn.clone(),
        comp_inn: data.comp_inn.clone(),
        kpp: data.kpp.clone()
    };

    let back_api_url = format!("{}/{}",
        state.config.back_api_url().trim_end_matches('/'),
        ApiRoutes::AuthRestorePassword.get_path().trim_start_matches('/')
    );

    let response = match state
        .config
        .get_inst_client()
        .post(&back_api_url)
        .headers(state.config.back_api_header().clone())
        .json(&password_data)
        .send()
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!(
                    "FUN restore_by_password FAILED BY POST QUERY TO BACK API, teck_err = {:?}, local_err = {:?}, url = {:?}",
                    err, Status::QueryPostRequestErr, back_api_url
                );
                return Ok(AuthStep::TryLater { text: TextInfo::ClientApiQueryError});
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
        return Ok(AuthStep::TryLater { text: TextInfo::BackApiError});
    }

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

    let log_info = UserLogInfo {
        pers_inn: data.pers_inn.clone(),
        comp_inn: data.comp_inn.clone(),
        kpp: data.kpp.clone(),
        token: success_result.token.clone()
    };

    match write_log_info(state, &data.nick, &log_info) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN restore_by_password FAILED by writing UserLogInfo, err = {}", err);
        }
    }

    match init_session(state, success_result.as_ref()).await {
        Ok(_) => Ok(AuthStep::SuccessShort {}),
        Err(err) => {
            log::error!("FUN restore_by_password FAILED BY init_session, err = {}",err);
            Err(err)
        }
    }

    
}
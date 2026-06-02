use shared_lib::Status;
use shared_lib::service::api_routes::implements::{ApiRoutes};
use shared_lib::service::auth_service::implements::{
    AuthStep, 
    PhoneDeviceData,
    TextInfo
};
use shared_lib::service::auth_service::client_state::UserLogInfo;

use crate::state::{ClientState, init_session};
use crate::service::auth_service::helper::write_log_info;

pub(crate) async fn make_session_token_by_tel_call(
    state: &ClientState,
    data: &PhoneDeviceData,
    nick: &str
) -> Result<AuthStep, Status> {
    
    let back_api_url = format!("{}/{}",
        state.config.back_api_url().trim_end_matches('/'),
        ApiRoutes::AuthMakeTokenTelCall.get_path().trim_start_matches('/')
    );

    let response = match state
        .config
        .get_std_client()
        .post(&back_api_url)
        .headers(state.config.back_api_header().clone())
        .json(data)
        .send()
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!(
                    "FUN make_session_token_by_tel_call FAILED BY POST QUERY TO BACK API, teck_err = {:?}, local_err = {:?}",
                    err, Status::QueryPostRequestErr 
                );
                return Ok(AuthStep::TryLater {text: TextInfo::ClientApiQueryError});
            }
        };
    
    if !response.status().is_success() {
        let back_err = response
            .json()
            .await
            .unwrap_or(Status::Unknown);
        log::error!(
            "FUN make_session_token_by_tel_call FAILED BY POST QUERY TO BACK API. Backend error code: {}, local_err = {:?}",
            back_err, Status::BackApiError
        );
        return Ok(AuthStep::TryLater {text: TextInfo::BackApiError});
    }

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
        pers_inn: success_result.user.person.pers_inn.clone(),
        comp_inn: success_result.user.company.comp_inn.clone(),
        kpp: success_result.user.company.kpp.clone(),
        token: success_result.token.clone()
    };

    match write_log_info(state, nick, &log_info) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN restore_by_password FAILED by writing UserLogInfo, err = {}", err);
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
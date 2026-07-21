use shared_lib::Status;

use shared_lib::service::auth_service::implements::{
    AuthStep, AuthInfo, RegFilesPathData
};
use shared_lib::service::crypto_service::implements::CheckSignDocData;
use shared_lib::service::api_routes::implements::ApiRoutes;


use crate::state::ClientState;
use crate::state::init_session;
use crate::back_api::post_query::post_query_back_api;
use crate::service::auth_service::helper::write_new_user_info_to_device;


pub(crate) async fn register_step2(
    state: &ClientState,
    data: &RegFilesPathData
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: AuthInfo::ClientApiSystemError };

    let RegFilesPathData { json_path, sign_path } = data;

    let init_file = match std::fs::read(json_path) {
        Ok(f) => f,
        Err(err) => {
            log::error!(
                "FUN register_step2 FAILED BY std::fs::read(json_path), tech_err = {:?}, local_err = {:?}",
                err, Status::FileReadError
            );
            return Ok(failed_result);
        }
    };

    let sign_file = match std::fs::read(sign_path) {
        Ok(f) => f,
        Err(err) => {
            log::error!(
                "FUN register_step2 FAILED BY std::fs::read(sign_path), tech_err = {:?}, local_err = {:?}",
                err, Status::FileReadError
            );
            return Ok(failed_result);
        }
    };

    let reg_files_data = CheckSignDocData {
        init_file, sign_file
    };


    let response = match post_query_back_api(
            state,
            state.config.get_std_client(),
            ApiRoutes::AuthRegisterStep2,
            &reg_files_data
    ).await {
        Ok(r) => r,
        Err(err) => {
            log::error!(
                "FUN register_step2 FAILED BY POST QUERY TO BACK API, local_err = {:?}", err
            );
            return Ok(failed_result);
        }
    };

    let auth_step: AuthStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN register_step2 FAILED BY MAPPING RESPONSE, err = {:?}, local_err = {:?}",
                err, Status::MappingError
            );
            return Ok(failed_result);
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
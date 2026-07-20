use std::fmt::format;

use shared_lib::Status;

use shared_lib::service::auth_service::implements::{
    AuthStep, AuthInfo, RegFilesPathData, RegFilesData
};
use shared_lib::service::auth_service::client_state::NickData;
use shared_lib::service::api_routes::implements::ApiRoutes;


use crate::state::ClientState;
use crate::state::init_session;
use crate::back_api::post_query::post_query_back_api;
use crate::service::auth_service::key_ring::write_keyring_token;
use crate::service::auth_service::nick_data::add_nick_data;


pub(crate) async fn register_step2(
    state: &ClientState,
    data: &RegFilesPathData
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: AuthInfo::ClientApiSystemError };

    let RegFilesPathData { json_path, sign_path } = data;

    let json_file = match std::fs::read(json_path) {
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

    let reg_files_data = RegFilesData {
        json_file, sign_file
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

    let pers_inn = &session_token.user.person.pers_inn;
    let comp_inn = &session_token.user.company.comp_inn;
    let kpp = &session_token.user.company.kpp;

    let sur_name = &session_token.user.person.metadata.fio.sur_name;
    let comp_name = session_token.user.company.metadata.comp_name.as_ref()
        .and_then(|c| c.short_egrul_name.as_ref())
        .map(|s| s.to_string())
        .unwrap_or("Неизвестная компания".to_string());

    
    let token = session_token.token.clone();

    let nick = format!("{}_{}", sur_name, comp_name);
    let key_ = format!("{}{}{}", pers_inn, comp_inn, kpp);

    let nick_data = NickData {
        nick,
        pers_inn: pers_inn.clone(),
        comp_inn: comp_inn.clone(),
        kpp: kpp.clone()
    };

    match add_nick_data(state, &nick_data) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN register_step2 FAILED BY FUN add_nick_data, err = {}", err);
            return Ok(failed_result);
        }
    }

    match write_keyring_token(state, &key_, &token) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN register_step2 FAILED BY FUN write_keyring_data, err = {}", err);
            return Ok(failed_result);
        }
    };

    match init_session(state, session_token.as_ref()).await {
        Ok(_) => Ok(AuthStep::SuccessShort {  }),
        Err(err) => {
            log::error!("FUN register_user FAILED BY init_session, err = {}",err);
            Ok(failed_result)
        }
    }


}
use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, DateTime};
use shared_lib::service::auth_service::implements::{
    AuthStep, 
    RegistrationData, 
    SvelteRegistrationData,
    TextInfo
};
use shared_lib::service::auth_service::client_state::UserLogInfo;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::sql_models::person::implements::{Person, PersonMetadata};
use shared_lib::primitives::composite::implements::Fio;
use sqlx::types::chrono;

use crate::state::{ClientState, init_session};
use crate::service::auth_service::helper::{get_device_id, write_log_info};



pub async fn register_user(
    state: &ClientState,
    data: SvelteRegistrationData
) -> Result<AuthStep , Status> {

    log::debug!("register_user running");

    let SvelteRegistrationData { 
        nick, 
        sur_name, 
        first_name, 
        mid_name, 
        pers_inn, 
        snils, 
        comp_inn, 
        kpp, 
        password, 
        phone, 
        email, 
        document_path, 
        signature_path 
    } = data;

    let mut quard = state.temp_info.lock().await;
    quard.nick = Some(nick.clone());

    let fio = Fio { sur_name, first_name, mid_name: Some(mid_name) };
    let metadata = PersonMetadata {
        fio, 
        snils, 
        phone: Some(phone.clone()), 
        email: Some(email.clone()), 
        passport: None, 
        address: None, 
        gender: None, 
        birth_day: None
    };


    let person = Person {
        pers_id: BoxUuid::unchecked(uuid::Uuid::new_v4()),
        pers_inn,
        metadata,
        last_update: DateTime::unchecked(chrono::Utc::now())
    };


    let doc_hash = match quard.file_hash.clone() {
        Some(t) => t,
        None => {
            log::error!("DOC HASH MISSED");
            return Ok(AuthStep::TryLater { text: TextInfo::ClientApiSystemError });
        }
    };

    let device_id = match get_device_id() {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN register_user FAILED BY FUN get_device_id, err = {:?}", err
            );
            return Ok(AuthStep::TryLater {text: TextInfo::ClientApiSystemError});
        }
    };

    let document = match std::fs::read(document_path) {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN register_user FAILED BY FILE READ, tech_err = {}, local_err = {}",
                err, Status::FileReadError
            );
            return Ok(AuthStep::TryLater {text: TextInfo::ClientApiSystemError});
        }
    };

    let signature = match std::fs::read(signature_path) {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN register_user FAILED BY FILE READ, tech_err = {}, local_err = {}",
                err, Status::FileReadError
            );
            return Ok(AuthStep::TryLater {text: TextInfo::ClientApiSystemError});
        }
    };


    let password_hash = blake3::hash(password.as_bytes()).to_hex().to_string();

    let registration_data = RegistrationData {
        person: person.clone(),
        comp_inn: comp_inn.clone(),
        kpp: kpp.clone(),
        password: password_hash,
        device_id,
        phone,
        email,
        doc_hash,
        document,  
        signature,
    };

    let back_api_url = format!("{}/{}",
        state.config.back_api_url().trim_end_matches('/'),
        ApiRoutes::AuthRegister.get_path().trim_start_matches('/'));

    let response = match state
        .config
        .get_std_client()
        .post(&back_api_url)
        .headers(state.config.back_api_header().clone())
        .json(&registration_data)
        .send()
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!(
                    "FUN register_user FAILED BY POST QUERY TO BACK API, teck_err = {:?}, local_err = {:?}",
                    err, Status::QueryPostRequestErr 
                );
                return Ok(AuthStep::TryLater {text: TextInfo::ClientApiQueryError});
            }
        };
    
    
    
    if !response.status().is_success() {
        let back_err = response
            .json::<Status>()
            .await
            .unwrap_or(Status::Unknown);
        log::error!(
            "FUN register_user FAILED, BACK API GIVE WRONG STATUS. Backend error code: {}, local_err = {:?}",
            back_err, Status::BackApiError
        );
        return Ok(AuthStep::TryLater {text: TextInfo::BackApiError});
    }

    let auth_step:AuthStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN register_user FAILED BY MAPPING RESPONSE, err = {:?}, local_err = {:?}",
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
        pers_inn: person.pers_inn,
        comp_inn,
        kpp,
        token: success_result.token.clone()
    };

    match write_log_info(state, &nick, &log_info) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN register_user FAILED by writing UserLogInfo, err = {}", err);
        }
    }

    match init_session(state, success_result.as_ref()).await {
        Ok(_) => Ok(AuthStep::SuccessShort {}),
        Err(err) => {
            log::error!("FUN register_user FAILED BY init_session, err = {}",err);
            Ok(AuthStep::TryLater {text: TextInfo::ClientApiSystemError})
        }
    }

}
 



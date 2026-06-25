use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, DateTime};
use shared_lib::service::auth_service::implements::{
    AuthStep, 
    RegistrationData, 
    SvelteRegistrationData,
    TextInfo
};
use shared_lib::service::auth_service::client_state::NickData;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::sql_models::person::implements::{Person, PersonMetadata};
use shared_lib::primitives::composite::implements::Fio;
use sqlx::types::chrono;

use crate::state::{ClientState, init_session};
use crate::back_api::post_query::post_query_back_api;
use crate::service::auth_service::helper::get_device_id;
use crate::service::auth_service::nick_data::add_nick_data;
use crate::service::auth_service::key_ring::{get_keyring_data, write_keyring_data};


pub async fn register_user(
    state: &ClientState,
    data: &SvelteRegistrationData
) -> Result<AuthStep , Status> {

    log::debug!("register_user running");

    let failed_result = AuthStep::TryLater { text: TextInfo::ClientApiSystemError };

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

    let fio = Fio { 
        sur_name: sur_name.clone(), 
        first_name: first_name.clone(), 
        mid_name: Some(mid_name.clone()) 
    };

    let metadata = PersonMetadata {
        fio, 
        snils: snils.clone(), 
        phone: Some(phone.clone()), 
        email: Some(email.clone()), 
        passport: None, 
        address: None, 
        gender: None, 
        birth_day: None
    };

    let person = Person {
        pers_id: BoxUuid::unchecked(uuid::Uuid::new_v4()),
        pers_inn: pers_inn.clone(),
        metadata,
        last_update: DateTime::unchecked(chrono::Utc::now())
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

    let key_ = format!("{}{}{}", pers_inn, comp_inn, kpp);

    let mut user_log_info = match get_keyring_data(state, &key_) {
        Ok(i) => i,
        Err(err) => {
            log::error!(
                "FUN register_user FAILED BY FUN get_keyring_data, err = {}", err
            );
            return Ok(failed_result);
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
        phone: phone.clone(),
        email: email.clone(),
        doc_hash: user_log_info.init_file_hash.clone(),
        document,  
        signature,
    };

    let response = match post_query_back_api(
            state, 
            state.config.get_std_client(), 
            ApiRoutes::AuthRegister, 
            &registration_data).await {
        Ok(r) => r, 
        Err(err) => {
            log::error!(
                "FUN register_user FAILED BY POST QUERY TO BACK API, local_err = {:?}", err
            );
            return Ok(AuthStep::TryLater {text: TextInfo::ClientApiQueryError});
        }
    };

    let auth_step:AuthStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN register_user FAILED BY MAPPING RESPONSE, err = {:?}, local_err = {:?}",
                err, Status::MappingError
            );
            return Ok(failed_result);
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
            log::error!("FUN register_user FAILED by writing UserLogInfo, err = {}", err);
            return Ok(failed_result);
        }
    }

    let nick_data = NickData {
        nick: nick.clone(),
        pers_inn: pers_inn.clone(),
        comp_inn: comp_inn.clone(),
        kpp: kpp.clone()
    };

    match add_nick_data(state, &nick_data) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN register_user FAILED BY FUN add_nick_data, err = {}", err);
            return Ok(failed_result);
        }
    }


    match init_session(state, success_result.as_ref()).await {
        Ok(_) => Ok(AuthStep::SuccessShort {}),
        Err(err) => {
            log::error!("FUN register_user FAILED BY init_session, err = {}",err);
            Ok(failed_result)
        }
    }

}
 



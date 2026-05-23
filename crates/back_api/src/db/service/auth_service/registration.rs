use std::sync::Arc;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2
};

use shared_lib::service::{api_routes::implements::CryptoApiRoutes, auth_service::client_state::SessionUser};
use shared_lib::sql_models::user::implements::UserSetData;
use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    SessionUserToken,
    CryptoVerifyRequest, 
    RegisterResponse, 
    RegistrationRequest, 
    VerifyData, 
    VerifyMethod,
    CryptoVerifyPersonResponse
};

use crate::config::BackApiState;
use crate::db::service::auth_service::validate_fields::validate_field;
use crate::db::sql_queries::users::get::exists_user_by_pers_comp::exists_user_by_pers_comp;
use crate::db::sql_queries::persons::add::by_person::add_person;
use crate::db::sql_queries::companys::get::company_by_inn_kpp::get_company_by_inn_kpp;
use crate::db::sql_queries::companys::helper::make_new_company;
use crate::db::sql_queries::users::set::user::set_user;
use crate::db::sql_queries::sessions::set::new_session::new_session;



pub(crate) async fn register_new_user(
    state: &Arc<BackApiState>,
    payload: RegistrationRequest
) -> Result<RegisterResponse, Status> {

    let RegistrationRequest { 
        person, 
        comp_inn,
        kpp, 
        password, 
        device_id, 
        phone,
        email,
        doc_hash, 
        document, 
        signature } = payload;
    
    let failed_result = RegisterResponse::Verify(VerifyData { 
        device_id: device_id.clone(), 
        method: VerifyMethod::TryLater {} });
    
    let failed_data = (
        &person, 
        &comp_inn, 
        &kpp, 
        &device_id);

    let check_hash = blake3::hash(&document).to_hex();

    if !check_hash.as_str().eq_ignore_ascii_case(&doc_hash) {
        tracing::warn!(
            failed_data = ?failed_data,
            "PERSON LEND ANOTHER FILE"
        );
        return Ok(RegisterResponse::Verify(VerifyData {
            device_id,
            method: VerifyMethod::MissedFile {}}));
    }

    let crypto_verify_request = CryptoVerifyRequest {
        document,
        signature
    };

    let crypto_url = format!(
        "{}/{}",
        state.config.crypto_servise.url.trim_end_matches('/'),
        CryptoApiRoutes::CryptoVerifyPerson.get_path().trim_start_matches('/')
    );

    let response = match state.config.get_inst_client()
        .post(&crypto_url)
        .json(&crypto_verify_request)
        .send()
        .await {
            Ok(r) => r,
            Err(err) => {
                tracing::error!(
                    err = ?err,
                    local_err = ?Status::QueryGetRequestErr,
                    failed_data = ?failed_data,
                    "FUN register_new_user FAILED BY REQUEST TO CRYPTO SERVICE"
                );
                return Ok(failed_result);
            }
        };

    
    if !response.status().is_success() {
        tracing::error!(
            failed_data = ?failed_data,
            "FUN register_new_user FAILED BY REQUEST TO CRYPTO SERVICE - CONNECTION PROBLEMS"
        );
        return Ok(failed_result);
    }

    let verify_person: CryptoVerifyPersonResponse = match response
        .json()
        .await  {
        Ok(r) => r,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::CryptoVerifyPersonResponseMappingErr,
                failed_data = ?failed_data,
                "FUN register_new_user FAILED BY MAPPING CryptoVerifyPersonResponse"
            );
            return Ok(failed_result);
        }
    };

    if !verify_person.is_signed {
        tracing::warn!(
            failed_data = ?failed_data,
            "FUN register_new_user FAILED BY WRONG SIGNATURE FILE"
        );
        return Ok(RegisterResponse::Verify(VerifyData { 
            device_id, 
            method: VerifyMethod::WrongSignFile { }}));
    }

    if let Err(res) = validate_field(
        "FIO",
        verify_person.fio,
        &person.metadata.fio,
        &device_id,
        &failed_data
    ) { return Ok(res);};
    
    if let Err(res) =  validate_field(
        "INN",
        verify_person.inn,
        &person.inn,
        &device_id,
        &failed_data
    ) { return Ok(res);};

    if let Err(res) = validate_field(
        "SNILS",
        verify_person.snils,
        &person.metadata.snils,
        &device_id,
        &failed_data
    ) { return Ok(res);};


    let person = match add_person(state, &person).await {
        Ok(p) => p,
        Err(err) => {
            tracing::error!(
                err = ?err,
                "FUN register_new_user FAILED BY ADD PERSON SQL QUERY"
            );
            return Ok(failed_result);
        }
    };

    let comp_opt = match get_company_by_inn_kpp(state, &comp_inn, &kpp).await {
        Ok(c_o) => c_o,
        Err(err) => {
            tracing::error!(
                err = ?err,
                failed_data = ?failed_data,
                "FUN get_company_by_inn_kpp FAILED IN FUN get_company_by_inn_kpp"
            );
            None
        }
    };

    let company = match comp_opt {
        Some(c) => c,
        None => {
            match make_new_company(state, &comp_inn, &kpp).await {
                Ok(c) => c,
                Err(err) => {
                    tracing::error!(
                        err = ?err,
                        failed_data = ?failed_data,
                        "FUN register_new_user FAILED BY FUN make_new_company"
                    );
                    return Ok(failed_result);
                }
            }
        }
    };

    let exist_flag = match exists_user_by_pers_comp(
        state, 
        &person.pers_id, 
        &company.comp_id).await {
            Ok(f) => f,
            Err(err) => {
                tracing::error!(
                    err =?err,
                    failed_data = ?failed_data,
                    "FUN get_session_user_by_company FAILED IN FUN register_new_user"
                );
                false
            }
        };

    if exist_flag {
        return Ok(RegisterResponse::Verify(VerifyData { 
            device_id, 
            method: VerifyMethod::UserAlreadyExists { } }));
    }

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let server_password_hash = match argon2
        .hash_password(password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(err) => {
            tracing::error!(
                err = ?err,
                failed_data = ?failed_data,
                "FUN register_new_user FAILED BY ARGON2 HASHING PASSWORD"
            );
            return Ok(failed_result);
        }
    };

    let user_set_data = UserSetData {
        pers_id: person.pers_id.clone(),
        comp_id: company.comp_id.clone(),
        phone,
        password_hash: server_password_hash,
        email,
        mchd_tax_guid: None,
        mchd_tax_file: None,
        mchd_home_guid: None,
        mchd_home_file: None,
    };

    let user = match set_user(state, &user_set_data).await {
        Ok(u) => u,
        Err(err) => {
            tracing::error!(
                err = ?err,
                failed_data = ?failed_data,
                "FUN register_new_user FAILED BY FUN set_user"
            );
            return Ok(failed_result);
        }
    };

    let token = match new_session(state, &user.user_id, &device_id).await {
        Ok(t) => t,
        Err(err) => {
            tracing::error!(
                err = ?err,
                failed_data = ?failed_data,
                "FUN register_new_user FAILED BY new_session FUN"
            );
            return Ok(failed_result);
        }
    };

    let session_user = SessionUser {
        user,
        person,
        company
    };

    let session_user_token = SessionUserToken {
        user: session_user,
        token
    };

    let ok_result = RegisterResponse::Success(Box::new(session_user_token));

    Ok(ok_result) 
}
use std::sync::Arc;

use shared_lib::service::api_routes::implements::CryptoApiRoutes;
use shared_lib::sql_models::person_models::implements::Person;
use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    CryptoVerifyRequest, 
    RegisterResponse, 
    RegistrationRequest, 
    VerifyData, 
    VerifyMethod,
    CryptoVerifyPersonResponse
};

use crate::config::BackApiState;
use crate::db::service::auth_service::validate_fields::validate_field;

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


    Err(Status::Unknown)
}
use argon2::{PasswordHash, Argon2, PasswordVerifier};

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    RegInitData, 
    RegFilesData, 
    AuthInfo, 
    AuthStep,
    PersonSignCheckResult
};
use shared_lib::service::api_routes::implements::CryptoApiRoutes;


use crate::config::BackApiState;
use crate::db::sql_queries::persons::get::person_by_inn::get_person_by_inn;
use crate::db::sql_queries::companys::get::company_by_inn_kpp::get_company_by_inn_kpp;
use crate::db::sql_queries::users::get::passw_tel_mail_by_pers_comp_id::{self, get_passw_rel_mail_by_pers_comp_id};



pub(crate) async fn register_step2(
    state: &BackApiState,
    data: &RegFilesData
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: AuthInfo::BackApiError };

    let RegFilesData { 
        json_file, 
        sign_file 
    } = data;

    let json_content = match String::from_utf8(json_file.clone()) {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(
                teck_err = ?err,
                local_err = ?Status::FileReadError,
                "FUN register_step2 FAILED BY String::from_utf8(json_file.clone())"
            );
            return Ok(failed_result);
        }
    };

    let json_data: RegInitData = match serde_json::from_str(&json_content) {
        Ok(d) => d,
        Err(err) => {
            tracing::error!(
                teck_err = ?err,
                local_err = ?Status::MappingError,
                "FUN register_step2 FAILED BY serde_json::from_str(&json_content)"
            );
            return Ok(failed_result);
        }
    };

    let RegInitData { 
        sur_name, 
        first_name, 
        mid_name, 
        pers_inn, 
        snils, 
        comp_inn, 
        kpp, 
        phone, 
        email, 
        password,
        device_id
    } = json_data;

    let person_option = match get_person_by_inn(state, &pers_inn).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN register_step2 FAILED BY FUN get_person_by_inn"
            );
            return Ok(failed_result);
        }
    };

    let person = match person_option {
        Some(p) => p,
        None => {
            tracing::error!(
                local_err = ?&Status::SystemLogicErr,
                "FUN register_step2 FAILED BY WRONG SYSTEM LOGIC, PERSON JUST MUST BE IN DATA"
            );
            return Ok(failed_result);
        }
    };

    let company_option = match get_company_by_inn_kpp(state, &comp_inn, &kpp).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN register_step2 FAILED BY FUN get_company_by_inn_kpp"
            );
            return Ok(failed_result);
        }
    };

    let company = match company_option {
        Some(c) => c,
        None => {
            tracing::error!(
                local_err = ?Status::SystemLogicErr,
                "FUN register_step2 FAILED BY WRONG SYSTEM LOGIC, COMPANY JUST MUST BE IN DATA"
            );
            return Ok(failed_result);
        }
    };

    let record_option = match get_passw_rel_mail_by_pers_comp_id(
            state, 
            &company.comp_id, 
            &person.pers_id).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN register_step2 FAILED BY FUN get_passw_rel_mail_by_pers_comp_id"
            );
            return Ok(failed_result);
        } 
    };

    let (password_hash, prev_phone, prev_email) = match record_option {
        Some(record) => record,
        None => {
            tracing::error!(
                local_err = ?Status::SystemLogicErr,
                "FUN register_step2 FAILED BY WRONG SYSTEM LOGIC,USER JUST MUST BE IN DATA"
            );
            return Ok(failed_result);
        }
    };

    if 
            sur_name != person.metadata.fio.sur_name ||
            first_name != person.metadata.fio.first_name ||
            mid_name != person.metadata.fio.mid_name ||
            pers_inn != person.pers_inn ||
            snils != person.metadata.snils ||
            comp_inn != company.comp_inn ||
            kpp != company.kpp ||
            phone != prev_phone ||
            email != prev_email {
        
        return Ok(failed_result);
    }

    let argon2_items = match PasswordHash::new(&password_hash) {
        Ok(i) => i,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,

                "FUN register_step2 BY WRONG PASSWORD DATA IN SQL Users"
            );
            return Ok(failed_result);
        } 
    };

    match Argon2::default().verify_password(password.to_string().as_bytes(), &argon2_items) {
        Ok(_) => {},
        Err(err) => {
            tracing::warn!(
                tech_err = ?err,
                "USER_SENDED_WRONG_PASSWORD!!!"
            );
            return Ok(AuthStep::Password {text: AuthInfo::WrongPassword});
        }    
    }

    let crypto_url = format!(
        "{}/{}",
        state.config.crypto_servise.url.trim_end_matches('/'),
        CryptoApiRoutes::CryptoVerifyPerson.get_path().trim_start_matches('/')
    );

    let response = match state.config.get_inst_client()
            .post(&crypto_url)
            .json(&data)
            .send()
            .await {
        Ok(r) => r,
        Err(err) => {
            tracing::error!(
                err = ?err,
                local_err = ?Status::QueryGetRequestErr,
                "FUN register_step2 FAILED BY REQUEST TO CRYPTO SERVICE"
            );
            return Ok(failed_result);
        }
    };

    if !response.status().is_success() {
        tracing::error!(
            "FUN register_step2 FAILED BY REQUEST TO CRYPTO SERVICE"
        );
        return Ok(failed_result);
    }

    let check_result: PersonSignCheckResult = match response
            .json()
            .await {
        Ok(r) => r,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN register_step2 FAILED BY MAPPING CryptoVerifyPersonResponse"
            );
            return Ok(failed_result);
        }
    };


    if !check_result.is_signed {
        return Ok(AuthStep::RegisterStep1 {text: AuthInfo::WrongSignFile})
    }










    Ok(failed_result)
}
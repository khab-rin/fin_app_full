use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    RegInitData, 
    RegFilesData, 
    AuthInfo, 
    AuthStep,
    PersonSignCheckResult
};
use shared_lib::service::api_routes::implements::CryptoApiRoutes;
use shared_lib::sql_models::user::implements::UserSetData;
use shared_lib::service::auth_service::implements::SessionUserToken;
use shared_lib::service::auth_service::client_state::SessionUser;


use crate::config::BackApiState;
use crate::db::service::auth_service::helper::{mask_email, mask_string};
use crate::db::sql_queries::persons::get::person_by_inn::get_person_by_inn;
use crate::db::sql_queries::companys::get::company_by_inn_kpp::get_company_by_inn_kpp;
use crate::db::sql_queries::users::get::by_inn_pers_comp_kpp::get_user_by_inn_pers_comp_kpp;
use crate::db::sql_queries::users::get::tel_mail_by_id::get_user_phone_mail_by_id;
use crate::db::sql_queries::users::add::user::add_user;
use crate::db::sql_queries::sessions::set::new_session::new_session;



pub(crate) async fn register_step2(
    state: &BackApiState,
    data: &RegFilesData
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: AuthInfo::BackApiError };

    let RegFilesData { 
        json_file, 
        ..
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


    let existed_user_option = match get_user_by_inn_pers_comp_kpp(
            state, 
            &pers_inn, 
            &comp_inn, 
            &kpp).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN init_user FAILED BY FUN get_user_by_inn_pers_comp_kpp"
            );
            return Ok(failed_result);
        }
    };

    if let Some(u) = existed_user_option {
        let tel_email_option = match get_user_phone_mail_by_id(state, &u.user_id).await {
            Ok(o) => o,
            Err(err) => {
                tracing::error!(
                    local_err = ?err,
                    "FUN init_user FAILED BY FUN get_user_phone_mail_by_id"
                );
                return Ok(failed_result);
            }
        };

        match tel_email_option {
            Some((prev_tel, prev_email)) => {
                return Ok(AuthStep::RegisterStep1Duplicate { 
                    sur_name: mask_string(sur_name.as_ref()),
                    first_name: mask_string(first_name.as_ref()),
                    mid_name: mid_name.as_deref().map(|s| mask_string(s)).unwrap_or_default(),
                    pers_inn: mask_string(pers_inn.as_ref()),
                    snils: mask_string(snils.as_ref()),
                    comp_inn: mask_string(comp_inn.as_ref()),
                    kpp: mask_string(kpp.as_ref()),
                    phone: mask_string(prev_tel.as_ref()),
                    email: mask_email(prev_email.as_ref()),
                    text: AuthInfo::UserAlreadyExists 
                });
            }
            None => {
                tracing::error!(
                    local_err = ?&Status::SystemLogicErr,
                    "FUN init_user FAILED BY SYSTEM LOGIC ERROR --> USER EXIST, TEL EMAIL MISS"
                );
                return Ok(failed_result);
            }
        }
    }




    let crypto_url = format!(
        "{}/{}",
        state.config.crypto_servise.url.trim_end_matches('/'),
        CryptoApiRoutes::CryptoVerifyPerson.get_path().trim_start_matches('/')
    );

    // 1. Отправка запроса
    let response = match state
        .config
        .get_inst_client()
        .post(&crypto_url)
        .json(&data)
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            // Логируем URL, отправляемые данные и подробную ошибку reqwest
            tracing::error!(
                target: "back_api::crypto_client",
                url = %crypto_url,
                payload = ?data,
                err = ?err,
                is_timeout = err.is_timeout(),
                is_connect = err.is_connect(),
                local_err = ?Status::QueryGetRequestErr,
                "FUN register_step2 FAILED BY NETWORK/CONNECT TO CRYPTO SERVICE"
            );
            return Ok(failed_result);
        }
    };

    // 2. Проверка HTTP-статуса ответа (4xx / 5xx)
    if !response.status().is_success() {
        let status_code = response.status();
        
        // Пытаемся вычитать тело ошибки от криптосервиса
        let error_body = response.text().await.unwrap_or_else(|_| "Failed to read body".to_string());

        tracing::error!(
            target: "back_api::crypto_client",
            url = %crypto_url,
            http_status = %status_code,
            response_body = %error_body,
            payload = ?data,
            local_err = ?Status::QueryGetRequestErr,
            "FUN register_step2 FAILED: CRYPTO SERVICE RETURNED ERROR STATUS"
        );
        return Ok(failed_result);
    }

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


    let salt = SaltString::generate(&mut OsRng);
    let hasher = Argon2::default();

    let argon2_hash = match hasher.hash_password(password.as_ref().as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::SystemErr,
                "FUN register_step2 FAILED BY FUN arg2.hash_password"
            );
            return Ok(failed_result);
        }
    };

    let user_set_data = UserSetData {
        pers_id: person.pers_id.clone(),
        comp_id: company.comp_id.clone(),

        phone: phone.clone(),
        password_hash: argon2_hash,
        email: email.clone(),

        guids: std::collections::HashSet::new()
    };

    let user = match add_user(state, &user_set_data).await {
        Ok(u) => u,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN register_step2 FAILED BY FUN  add_user"
            );
            return Ok(failed_result);
        }
    };

    let token = match new_session(state, &user.user_id, &device_id).await {
        Ok(t) => t,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN register_step2 FAILED BY FUN new_session"
            );
            return Ok(failed_result);
        }
    };

    let res = AuthStep::SuccessFull {
        session_user_token: Box::new(SessionUserToken {
            token,
            user: SessionUser {
                person,
                company,
                user,
            }
        })
    };

    Ok(res)

}
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

use shared_lib::{Status, ProcessError};
use shared_lib::service::auth_service::implements::{
    RegInitData, 
    AuthInfo, 
    AuthStep,
};
use shared_lib::service::crypto_service::implements::{
    CheckSignDocData,
    PersonSignCheckResult
};
use shared_lib::service::api_routes::implements::CryptoApiRoutes;
use shared_lib::sql_models::user::implements::UserSetData;
use shared_lib::service::auth_service::implements::SessionUserToken;
use shared_lib::service::auth_service::general::SessionUser;


use crate::config::BackApiState;
use crate::db::service::auth_service::helper::{mask_email, mask_string};
use crate::db::service::mchd::mchd_storage::add_new_manager;
use crate::db::sql_queries::persons::get::person_by_inn::get_person_by_inn;
use crate::db::sql_queries::companys::get::company_by_inn_kpp::get_company_by_inn_kpp;
use crate::db::sql_queries::users::get::by_inn_pers_comp_kpp::get_user_by_inn_pers_comp_kpp;
use crate::db::sql_queries::users::get::tel_mail_by_id::get_user_phone_mail_by_id;
use crate::db::sql_queries::users::add::user::add_user;
use crate::db::sql_queries::sessions::set::new_session::new_session;
use crate::db::service::auth_service::pers_sign_parser::{
    parse_crypto_fields_org,
    check_auth_manager,
    check_auth_person
};


pub(crate) async fn register_step2(
    state: &BackApiState,
    data: &CheckSignDocData
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: AuthInfo::BackApiError };

    let CheckSignDocData { 
        init_file, 
        ..
    } = data;

    let json_content = match String::from_utf8(init_file.clone()) {
        Ok(c) => c,
        Err(err) => {
            err.process_err(Status::FileReadError, "");
            return Ok(failed_result);
        }
    };

    let json_data: RegInitData = match serde_json::from_str(&json_content) {
        Ok(d) => d,
        Err(err) => {
            err.process_err(Status::MappingError, "");
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
    } = json_data.clone();

    let person_option = match get_person_by_inn(state, &pers_inn).await {
        Ok(o) => o,
        Err(err) => {
            err.process_err(err, "");
            return Ok(failed_result);
        }
    };

    let person = match person_option {
        Some(p) => p,
        None => {
            Status::Tech.process_err(Status::SystemLogicErr, "USER MUST BE IN SYSTEM ALREADY");
            return Ok(failed_result);
        }
    };

    let company_option = match get_company_by_inn_kpp(state, &comp_inn, &kpp).await {
        Ok(o) => o,
        Err(err) => {
            err.process_err(err, "");
            return Ok(failed_result);
        }
    };

    let company = match company_option {
        Some(c) => c,
        None => {
            Status::Tech.process_err(Status::SystemLogicErr, "");
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
            err.process_err(err, "");
            return Ok(failed_result);
        }
    };

    if let Some(u) = existed_user_option {
        let tel_email_option = match get_user_phone_mail_by_id(state, &u.user_id).await {
            Ok(o) => o,
            Err(err) => {
                err.process_err(err, "");
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
                Status::Tech.process_err(Status::SystemLogicErr, "");
                return Ok(failed_result);
            }
        }
    }




    let crypto_url = format!(
        "{}/{}",
        state.config.crypto_servise.url.trim_end_matches('/'),
        CryptoApiRoutes::CryptoVerifyPerson.get_path().trim_start_matches('/')
    );

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
            let ext_inf = format!("url = {:?}, is_timeout = {:?}, is_connect = {:?}", crypto_url, err.is_timeout(), err.is_connect());
            err.process_err(Status::QueryGetRequestErr, &ext_inf);
            return Ok(failed_result);
        }
    };

    if !response.status().is_success() {
        let status_code = response.status();
        let error_body = response.text().await.unwrap_or_else(|_| "Failed to read body".to_string());
        let ext_inf = format!("url = {:?}, http_status = {:?}, response_body = {:?}", crypto_url, status_code, error_body);
        Status::Tech.process_err(Status::QueryGetRequestErr, &ext_inf);
        return Ok(failed_result);
    }

    let check_result: PersonSignCheckResult = match response
            .json()
            .await {
        Ok(r) => r,
        Err(err) => {
            err.process_err(Status::MappingError, "");
            return Ok(failed_result);
        }
    };

    if !check_result.is_signed {
        return Ok(AuthStep::RegisterStep1 {text: AuthInfo::WrongSignFile});
    }

    let sign_fields = match parse_crypto_fields_org(&check_result.text) {
        Ok(r) => r,
        Err(err) => {
            err.process_err(err, "");
            return Ok(failed_result)
        }
    };


    if !check_auth_person(&json_data, &sign_fields) {
        return Ok(AuthStep::RegisterStep1 {text: AuthInfo::WrongSignFile});
    }

    let salt = SaltString::generate(&mut OsRng);
    let hasher = Argon2::default();

    let argon2_hash = match hasher.hash_password(password.as_ref().as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(err) => {
            err.process_err(Status::SystemErr, "");
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
            err.process_err(err, "");
            return Ok(failed_result);
        }
    };

    if check_auth_manager(&json_data, &sign_fields) {
        if let Err(err) = add_new_manager(&user.user_id).await {
            err.process_err(err, "");
            return Ok(failed_result);
        }
    }

    let token = match new_session(state, &user.user_id, &device_id).await {
        Ok(t) => t,
        Err(err) => {
            err.process_err(err, "");
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
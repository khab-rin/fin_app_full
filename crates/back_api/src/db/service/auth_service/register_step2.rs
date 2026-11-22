use std::collections::HashSet;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

use shared_lib::{ProcessError, Status, LL};
use shared_lib::sql_models::id_rules::IdRules;
use shared_lib::primitives::frozen::text::{DateTime};
use shared_lib::primitives::composite::implements::Fio;
use shared_lib::service::auth_service::implements::{
    RegInitData, 
    AuthInfo, 
    AuthStep,
};
use shared_lib::service::cryp_serv::implements::{
    CheckSignDocData,
    PersonSignCheckResult
};

use shared_lib::service::api_routes::implements::CryptoApiRoutes;
use shared_lib::sql_models::user::implements::UserSetData;
use shared_lib::sql_models::person::implements::{Person, PersonMetadata};
use shared_lib::service::auth_service::implements::SessionUserToken;
use shared_lib::service::auth_service::general::SessionUser;


use crate::config::BackApiState;
use crate::db::service::auth_service::helper::{mask_email, mask_string};
use crate::db::service::mchd::mchd_storage::add_new_manager;
use crate::db::sql_queries::persons::add::fun_get_update_insert_person::get_update_insert_person_function;
use crate::db::sql_queries::companys::get::fun_get_or_parse_insert_one::get_or_parse_insert_one_company_function;
use crate::db::sql_queries::users::get::by_inn_pers_comp_kpp::get_user_by_inn_pers_comp_kpp;
use crate::db::sql_queries::users::get::tel_mail_by_id::get_user_phone_mail_by_id;
use crate::db::sql_queries::users::add::que::sql_insert_update_user_function;
use crate::db::sql_queries::sessions::set::que_insert_session::sql_insert_session_fun;
use crate::db::service::auth_service::pers_sign_parser::{
    parse_crypto_fields_org,
    check_auth_manager,
    check_auth_person
};


pub(crate) async fn register_step2(
    state: &BackApiState,
    data: &CheckSignDocData
) -> Result<AuthStep, Status> {

    let mut failed_result = AuthStep::TryLater { text: AuthInfo::BackApiError };


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

	let rule = IdRules::Person { pers_inn: pers_inn.clone(), snils: snils.clone() };
	let pers_id = rule.make_stat_id();

	let mut person =  Person {
		pers_id,
		pers_inn: pers_inn.clone(),
		metadata : PersonMetadata {
			snils: snils.clone(),
			fio: Fio {
				sur_name: sur_name.clone(),
				first_name: first_name.clone(),
				mid_name: mid_name.clone(),
			},
			passport: None,
			address: None,
			gender: None,
			birth_day: None,
			phone: HashSet::from([phone.clone()]),
			email: HashSet::from([email.clone()])
		},
		last_update: DateTime::unchecked(chrono::Utc::now())
	};

    person = match get_update_insert_person_function(state, &person).await {
        Ok(p) => p,
        Err(err) => {
            err.process_err(err, "");
            return Ok(AuthStep::from_err(&err));  
        }
    };

    let company_option = match get_or_parse_insert_one_company_function(state, &comp_inn, &kpp).await {
        Ok(o) => o,
        Err(err) => {
            err.process_err(err, "");
            return Ok(AuthStep::from_err(&err));  
        }
    };

    let company = match company_option {
        Some(c) => c,
        None => {
            Status::log(LL::Debug, &format!("Компания не найдена. Инн - {:?}, Кпп - {:?}", comp_inn, kpp));
			if let AuthStep::TryLater { text } = &mut failed_result {
				*text = AuthInfo::MissCompany;
			}
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
            return Ok(AuthStep::from_err(&err));  
        }
    };

    if let Some(u) = existed_user_option {
        let tel_email_option = match get_user_phone_mail_by_id(state, &u.user_id).await {
            Ok(o) => o,
            Err(err) => {
                err.process_err(err, "");
                return Ok(AuthStep::from_err(&err)); 
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

	// Status::log(LL::Info, &format!("check = {}", check_result.text));

    if !check_result.is_signed {
        return Ok(AuthStep::RegisterStep1 {text: AuthInfo::WrongSignFile});
    }

    let sign_fields = match parse_crypto_fields_org(&check_result.text) {
        Ok(r) => r,
        Err(err) => {
            err.process_err(err, "");
            return Ok(AuthStep::from_err(&err));  
        }
    };

	// Status::log(LL::Info, &format!("check = {:?}", sign_fields));


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

	let rule = IdRules::User { comp_id: company.comp_id.clone(), pers_id: person.pers_id.clone() };
	let user_id = rule.make_stat_id();

    let user_set_data = UserSetData {
		user_id,
        pers_id: person.pers_id.clone(),
        comp_id: company.comp_id.clone(),

        phone: phone.clone(),
        password_hash: argon2_hash,
        email: email.clone(),

        guids: std::collections::HashSet::new()
    };

    let user = match sql_insert_update_user_function(&state.pool_fast, &user_set_data).await {
        Ok(u) => u,
        Err(err) => {
            err.process_err(err, "");
            return Ok(AuthStep::from_err(&err));  
        }
    };

    if check_auth_manager(&json_data, &sign_fields) && let Err(err) = add_new_manager(&user.user_id).await {
		err.process_err(err, "");
		return Ok(failed_result);
    }

    let token = match sql_insert_session_fun(&state.pool_fast, &user.user_id, &device_id).await {
        Ok(t) => t,
        Err(err) => {
            err.process_err(err, "");
            return Ok(AuthStep::from_err(&err)); 
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
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2
};

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, DateTime};
use shared_lib::primitives::composite::implements::Fio;
use shared_lib::sql_models::person::implements::{Person, PersonMetadata};
use shared_lib::service::auth_service::implements::{AuthInfo, AuthStep, RegInitData};
use shared_lib::sql_models::user::implements::UserSetData;

use crate::config::BackApiState;
use crate::db::sql_queries::users::get::by_inn_pers_comp_kpp::get_user_by_inn_pers_comp_kpp;
use crate::db::sql_queries::users::get::tel_mail_by_id::get_user_phone_mail_by_id;
use crate::db::sql_queries::persons::add::by_person::add_person;
use crate::db::sql_queries::companys::add::company::add_company;
use crate::db::sql_queries::users::add::user::add_user;
use crate::db::service::auth_service::helper::{mask_email, mask_phone};
use crate::db::parsers::dadata::inn_kpp_query::parse_company_by_inn_kpp;


pub(crate) async fn init_user(
    state: &BackApiState,
    data: &RegInitData
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: AuthInfo::BackApiError };

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
        password 
    } = data;



    let existed_user_option = match get_user_by_inn_pers_comp_kpp(
            state, 
            pers_inn, 
            comp_inn, 
            kpp).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN init_user FAILED BY FUN get_user_by_inn_pers_comp_kpp"
            );
            return Ok(failed_result);
        }
    };

    match existed_user_option {
        Some(u) => {
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
                Some((tel, email)) => {
                    return Ok(AuthStep::InitUserDuplicate { 
                        tel: mask_phone(tel.as_ref()), 
                        email: mask_email(email.as_ref()), 
                        text: AuthInfo::UserAlreadyExists });
                }
                None => {
                    tracing::error!(
                        local_err = ?&Status::SystemLogicErr,
                        "FUN init_user FAILED BY SYSTEM LOGIC ERROR --> USER EXIST, TEL EMAIL MISS"
                    );
                    return Ok(failed_result);
                }
            }
        },
        None => {}
    }
    
    let new_person = Person {
        pers_id: BoxUuid::unchecked(uuid::Uuid::new_v4()),
        pers_inn: pers_inn.clone(),
        metadata: PersonMetadata {
            snils: snils.clone(),
            fio: Fio {
                sur_name: sur_name.clone(),
                first_name: first_name.clone(),
                mid_name: mid_name.clone()
            },
            passport: None,
            address: None,
            gender: None,
            birth_day: None,
            phone: std::collections::HashSet::from([phone.clone()]),
            email: std::collections::HashSet::from([email.clone()]),
        },
        last_update: DateTime::unchecked(chrono::Local::now())
    };

    let person = match add_person(state, &new_person).await {
        Ok(p) => p,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN init_user FAILED BY FUN add_person"
            );
            return Ok(failed_result);
        }
    };

    let new_company = match parse_company_by_inn_kpp(state, comp_inn, kpp).await {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN init_user FAILED BY FUN parse_company_by_inn_kpp"
            );
            return Ok(failed_result)
        }
    };

    let company = match add_company(state, &new_company).await {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN init_user FAILED BY FUN add_company"
            );
            return Ok(failed_result)
        }
    };

    let salt = SaltString::generate(&mut OsRng);
    let arg2 = Argon2::default();

    let server_passord_hash = match arg2.hash_password(password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                "FUN init_user FAILED BY FUN arg2.hash_password"
            );
            return Ok(failed_result);
        }
    };

    let user_set_data = UserSetData {
        pers_id: person.pers_id.clone(),
        comp_id: company.comp_id.clone(),

        phone: phone.clone(),
        password_hash: server_passord_hash,
        email: email.clone(),

        guids: std::collections::HashSet::new()
    };


    let _ = match add_user(state, &user_set_data).await {
        Ok(_) => {},
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN init_user FAILED BY FUN add_user"
            );
            return Ok(failed_result);
        }
    };


    Ok(AuthStep::InitUserSuccess {  })
}
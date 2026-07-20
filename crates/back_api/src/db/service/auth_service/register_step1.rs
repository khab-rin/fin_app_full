use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, DateTime};
use shared_lib::primitives::composite::implements::Fio;
use shared_lib::sql_models::person::implements::{Person, PersonMetadata};
use shared_lib::service::auth_service::implements::{AuthInfo, AuthStep, RegInitData};

use crate::config::BackApiState;
use crate::db::sql_queries::persons::add::by_person::add_person;
use crate::db::sql_queries::companys::add::company::add_company;
use crate::db::parsers::dadata::inn_kpp_query::parse_company_by_inn_kpp;
use crate::db::service::auth_service::make_init_files::make_init_files;


pub(crate) async fn register_step1(
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
        ..
    } = data;


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

    let _ = match add_person(state, &new_person).await {
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

    let _ = match add_company(state, &new_company).await {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN init_user FAILED BY FUN add_company"
            );
            return Ok(failed_result)
        }
    };

    // let salt = SaltString::generate(&mut OsRng);
    // let arg2 = Argon2::default();

    // let server_passord_hash = match arg2.hash_password(password.as_bytes(), &salt) {
    //     Ok(h) => h.to_string(),
    //     Err(err) => {
    //         tracing::error!(
    //             tech_err = ?err,
    //             "FUN init_user FAILED BY FUN arg2.hash_password"
    //         );
    //         return Ok(failed_result);
    //     }
    // };

    // let user_set_data = UserSetData {
    //     pers_id: person.pers_id.clone(),
    //     comp_id: company.comp_id.clone(),

    //     phone: phone.clone(),
    //     password_hash: server_passord_hash,
    //     email: email.clone(),

    //     guids: std::collections::HashSet::new()
    // };


    // let user = match add_user(state, &user_set_data).await {
    //     Ok(u) => {u},
    //     Err(err) => {
    //         tracing::error!(
    //             local_err = ?err,
    //             "FUN init_user FAILED BY FUN add_user"
    //         );
    //         return Ok(failed_result);
    //     }
    // };

    // let user_id = user.user_id;

    make_init_files(state, data).await

}
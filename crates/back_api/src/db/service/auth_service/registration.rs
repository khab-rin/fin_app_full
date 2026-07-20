use std::collections::HashSet;
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
    RegFilesData, 
    AuthStep,
    AuthInfo
};

use crate::config::BackApiState;
use crate::db::sql_queries::users::get::exists_user_by_pers_comp::exists_user_by_pers_comp;
use crate::db::sql_queries::persons::add::by_person::add_person;
use crate::db::sql_queries::companys::get::company_by_inn_kpp::get_company_by_inn_kpp;
use crate::db::sql_queries::companys::helper::make_new_company;
use crate::db::sql_queries::users::add::user::add_user;
use crate::db::sql_queries::sessions::set::new_session::new_session;
use crate::db::service::auth_service::pers_sign_parser::person_checker;
use crate::db::sql_queries::companys::add::company::add_company;



pub(crate) async fn register_new_user(
    state: &Arc<BackApiState>,
    payload: RegFilesData
) -> Result<AuthStep, Status> {

    // let RegistrationData { 
    //     person, 
    //     comp_inn,
    //     kpp, 
    //     password, 
    //     device_id, 
    //     phone,
    //     email,
    // } = payload;
    
    // let failed_data = (
    //     &person, 
    //     &comp_inn, 
    //     &kpp, 
    //     &device_id);

    // let check_hash = blake3::hash(&document).to_hex();

    // if !check_hash.as_str().eq_ignore_ascii_case(&doc_hash) {
    //     tracing::warn!(
    //         failed_data = ?failed_data,
    //         "PERSON LEND ANOTHER FILE"
    //     );
    //     return Ok(AuthStep::RegisterStep1 {text: AuthInfo::MissedFile});
    // }

    // let crypto_verify_request = CryptoVerifyData {
    //     document,
    //     signature
    // };
    
    // let crypto_url = format!(
    //     "{}/{}",
    //     state.config.crypto_servise.url.trim_end_matches('/'),
    //     CryptoApiRoutes::CryptoVerifyPerson.get_path().trim_start_matches('/')
    // );

    // let response = match state.config.get_inst_client()
    //     .post(&crypto_url)
    //     .json(&crypto_verify_request)
    //     .send()
    //     .await {
    //         Ok(r) => r,
    //         Err(err) => {
    //             tracing::error!(
    //                 err = ?err,
    //                 local_err = ?Status::QueryGetRequestErr,
    //                 failed_data = ?failed_data,
    //                 "FUN register_new_user FAILED BY REQUEST TO CRYPTO SERVICE"
    //             );
    //             return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
    //         }
    //     };
    
    // if !response.status().is_success() {
    //     tracing::error!(
    //         failed_data = ?failed_data,
    //         "FUN register_new_user FAILED BY REQUEST TO CRYPTO SERVICE - CONNECTION PROBLEMS"
    //     );
    //     return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
    // }


    // let crypto_response: CryptoServiceResponse = match response
    //     .json()
    //     .await  {
    //     Ok(r) => r,
    //     Err(err) => {
    //         tracing::error!(
    //             tech_err = ?err,
    //             local_err = ?Status::MappingError,
    //             failed_data = ?failed_data,
    //             "FUN register_new_user FAILED BY MAPPING CryptoVerifyPersonResponse"
    //         );
    //         return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
    //     }
    // };


    // if !crypto_response.is_signed {
    //     return Ok(AuthStep::RegisterStep1 { text: AuthInfo::WrongSignFile });
    // }


    // match person_checker(&crypto_response.text, &person) {
    //     Ok(true) => {},
    //     Ok(false) => {
    //         return Ok(AuthStep::RegisterStep1 { text: AuthInfo::WrongPerson });
    //     }
    //     Err(err) => {
    //         tracing::error!(
    //             err = ?err,
    //             "FUN register_new_user FAILED BY FUN person_checker"
    //         );
    //         return Ok(AuthStep::TryLater { text: AuthInfo::BackApiError });
    //     }
    // }
   
    // let person = match add_person(state, &person).await {
    //     Ok(p) => p,
    //     Err(err) => {
    //         tracing::error!(
    //             err = ?err,
    //             "FUN register_new_user FAILED BY ADD PERSON SQL QUERY"
    //         );
    //         return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
    //     }
    // };

    // let company_option = match get_company_by_inn_kpp(state, &comp_inn, &kpp).await {
    //     Ok(c_o) => c_o,
    //     Err(err) => {
    //         tracing::error!(
    //             err = ?err,
    //             failed_data = ?failed_data,
    //             "FUN get_company_by_inn_kpp FAILED IN FUN get_company_by_inn_kpp"
    //         );
    //         None
    //     }
    // };

    // let company = match company_option {
    //     Some(c) => {
    //         c
    //     },
    //     None => {
    //         match make_new_company(state, &comp_inn, &kpp).await {
    //             Ok(c) => {
    //                 match add_company(state, &c).await {
    //                     Ok(comp) => comp,
    //                     Err(err) => {
    //                         tracing::error!(
    //                             err = ?err,
    //                             "FUN register_new_user FAILED BY FUN add_company"
    //                         );
    //                         return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
    //                     }
    //                 }
    //             },
    //             Err(err) => {
    //                 tracing::error!(
    //                     err = ?err,
    //                     failed_data = ?failed_data,
    //                     "FUN register_new_user FAILED BY FUN make_new_company"
    //                 );
    //                 return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
    //             }
    //         }
    //     }
    // };

    // let exist_flag = match exists_user_by_pers_comp(
    //     state, 
    //     &person.pers_id, 
    //     &company.comp_id).await {
    //         Ok(f) => f,
    //         Err(err) => {
    //             tracing::error!(
    //                 err =?err,
    //                 failed_data = ?failed_data,
    //                 "FUN get_session_user_by_company FAILED IN FUN register_new_user"
    //             );
    //             false
    //         }
    //     };


    // if exist_flag {
    //     return Ok(AuthStep::Password{text: AuthInfo::UserAlreadyExists});
    // }

    // let salt = SaltString::generate(&mut OsRng);

    // let argon2 = Argon2::default();

    // let server_password_hash = match argon2
    //     .hash_password(password.as_bytes(), &salt) {
    //     Ok(h) => h.to_string(),
    //     Err(err) => {
    //         tracing::error!(
    //             err = ?err,
    //             failed_data = ?failed_data,
    //             "FUN register_new_user FAILED BY ARGON2 HASHING PASSWORD"
    //         );
    //         return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
    //     }
    // };

    // let user_set_data = UserSetData {
    //     pers_id: person.pers_id.clone(),
    //     comp_id: company.comp_id.clone(),
    //     phone,
    //     password_hash: server_password_hash,
    //     email,
    //     guids: HashSet::new(),
    // };

    // let user = match add_user(state, &user_set_data).await {
    //     Ok(u) => u,
    //     Err(err) => {
    //         tracing::error!(
    //             err = ?err,
    //             failed_data = ?failed_data,
    //             "FUN register_new_user FAILED BY FUN set_user"
    //         );
    //         return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
    //     }
    // };

    // let token = match new_session(state, &user.user_id, &device_id).await {
    //     Ok(t) => t,
    //     Err(err) => {
    //         tracing::error!(
    //             err = ?err,
    //             failed_data = ?failed_data,
    //             "FUN register_new_user FAILED BY new_session FUN"
    //         );
    //         return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
    //     }
    // };

    // let session_user = SessionUser {
    //     user,
    //     person,
    //     company
    // };

    // let session_user_token = SessionUserToken {
    //     user: session_user,
    //     token
    // };

    // let ok_result = AuthStep::SuccessFull { session_user_token: Box::new(session_user_token) };

    Err(Status::Unknown)
}
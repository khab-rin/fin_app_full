use crate::{Status, ClientState};
use crate::sql_models::company::implements::Company;
use crate::sql_models::operation::parser::InnKppMapAcc;
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::back_api::post_query::post_query_back_api;

pub async fn add_companys_by_inn_cpp_acc(
    state: &ClientState,
    data: &InnKppMapAcc
) -> Result<(), Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN add_companys_by_inn_cpp_acc FAILED BY state.get_session()", err
            );
            return Err(err);
        }
    };

   let response = match post_query_back_api(
            state, 
            state.config.get_sql_long(), 
            ApiRoutes::SqlComppanysAddByInnKpp, 
            data).await {
        Ok(r) => r,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN add_companys_by_inn_cpp_acc FAILED BY FUN post_query_back_api",err
            );
            return Err(err);
        }
    };

    let companys: Vec<Company> = match response.json().await {
        Ok(v) => v,
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN add_companys_by_inn_cpp_acc FAILED BY FUN post_query_back_api",
                err, Status::MappingError
            );
            return Err(Status::MappingError);
        }
    };

    let mut tx = match session.local_db.begin().await {
        Ok(t) => t,
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN add_companys_by_inn_cpp_acc FAILED BY session.local_db.begin()",
                err, Status::SqLitePoolErr
            );
            return Err(Status::SqLitePoolErr);
        }
    };

    for company in companys.iter() {
        let comp_id = company.comp_id.as_ref();
        let comp_inn = company.comp_inn.as_ref();
        let kpp = company.kpp.as_ref();
        let comp_type = company.comp_type.as_str();
        let comp_status = company.comp_status.as_str();
        let metadata = serde_json::to_string(&company.metadata).unwrap_or_else(|_| "{}".to_string());
        let last_update = company.last_update.as_ref();

        match sqlx::query_file!(
            "src/client/sql_queries/companys/add/add_company.sql",
            comp_id,
            comp_inn,
            kpp,
            comp_type,
            comp_status,
            metadata,
            last_update
        ).execute(&mut *tx).await {
            Ok(_) => {},
            Err(err) => {
                log::error!(
                    "tech_err = {:?}, local_err = {:?}, FUN add_companys_by_inn_cpp_acc FAILED BY SQL QUERY",
                    err, Status::SqlQueryWrongLogic
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        };


    }

    match tx.commit().await {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN add_companys_by_inn_cpp_acc FAILED BY tx.commit().await",
                err, Status::SqliteCommitErr
            );
            return Err(Status::SqliteCommitErr);
        }
    }

    Ok(())
}
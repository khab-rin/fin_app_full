use crate::{Status, ClientState, ProcessError};
use crate::sql_models::company::implements::{Company, CompCrateData};
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::back_api::post_query::post_query_back_api;

pub async fn add_companys_by_inn_cpp_acc(
    state: &ClientState,
    data: &Vec<CompCrateData>
) -> Result<Vec<Company>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

   let response = post_query_back_api(
            state, 
            state.config.get_sql_long(), 
            ApiRoutes::SqlComppanysAddByInnKpp, 
            data)
        .await
        .map_err(|err| err.process_err(err, ""))?;

    let companys: Vec<Company> = response.json().await
        .map_err(|err| err.process_err(Status::MappingError, ""))?; 

    let mut tx = session.local_db.begin().await
        .map_err(|err| err.process_err(Status::SqLitePoolErr, ""))?;

    for company in companys.iter() {
        let comp_id = company.comp_id.as_ref();
        let comp_inn = company.comp_inn.as_ref();
        let kpp = company.kpp.as_ref();
        let comp_type = company.comp_type.as_str();
        let comp_status = company.comp_status.as_str();
        let metadata = serde_json::to_string(&company.metadata).unwrap_or_else(|_| "{}".to_string());
        let last_update = company.last_update.as_ref();

        sqlx::query_file!(
            "src/client/sql_queries/companys/add/add_company.sql",
            comp_id,
            comp_inn,
            kpp,
            comp_type,
            comp_status,
            metadata,
            last_update
        ).execute(&mut *tx).await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    }

    tx.commit().await
        .map_err(|err| err.process_err(Status::SqliteCommitErr, ""))?;

    Ok(companys)
}
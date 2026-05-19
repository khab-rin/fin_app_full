use std::sync::Arc;
use axum::{Json, extract::State};

use shared_lib::sql_models::company_models::implements::Company;
use shared_lib::alias_types::implements::InnKppAccVec;
use shared_lib::Status;

use crate::config::BackApiState;
use crate::db::sql_queries::companys::add_company::query::sync_server_companys;

type HandlerResAddCompany = Result<Json<Vec<Company>>, Status>;

pub async fn auto_add_company_handler(
    State(state): State<Arc<BackApiState>>,
    Json(payload): Json<InnKppAccVec>
) -> HandlerResAddCompany {
 
    let res = sync_server_companys(&state, payload).await?;
    Ok(Json(res))

}

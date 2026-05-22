use std::path::Path;

use shared_lib::Status;
use shared_lib::service::process::bank_statement::implements::BankStatementProceedResult;
use shared_lib::sql_models::company::implements::{Company, CompanyCurt};
use shared_lib::alias_types::implements::{InnKppAccMap, InnKppAccVec};
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::client_state::ClientState;

use crate::parsers::bank_statement::parser::bank_parser;
use crate::config::Config;
use crate::sql_queries::companys::sync_company::query::sync_local_companys;
use crate::service::process::bank_statement::make_inn_kpp_vec::{make_inn_kpp_map_func};
use crate::POOL;


pub(crate) async fn process_statement<P: AsRef<Path>>(
    state: &ClientState,
    path: P
) -> Result<BankStatementProceedResult, Status> {
    let mut result = BankStatementProceedResult::default();

    let client = Config::get_client();
    
    let pool = POOL.get().ok_or(Status::MainPoolGetError)?;

    let parse_result = bank_parser(path)?;

    let companys_map:InnKppAccMap = make_inn_kpp_map_func(&parse_result.correct_lines);

    let companys_vec:InnKppAccVec = companys_map.into_iter().collect();

    let api_url = format!(
        "{}/{}", 
        state.api_url.trim_end_matches('/'), 
        ApiRoutes::AutoAddCompany.get_path().trim_start_matches('/'));

    let response = client.post(api_url)
        .json(&companys_vec)
        .send()
        .await
        .inspect_err(|err| {
            log::error!(
                "tech_err = {}, stat_err = {}",
                err,
                Status::FrontBackPostError
            );
        })
        .map_err(|_| Status::FrontBackPostError)?;
    

    let companys: Vec<Company> = response
        .json()
        .await
        .inspect_err(|err| {
            log::error!(
                "tech_err = {}, stat_err = {}",
                err,
                Status::FrontBackPostResponseParseError
            );
        })
        .map_err(|_| Status::FrontBackPostResponseParseError)?;

    
    let id_inn_kpp_pairs:Vec<CompanyCurt> = sync_local_companys(pool, &companys).await?;

    result.companys_curt = id_inn_kpp_pairs;


    Ok(result)
}

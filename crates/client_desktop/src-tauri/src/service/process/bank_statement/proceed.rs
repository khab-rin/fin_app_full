use std::path::Path;

use shared_lib::Status;
use shared_lib::service::process::bank_statement::implements::BankStatementProceedResult;
use shared_lib::sql_models::company::implements::{Company, CompanyCurt};
use shared_lib::alias_types::implements::{InnKppAccMap, InnKppAccVec};
use shared_lib::service::api_routes::implements::ApiRoutes;

use crate::state::ClientState;
use crate::parsers::bank_statement::parser::bank_parser;
use crate::sql_queries::companys::sync_company::query::sync_local_companys;
use crate::service::process::bank_statement::make_inn_kpp_vec::{make_inn_kpp_map_func};


pub(crate) async fn process_statement<P: AsRef<Path>>(
    state: &ClientState,
    path: P
) -> Result<BankStatementProceedResult, Status> {

    let mut result = BankStatementProceedResult::default();

    let parse_result = bank_parser(state, path).await?;

    let companys_map:InnKppAccMap = match make_inn_kpp_map_func(state, &parse_result.correct_lines).await {
        Ok(c) => c,
        Err(err) => {
            log::error!("FUN process_statement FAILED BY make_inn_kpp_map_func, err={}", err);
            return Err(err);
        }
    };

    let companys_vec:InnKppAccVec = companys_map.into_iter().collect();

    let api_url = format!(
        "{}/{}", 
        state.config.back_api_url().trim_end_matches('/'), 
        ApiRoutes::AutoAddCompany.get_path().trim_start_matches('/'));
    
    let client = state.config.get_std_client();

    let response = client.post(api_url)
        .json(&companys_vec)
        .send()
        .await
        .inspect_err(|err| {
            log::error!(
                "tech_err = {}, stat_err = {}",
                err,
                Status::QueryPostRequestErr
            );
        })
        .map_err(|_| Status::QueryPostRequestErr)?;
    
    if response.status() != 200 {
        log::error!(
            "FUN process_statement FAILED BY QUERY TO BACK API, response = {:?}, local_err = {}",
            response, Status::BackApiError
        );
        return Err(Status::BackApiError);
    }

    let companys: Vec<Company> = response
        .json()
        .await
        .inspect_err(|err| {
            log::error!(
                "tech_err = {}, local_err = {}",
                err,
                Status::QueryBodyReadErr
            );
        })
        .map_err(|_| Status::QueryBodyReadErr)?;

    
    let id_inn_kpp_pairs:Vec<CompanyCurt> = sync_local_companys(state, &companys).await?;

    result.companys_curt = id_inn_kpp_pairs;


    Ok(result)
}

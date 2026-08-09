use std::sync::Arc;

use shared_lib::{ProcessError, Status};
use shared_lib::primitives::frozen::text::{CompInn, Kpp};
use shared_lib::parsers::dadata::implements::*;

use crate::config::BackApiState;

pub async fn dadata_reqwest_func(
    state: &Arc<BackApiState>, 
    inn: &CompInn, 
    kpp: &Kpp
) -> Result<CtrprtyMetadata, Status> {

    tracing::debug!("dadata_reqwest_func started");

    
    let client = state.config.get_inst_client();

    let header = state.config.get_dadata_header();
    let url =  &state.config.dadata.dadata_comp_url;

    let response = client
        .post(url)
        .headers(header.clone())
        .json(&serde_json::json!({"query": inn}))
        .send()
        .await
        .map_err(|err| err.process_err(Status::QueryPostRequestErr, ""))?;
        

    let status = response.status();

    if !status.is_success() {
        let error_body = response.text().await.unwrap_or_else(|_| "Не удалось прочитать тело ответа".to_string());
        return Err(Status::QueryPostRequestErr.process_err(Status::QueryPostRequestErr, &error_body));
    }

    let resp_wrap:DadaRespWrap = response
        .json()
        .await
        .map_err(|err| err.process_err(Status::MappingError, ""))?;


    let mut iterator = resp_wrap.suggestions.into_iter();

    let mut main_metadata = iterator
        .next()
        .ok_or_else(|| Status::Tech.process_err(Status::QueryResponseFormatErr, ""))?
        .data
        .ok_or_else(||Status::Tech.process_err(Status::QueryResponseFormatErr, ""))?;

    if main_metadata.kpp.is_none() {
        main_metadata.kpp = Some(Kpp::new("0")?);
    }
  
    let element = iterator
        .find(|s| {
            s.data.as_ref().and_then(|d| d.kpp.as_ref()) == Some(kpp)
        });
    
    if let Some(branch_elem) = element {
        if let Some(branch_metadata) = branch_elem.data {
            main_metadata.merge_dynamic(branch_metadata);
        }
    }


    Ok(main_metadata)
}

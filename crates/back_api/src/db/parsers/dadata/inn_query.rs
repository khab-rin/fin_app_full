use shared_lib::{ProcessError, Status};
use shared_lib::primitives::frozen::text::{CompInn, Kpp};
use shared_lib::parsers::dadata::implements::*;

use crate::config::BackApiState;

pub async fn dadata_reqwest_func(
    state: &BackApiState, 
    comp_inn: &CompInn, 
    kpp: &Kpp
) -> Result<Option<CtrprtyMetadata>, Status> {

    tracing::debug!("dadata_reqwest_func started");

    let ext_info = format!("inn = {:?}, kpp = {:?}", comp_inn, kpp);
    
    let client = state.config.get_inst_client();

    let header = state.config.get_dadata_header();
    let url =  &state.config.dadata.dadata_comp_url;

    let response = client
        .post(url)
        .headers(header.clone())
        .json(&serde_json::json!({"query": comp_inn}))
        .send()
        .await
        .map_err(|err| err.process_err(Status::QueryPostRequestErr, ""))?;
        

    let status = response.status();

    if !status.is_success() {
        let error_body = response.text().await.unwrap_or_else(|_| "Не удалось прочитать тело ответа".to_string());
        return Err(Status::Tech.process_err(Status::QueryPostRequestErr, &error_body));
    }

    let raw_body = response
        .text()
        .await
        .map_err(|err| err.process_err(Status::MappingError, &ext_info))?;


    let resp_wrap: DadaRespWrap = serde_json::from_str(&raw_body)
        .map_err(|err| {
            let info = format!("SerdeError: {}; {}", err, ext_info);
            err.process_err(Status::MappingError, &info)
        })?;


    let mut iterator = resp_wrap.suggestions.into_iter();

    let mut  main_metadata = match iterator.next() {
        Some(elem) => elem.data
            .ok_or_else(|| 
                Status::Tech.process_err(Status::QueryResponseFormatErr, ""))?,
        None => return Ok(None)
    };

    if kpp.is_empty() {
        return Ok(Some(main_metadata));
    }

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


    Ok(Some(main_metadata))
}

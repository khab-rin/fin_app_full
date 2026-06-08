use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{CompInn, Kpp};
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
        .inspect_err(|err| {
            tracing::error!(
                tech_error = ?err,
                status_err = ?Status::QueryPostRequestErr,
                inn = %inn, 
                kpp = %kpp);
        })
        .map_err(|_| Status::QueryPostRequestErr)?;

    let status = response.status();

    if !status.is_success() {
        let error_body = response.text().await.unwrap_or_else(|_| "Не удалось прочитать тело ответа".to_string());
        
        tracing::error!(
            status_code = %status,
            body = %error_body,
            "DaData вернула ошибку вместо данных компании!"
        );
        return Err(Status::QueryPostRequestErr);
    }

    let resp_wrap:DadaRespWrap = response
        .json()
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_error = ?err,
                status_err = ?Status::MappingError,
                inn = %inn, 
                kpp = %kpp);
        })
        .map_err(|_| Status::MappingError)?;

    let mut iterator = resp_wrap.suggestions.into_iter();

    let mut main_metadata = iterator
        .next()
        .ok_or(Status::QueryResponseFormatErr)
        .inspect_err(|err| {
            tracing::error!(
                status_err = ?err,
                inn = %inn, 
                kpp = %kpp);
        })?
        .data
        .ok_or(Status::QueryResponseFormatErr)
        .inspect_err(|err| {
            tracing::error!(
                status_err = ?err,
                inn = %inn, 
                kpp = %kpp);
        })?;

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

    
    if let Some(a) = main_metadata
            .address
            .as_mut()
            .and_then(|a| a.address_data.as_mut()) {
        if a.lev_9_1_type.is_some() {
            let prev_t9 = a.lev_9_type.take().unwrap_or_default();
            let prev_n9 = a.lev_9_name.take().unwrap_or_default();
            let prev_t91 = a.lev_9_1_type.take().unwrap_or_default();
            let prev_n91 = a.lev_9_1_name.take().unwrap_or_default();
            let mut full_t = format!("{} {}", prev_t9, prev_t91);
            a.lev_9_type = Some(full_t.trim().to_owned().into_boxed_str());
            full_t = format!("{} {}", prev_n9, prev_n91);
            a.lev_9_name = Some(full_t.trim().to_owned().into_boxed_str());
        }
    };

    Ok(main_metadata)
}

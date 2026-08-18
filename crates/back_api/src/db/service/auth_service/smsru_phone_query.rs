use std::sync::Arc;

use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::Phone;
use shared_lib::service::auth_service::implements::SmsruCallResponse;

use crate::config::BackApiState;

pub(crate) async fn smsru_get_phone(
    state: &Arc<BackApiState>,
    phone: &Phone
) -> Result<(String, Phone), Status> {

    let url = &state.config.smsru.call_add_url;
    let api_key = &state.config.smsru.api;
    let client = state.config.get_std_client();

    let query_params = vec![
        ("api_id", api_key.to_string()),
        ("phone", phone.to_string()),
        ("json", "1".to_string()),
    ];

     
    let response = client
        .get(url)
        .query(&query_params)
        .send()
        .await
        .map_err(|err| err.process_err(Status::QueryGetRequestErr, ""))?;


    let data: SmsruCallResponse = response
        .json()
        .await
        .map_err(|err| err.process_err(Status::MappingError, ""))?;


    if data.status == "OK" && *data.status_code.as_ref() == 100 {
        
        let check_id = data
            .check_id
            .ok_or_else(|| Status::Tech.process_err(Status::QueryResponseFormatErr, ""))?;

        
        let call_phone:Phone = data
            .call_phone
            .ok_or_else(|| Status::Tech.process_err(Status::QueryResponseFormatErr, ""))?;
        
        Ok((check_id, call_phone))

    } else { Err(Status::BackSmsRuBalance)}   

}
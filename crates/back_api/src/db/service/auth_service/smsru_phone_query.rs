use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::Phone;
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
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::QueryGetRequestErr,
                "FUN smsru_get_phone FAILED BY GET TEL QUERY"
            )
        }).map_err(|_| Status::QueryGetRequestErr)?;

    let data: SmsruCallResponse = response
        .json()
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN smsru_get_phone FAILED BY MAPPING SmsruCallResponse"

            )
        }).map_err(|_| Status::MappingError)?;

    if data.status == "OK" && data.status_code == 100 {
        
        let check_id = data
            .check_id
            .ok_or(Status::QueryResponseFormatErr)
            .inspect_err(|err| {
                tracing::warn!(
                    loacl_err = ?err,
                    "FUN smsru_get_phone FAILED BY MISSING check_id IN RESPONSE"
                )
            })?;
        
        let call_phone:Phone = data
            .call_phone
            .ok_or(Status::QueryResponseFormatErr)
            .inspect_err(|err| {
                tracing::warn!(
                    local_err = ?err,
                    "FUN smsru_get_phone FAILED BY MISSING check_id IN RESPONSE"
                )
            })?;
        
        Ok((check_id, call_phone))

    } else { Err(Status::BackSmsRuBalance)}   

}
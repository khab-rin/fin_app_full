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
    let client = state.config.get_client();
     
    let response = client
        .get(url)
        .query(&[
            ("api_id", api_key.as_str()),
            ("phone", phone),
            ("json", "1")
        ])
        .send()
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::BackAuthSmsRuQuery
            )
        }).map_err(|_| Status::BackAuthSmsRuQuery)?;

    let data: SmsruCallResponse = response
        .json()
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::BackAuthSmsruResponseMapping
            )
        }).map_err(|_| Status::BackAuthSmsruResponseMapping)?;

    if data.status == "OK" && data.status_code == 100 {
        
        let check_id = data
            .check_id
            .ok_or(Status::BackAuthSmsRuWrongResponse)
            .inspect_err(|_| {
                tracing::warn!(
                    teck_err = ?Status::BackAuthSmsRuWrongResponse,
                )
            })?;
        
        let call_phone:Phone = data
            .call_phone
            .ok_or(Status::BackAuthSmsRuWrongResponse)
            .inspect_err(|err| {
                tracing::warn!(
                    local_warn = ?err,
                )
            })?;
        
        Ok((check_id, call_phone))

    } else { Err(Status::BackAuthSmsRuQuery)}   

}
use std::sync::Arc;

use shared_lib::{ProcessError, Status};
use shared_lib::service::auth_service::implements::{SmsruGetResResponse, SmsRuResponseTextCode};

use crate::config::BackApiState;



pub(crate) async fn smsru_get_cf(
    state: &Arc<BackApiState>,
    external_id: &str
) -> Result<SmsRuResponseTextCode, Status> {

    let query_params = vec![
        ("api_id", state.config.smsru.api.as_str()),
        ("check_id", external_id),
        ("json", "1")
    ];

    let client = state.config.get_std_client();

    let response = client
        .get(&state.config.smsru.get_stat_url)
        .query(&query_params)
        .send()
        .await
        .map_err(|err| err.process_err(Status::QueryGetRequestErr, ""))?; 


            
    let text_body = response.text().await
        .map_err(|err| err.process_err(Status::QueryBodyReadErr, ""))?;


    let smsru_response: SmsruGetResResponse = serde_json::from_str(&text_body)
        .map_err(|err| err.process_err(Status::MappingError, ""))?;


    if smsru_response.status_code != 100 {
        return Err(Status::Tech.process_err(Status::BackSmsRuBalance, ""));
    }


    if let Some(status) = smsru_response.check_status {
        match status {
            400 => Ok(SmsRuResponseTextCode::Polling),
            401 => Ok(SmsRuResponseTextCode::SuccessConfirmed),
            402 => Ok(SmsRuResponseTextCode::TimeOut),
            _ => Ok(SmsRuResponseTextCode::UnknownCode)
        }
    } else {
        Err(Status::Tech.process_err(Status::BackApiError, ""))
    }


}
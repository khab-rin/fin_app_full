use std::sync::Arc;

use shared_lib::Status;
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

    let response = match client
        .get(&state.config.smsru.get_stat_url)
        .query(&query_params)
        .send()
        .await {
            Ok(r) => r,
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?&Status::QueryGetRequestErr,
                    "FUN smsru_get_cf FAILED BY SMSRU GET QUERY"
                );
                return Err(Status::QueryGetRequestErr);
            }
        };


            
    let text_body = match response.text().await {
        Ok(t) => t,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::QueryBodyReadErr,
                "FUN smsru_get_cf FAILED BY READ SMSRU RESPONSE"
            );
            return Err(Status::QueryBodyReadErr);
        }
    };

    let smsru_response: SmsruGetResResponse = match serde_json::from_str(&text_body) {
        Ok(parsed) => parsed,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN smsru_get_cf FAILED BY MAPPING SMSRU RESPONSE BODY"
            );
            return Err(Status::MappingError);
        }
    };

    if smsru_response.status_code != 100 {
        tracing::error!(
            local_err = ?Status::BackSmsRuBalance,
            "FUN smsru_get_cf FAILED BY SMSRU RESPONSE CODE"
            );
        return Err(Status::BackSmsRuBalance)
    }



    if let Some(status) = smsru_response.check_status {
        match status {
            400 => Ok(SmsRuResponseTextCode::Polling),
            401 => Ok(SmsRuResponseTextCode::SuccessConfirmed),
            402 => Ok(SmsRuResponseTextCode::TimeOut),
            _ => Ok(SmsRuResponseTextCode::UnknownCode)
        }
    } else {
        Err(Status::BackApiError)
    }


}
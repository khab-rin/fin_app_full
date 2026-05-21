use std::sync::Arc;

use reqwest_middleware::{ClientBuilder,ClientWithMiddleware,RequestBuilder, Extension, Error, RequestInitialiser,reqwest};

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::DateTime;
use shared_lib::service::auth_service::implements::{SmsruGetResResponse, SmsRuResponseTextCode};

use crate::config::BackApiState;



pub(crate) async fn smsru_get_cf(
    state: &Arc<BackApiState>,
    expires_t: &DateTime,
    external_id: &str
) -> Result<bool, Status> {

    let now = chrono::Utc::now();

    let remaining = expires_t.as_ref().signed_duration_since(now);

    if remaining.num_seconds() < 0 { return Ok(false); }

    let total_duration = std::time::Duration::from_secs(remaining.num_seconds() as u64);
    let poll_intervals = &state.config.network.std_poll_intervals;

    let query_params = vec![
        ("api_id", state.config.smsru.api.as_str()),
        ("check_id", external_id),
        ("json", "1")
    ];

    let client = state.config.get_std_client();


    let pool_result = tokio::time::timeout(total_duration, async {
        loop {

            let resp = client
                .get(&state.config.smsru.get_stat_url)
                .json(&query_params)
                .send()
                .await;

            let response = match client
                .get(&state.config.smsru.get_stat_url)
                .query(&query_params)
                .send()
                .await {
                    Ok(r) => r,
                    Err(err) => {
                        tracing::warn!(
                            tech_err = ?err,
                            local_err = ?Status::QueryGetRequestErr,
                            "FUN smsru_get_cf BAD GET QUERY"
                        ); 
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        continue; 
                    } 
                };
            
            let text_body = match response.text().await {
                Ok(t) => t,
                Err(err) => {
                    tracing::error!(
                        tech_err = ?err,
                        local_err = ?Status::QueryBodyReadErr,
                        "SMSRU_NETWORK_REQUEST"
                    );
                    return Err(Status::QueryBodyReadErr);
                }
            };

            let smsru_response: SmsruGetResResponse = match serde_json::from_str(&text_body) {
                Ok(parsed) => parsed,
                Err(err) => {
                    tracing::error!(
                        tech_err = ?err,
                        local_err = ?Status::SmsruGetResResponseMappigErr,
                        "SmsruGetResResponse_MAPPING_ERR"
                    );
                    return Err(Status::SmsruGetResResponseMappigErr);
                }
            };

            if smsru_response.status_code != 100 {
                tracing::error!(
                        local_err = ?Status::BackSmsRuBalance,
                        "CHECK_SMSRU_BALANCE"
                    );
                return Err(Status::BackSmsRuBalance)
            }

            if let Some(status) = smsru_response.check_status {
                match status {
                    SmsRuResponseTextCode::SuccessConfirmed => return Ok(true),  
                    SmsRuResponseTextCode::TimeOut => return Ok(false),
                    _ => {}
                }
            };

            tokio::time::sleep(*poll_intervals).await;
        }
    }).await;

    match pool_result {
        Ok(res) => res,
        Err(_) => Ok(false)
    }

}
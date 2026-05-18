use std::os::linux::raw::stat;
use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::DateTime;
use shared_lib::service::auth_service::implements::{SmsruGetResResponse, SmsRuResponseTextCode};

use crate::config::ApiState;

pub(crate) async fn smsru_get_cf(
    state: &Arc<ApiState>,
    expires_t: &DateTime,
    external_id: &str
) -> Result<bool, Status> {

    let now = chrono::Utc::now();
    let remaining = expires_t.as_ref().signed_duration_since(now);

    if remaining.num_seconds() < 0 { return Ok(false); }

    let total_duration = std::time::Duration::from_secs(remaining.num_seconds() as u64);
    let poll_intervals = &state.config.network.poll_intervals;
    let query_params = vec![
        ("api_id", state.config.smsru.api.as_str()),
        ("check_id", external_id),
        ("json", "1")
    ];

    let max_connect_retrys = state.config.network.connect_retrys;
    let mut cur_connect_retrys = 1;

    let pool_result = tokio::time::timeout(total_duration, async {
        loop {
            let res = match state
                .config
                .get_client()
                .get(&state.config.smsru.get_stat_url)
                .query(&query_params)
                .send()
                .await {
                    Ok(r) => r,
                    Err(err) => {

                        if cur_connect_retrys == 1 {
                            tracing::error!(
                                tech_err = ?err,
                                "SMSRU_NETWORK_REQUEST"
                            );
                        } else if cur_connect_retrys == max_connect_retrys {
                            tracing::error!(
                                tech_err = ?err,
                                local_err = ?Status::BackFunSmsRuCfFailed,
                                "SMSRU_NETWORK_REQUEST"
                            );
                            return Err(Status::BackFunSmsRuCfFailed);
                        }

                        cur_connect_retrys += 1;
                        tokio::time::sleep(*poll_intervals).await;
                        continue;
                    } 
                };
            
            let text_body = match res.text().await {
                Ok(t) => t,
                Err(err) => {
                    if cur_connect_retrys == 1 {
                            tracing::error!(
                                tech_err = ?err,
                                "SMSRU_NETWORK_REQUEST"
                            );
                        } else if cur_connect_retrys == max_connect_retrys {
                            tracing::error!(
                                tech_err = ?err,
                                local_err = ?Status::BackFunSmsRuCfFailed,
                                "SMSRU_NETWORK_REQUEST"
                            );
                            return Err(Status::BackFunSmsRuCfFailed);
                        }

                    cur_connect_retrys += 1;
                    tokio::time::sleep(*poll_intervals).await;
                    continue;
                }
            };

            cur_connect_retrys = 1;

            let smsru_response: SmsruGetResResponse = match serde_json::from_str(&text_body) {
                Ok(parsed) => parsed,
                Err(err) => {
                    tracing::error!(
                        tech_err = ?err,
                        local_err = ?Status::SmsruGetResResponseStructWrongMapping,
                        "SmsruGetResResponse_MAPPING_ERR"
                    );
                    return Err(Status::SmsruGetResResponseStructWrongMapping);
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
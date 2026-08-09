use crate::{Status, ClientState, ProcessError};
use crate::service::mchd::home_mchd_power::HomeMchdPower;
use crate::service::mchd::service::{MchdStep, MchdInfo};
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::back_api::post_query::post_query_back_api;

pub async fn show_powers(
    state: &ClientState,
) -> Result<MchdStep, Status> {

    let failed_result = Ok(MchdStep::TryLater { text: MchdInfo::ClientServiceError });


    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    let user_id = session.session_user.user.user_id.clone();

    let response = match post_query_back_api(
            state, 
            state.config.get_inst_client(), 
            ApiRoutes::MchdShowPowers, 
            &user_id).await {
        Ok(r) => r,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    let mchd_step: MchdStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            err.process_err(Status::MappingError, "");
            return failed_result;
        }
    };

    Ok(mchd_step)
}



pub async fn check_access(
    state: &ClientState,
    power: &HomeMchdPower
) -> Result<bool, Status> {

    let step = show_powers(state)
        .await
        .map_err(|err| err.process_err(err, ""))?;

    let powers = match step {
        MchdStep::ShowPowers { fns, btb, home, .. } => [fns, btb, home],
        _ => {
            return Err(Status::Tech.process_err(Status::SystemLogicErr, ""));
        }
    };

    for t in powers {
        if t.contains(power) {
            return Ok(true);
        }
    }

    Ok(false)
}
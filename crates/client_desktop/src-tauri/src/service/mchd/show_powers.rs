use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, MchdInfo};
use shared_lib::service::api_routes::implements::ApiRoutes;

use crate::back_api::post_query::post_query_back_api;
use crate::state::ClientState;

pub(crate) async fn show_powers(
    state: &ClientState,
) -> Result<MchdStep, Status> {

    let failed_result = MchdStep::TryLater { text: MchdInfo::ClientServiceError };


    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN lend_mchd_to_back_api_for_register FAILED BY MISS Session, err = {}", err
            );
            return Ok(failed_result);
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
            log::error!(
                "FUN show_powers FAILED BY POST QUERY TO BACK API, local_err = {:?}", err
            );
            return Ok(MchdStep::TryLater {text: MchdInfo::ClientApiQueryError});
        }
    };

    let mchd_step: MchdStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN show_powers FAILED BY MAPPING MchdStep, tech_err = {:?}, local_err = {:?}",
                err, Status::MappingError
            );
            return Ok(failed_result);
        }
    };

    Ok(mchd_step)
}
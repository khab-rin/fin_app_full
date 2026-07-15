use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdInfo, MchdStep, RegisterMchdData};
use shared_lib::service::api_routes::implements::ApiRoutes;

use crate::state::ClientState;
use crate::back_api::post_query::post_query_back_api;


pub(crate) async fn lend_mchd_to_back_api_for_register(
    state: &ClientState,
    xml_file_path: &String,
    sig_file_path: &String
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

    let xml_file = match std::fs::read(xml_file_path) {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN lend_mchd_to_back_api_for_register FAILED BY FILE READ, tech_err = {}, local_err = {}",
                err, Status::FileReadError
            );
            return Ok(failed_result);
        }
    };

    let sig_file = match std::fs::read(sig_file_path) {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN lend_mchd_to_back_api_for_register FAILED BY FILE READ, tech_err = {}, local_err = {}",
                err, Status::FileReadError
            );
            return Ok(failed_result);
        }
    };

    let user_id = session.session_user.user.user_id.clone();

    let data = RegisterMchdData {
        xml_file,
        sig_file,
        user_id
    };

    let response = match post_query_back_api(
            state, 
            state.config.get_std_client(),
            ApiRoutes::MchdLend,
            &data).await {
        Ok(r) => r, 
        Err(err) => {
            log::error!(
                "FUN register_user FAILED BY POST QUERY TO BACK API, local_err = {:?}", err
            );
            return Ok(MchdStep::TryLater {text: MchdInfo::ClientApiQueryError});
        }
    };
    
    


    Err(Status::Unknown)
}
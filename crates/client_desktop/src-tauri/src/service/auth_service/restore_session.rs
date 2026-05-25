use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;
use shared_lib::service::auth_service::implements::SessionUserToken;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::client_state::ClientState;

use crate::config::init_session;
use crate::service::auth_service::helper::get_device_id;

pub(crate) async fn restore_session(
    state: &ClientState,
    token: BoxUuid
) -> Result<Status, Status> {

    let device_id = match get_device_id() {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN restore_session FAILED BY GETTING device_id, err = {:?}", err
            );
            return Err(err);
        }
    };
    
    let payload = serde_json::json!({
        "token": token,
        "device_id": device_id
    });

    let api_url = format!(
        "{}/{}", 
        state.api_url.trim_end_matches('/'), 
        ApiRoutes::AuthRestoreToken.get_path().trim_start_matches('/'));
    
    let response = state
        .client
        .post(&api_url)
        .json(&payload)
        .send()
        .await
        .inspect_err(|err| {
            log::error!(
                "tech_err = {}, stat_err = {}",
                err, Status::QueryPostRequestErr
            )
        }).map_err(|_| Status::QueryPostRequestErr)?;
    
    if !response.status().is_success() {
        return Err(Status::BackApiError);
    }

    let user_data: SessionUserToken = response
        .json()
        .await
        .inspect_err(|err| {
            log::error!(
                "tech_err = {}, stat_err = {}",
                err, Status::MappingError
            )
        }).map_err(|_| Status::MappingError)?;
    

    init_session(state, &user_data).await

}
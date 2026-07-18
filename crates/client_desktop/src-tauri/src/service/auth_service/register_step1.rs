use shared_lib::Status;
use shared_lib::primitives::frozen::implements::Password;

use shared_lib::service::auth_service::implements::{RegInitData, AuthStep, AuthInfo};
use shared_lib::service::api_routes::implements::ApiRoutes;


use crate::state::{ClientState};
use crate::back_api::post_query::post_query_back_api;


pub(crate) async  fn register_step1(
    state: &ClientState,
    data: &RegInitData
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: AuthInfo::ClientApiSystemError };

    let blake_password = blake3::hash(data.password.as_ref().as_bytes())
        .to_hex()
        .to_string();

    let mut data_copy = data.clone();

    data_copy.password = Password::unchecked(blake_password);

    let response = match post_query_back_api(
            state, 
            state.config.get_std_client(),
            ApiRoutes::AuthRegisterStep1,
            &data_copy).await {
        Ok(r) => r, 
        Err(err) => {
            log::error!(
                "FUN register_step1 FAILED BY POST QUERY TO BACK API, local_err = {:?}", err
            );
            return Ok(failed_result);
        }
    };

    let auth_step: AuthStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN register_step1 FAILED BY MAPPING RESPONSE, err = {:?}, local_err = {:?}",
                err, Status::MappingError
            );
            return Ok(failed_result);
        }
    };

    Ok(auth_step)

}
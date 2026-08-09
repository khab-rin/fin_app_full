use crate::{Status, ClientState, ProcessError};
use crate::primitives::frozen::text::Password;

use crate::service::auth_service::implements::{RegInitData, AuthStep, AuthInfo};
use crate::service::api_routes::implements::ApiRoutes;


use crate::client::back_api::post_query::post_query_back_api;


pub
 async  fn register_step1(
    state: &ClientState,
    data: &RegInitData
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: AuthInfo::ClientApiSystemError };

    let blake_password = blake3::hash(data.password.as_ref().as_bytes())
        .to_hex()
        .to_string();

    let mut data_copy = data.clone();

    data_copy.password = Password::unchecked(blake_password);

    let response = post_query_back_api(
            state, 
            state.config.get_std_client(),
            ApiRoutes::AuthRegisterStep1,
            &data_copy)
        .await
        .map_err(|err| err.process_err(err, ""))?;
    

    let auth_step: AuthStep = response.json()
        .await
        .map_err(|err| err.process_err(Status::MappingError, ""))?;


    Ok(auth_step)

}
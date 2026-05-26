use shared_lib::Status;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::implements::{
    PasswordData,
    PasswordDataShort,
    AuthStep
};
use crate::state::ClientState;
use crate::service::auth_service::helper::get_device_id;
use crate::config::init_session;

pub(crate) async fn restore_by_password(
    state: &ClientState,
    data: &PasswordDataShort
) -> Result<AuthStep, Status> {

    let device_id = match get_device_id() {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN restore_by_password FAILED BY FUN get_device_id, err = {:?}", err
            );
            return Err(err);
        }
    };

    let password_data = PasswordData {
        password: data.password.clone(),
        device_id,
        pers_inn: data.pers_inn.clone(),
        comp_inn: data.comp_inn.clone(),
        kpp: data.kpp.clone()
    };

    let back_api_url = format!("{}/{}",
        state.api_url.trim_end_matches('/'),
        ApiRoutes::AuthRestorePassword.get_path().trim_start_matches('/')
    );

    let response = match state
        .client
        .post(&back_api_url)
        .json(&password_data)
        .send()
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!(
                    "FUN restore_by_password FAILED BY POST QUERY TO BACK API, teck_err = {:?}, local_err = {:?}",
                    err, Status::QueryPostRequestErr 
                );
                return Ok(AuthStep::TryLater {});
            }
        };
    
    if !response.status().is_success() {
        let back_err = response
            .json::<Status>()
            .await
            .unwrap_or(Status::Unknown);
        log::error!(
            "FUN restore_session_by_nik FAILED BY POST QUERY TO BACK API. Backend error code: {}, local_err = {:?}",
            back_err, Status::BackApiError
        );
        return Ok(AuthStep::TryLater {});
    }

    let auth_step:AuthStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN restore_by_password FAILED BY POST QUERY TO BACK API, err = {:?}, local_err = {:?}",
                err, Status::MappingError
            );
            return Ok(AuthStep::TryLater {});
        }
    };

    if let AuthStep::SuccessFull {session_user_token} = &auth_step {
        match init_session(state, session_user_token.as_ref()).await {
            Ok(_) => return Ok(AuthStep::SuccessShort {}),
            Err(_) => return Err(Status::SystemErr)
        }   
    }

    Ok(auth_step)
    
}
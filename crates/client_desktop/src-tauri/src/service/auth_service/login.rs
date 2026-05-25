use keyring::Entry;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;
use shared_lib::service::auth_service::implements::{ RestoreByAuthDataRequest, AuthData };
use shared_lib::service::auth_service::client_state::ClientState;

use crate::service::auth_service::restore_session::restore_session;
use crate::service::auth_service::helper::get_device_id;



pub(crate) async fn login(
    state: &ClientState,
    auth_data: AuthData
) -> Result<Status, Status> {
    
    let AuthData { pers_inn, password, comp_inn, kpp } = &auth_data;

    let token_key = format!("{}:{}:{}", pers_inn, comp_inn, kpp);
    
    let entry = Entry
        ::new(&state.app_name, &token_key)
        .inspect(|err| {
            log::error!(
                "tecn_err - {:?}, cust_err - {}",
                err, Status::SystemErr
            )
        }).map_err(|_| Status::SystemErr)?;

    if let Ok(token_str) = entry.get_password() {

        let token = BoxUuid::new(&token_str)
        .inspect_err(|_| {
            log::error!(
                "Wrong_token_in_system_for_persinn_{}_compinn_{}_kpp_{}: {}",
                pers_inn, comp_inn, kpp, Status::DataCorruptionErr
            )
        }).map_err(|_| Status::DataCorruptionErr)?;

        return restore_session(state, token).await;

    } else {
        log::info!(
            "New_user_in_system, user_inn - {}, company_inn - {}, kpp - {}",
            pers_inn, comp_inn, kpp
        );

        let device_id = get_device_id()?;
        
        
        let register_request = RestoreByAuthDataRequest {
            auth_data,
            device_id
        };

        return Ok(Status::Unknown);
    }
    
    
}
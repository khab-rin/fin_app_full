use shared_lib::Status;
use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::service::auth_service::implements::{PasswordData, SessionUserToken};

use crate::state::ClientState;
use crate::service::auth_service::helper::get_device_id;


pub async fn register_user(
    state: &ClientState,
    data: &PasswordData
) -> Result<(), Status> {
    let PasswordData { 
        password, 
        device_id, 
        pers_inn, 
        comp_inn, 
        kpp } = data;


    if password.len() < 8 { return Err(Status::ValideInput) }

    let device_id = get_device_id()?;


    Err(Status::Unknown)
}
 



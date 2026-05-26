use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;
use shared_lib::service::auth_service::client_state::{ClientState, UserLogInfo};
use shared_lib::service::auth_service::implements::AuthStep;

pub(crate) fn get_device_id() -> Result<BoxUuid, Status> {
    let id_string = machine_uid::
        get().inspect_err(|err| {
            log::error!(
                "tech_err = {}, stat_err = {}",
                err, Status::SystemErr
            )
        }).map_err(|_| Status::SystemErr)?;

    let id_uuid_str = uuid::Uuid::new_v5(
        &uuid::Uuid::NAMESPACE_DNS,
        id_string.as_bytes()
    ).to_string();

    BoxUuid::new(&id_uuid_str)
}

pub(crate) fn get_keyring_data(
    state: &ClientState,
    nik: &str
) -> Result<Option<UserLogInfo>, Status> {

    let entry = match keyring::Entry::new(&state.app_name, nik) {
        Ok(e) => e,
        Err(err) => {
            log::error!(
                "FUN get_keyring_data FAILED BY keyring::Entry::new, teck_err = {:?}, local_err = {:?}", 
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    let json_data = match entry.get_password() {
        Ok(d) => d,
        Err(keyring::Error::NoEntry) => return Ok(None),
        Err(err) => {
            log::error!(
                "FUN get_keyring_data FAILED BY entry.get_password, teck_err = {:?}, local_err = {:?}", 
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    let user_log_data: UserLogInfo = match serde_json::from_str(&json_data) {
        Ok(d) => d,
        Err(err) => {
            log::error!(
                "FUN get_keyring_data FAILED BY mapping UserLogInfo, teck_err = {:?}, local_err = {:?}", 
                err, Status::MappingError
            );
            return Err(Status::SystemErr);
        }
    };

    Ok(Some(user_log_data))
} 
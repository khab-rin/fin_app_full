use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;
use shared_lib::primitives::frozen::implements_base::String1_50;
use shared_lib::service::auth_service::client_state::UserLogInfo;

use crate::state::ClientState;
use crate::service::auth_service::nick_data::add_nickname;

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
    nick: &str
) -> Result<Option<UserLogInfo>, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let entry = match keyring::Entry::new(app_name, nick) {
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


pub(crate) fn write_log_info(
    state: &ClientState,
    nick: &String1_50,
    log_info: &UserLogInfo
) -> Result<Status, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let log_info_json = match serde_json::to_string(log_info) {
        Ok(l) => l,
        Err(err) => {
            log::error!(
                "FUN write_log_info FAILED BY MAPPING UserLogInfo, tech_err = {}, local_err = {}",
                err, Status::MappingError
            );
            return Err(Status::MappingError);
        }
    };

    let entry = match keyring::Entry::new(app_name, nick) {
        Ok(e) => e,
        Err(err) => {
            log::error!(
                "FUN write_log_info failed BY keyring::Entry::new, tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    match entry.set_password(&log_info_json) {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "FUN write_log_info failed BY keyring::entry.set_password, tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    }

    match add_nickname(state, nick) {
        Ok(_) => Ok(Status::Success),
        Err(err) => {
            log::error!(
                "FUN write_log_info failed BY FUN add_nickname, tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            Err(Status::SystemErr)
        } 
    }

    


}
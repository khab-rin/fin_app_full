use shared_lib::Status;
use shared_lib::primitives::frozen::implements_base::String1_50;
use shared_lib::service::auth_service::client_state::UserLogInfo;

use crate::state::ClientState;
use crate::service::auth_service::nick_data::get_nick_data_by_nick;

pub(crate) fn get_keyring_data(
    state: &ClientState,
    key_: &String
) -> Result<UserLogInfo, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let entry = match keyring::Entry::new(app_name, key_) {
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
        Err(keyring::Error::NoEntry) => return Ok(UserLogInfo::default()),
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
                "FUN get_keyring_data FAILED BY mapping UserLogInfo, tech_err = {:?}, local_err = {:?}", 
                err, Status::MappingError
            );
            return Err(Status::SystemErr);
        }
    };

    Ok(user_log_data)
} 


pub(crate) fn write_keyring_data (
    state: &ClientState,
    key_: &String,
    log_info: &UserLogInfo
) -> Result<bool, Status> {

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

    let entry = match keyring::Entry::new(app_name, key_) {
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
        Ok(_) => Ok(true),
        Err(err) => {
            log::error!(
                "FUN write_log_info failed BY keyring::entry.set_password, tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            Err(Status::SystemErr)
        }
    }

}


pub(crate) fn delete_keyring_data(
    state: &ClientState,
    key_: &String
) -> Result<bool, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let entry = match keyring::Entry::new(app_name, key_) {
        Ok(e) => e,
        Err(err) => {
            log::error!(
                "FUN write_log_info failed BY keyring::Entry::new, tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    match entry.delete_credential() {
        Ok(_) => Ok(true),
        Err(keyring::Error::NoEntry) => Ok(false),
        Err(err) => {
            log::error!(
                "FUN delete_keyring_data FAILED BY entry.delete_credential(), tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            Err(Status::SystemErr)
        }
    }

}
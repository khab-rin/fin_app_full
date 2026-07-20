use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;

use crate::state::ClientState;

pub(crate) fn get_keyring_token(
    state: &ClientState,
    key_: &str
) -> Result<Option<BoxUuid>, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let entry = match keyring::Entry::new(app_name, key_) {
        Ok(e) => e,
        Err(err) => {
            log::error!(
                "FUN get_keyring_token FAILED BY keyring::Entry::new, teck_err = {:?}, local_err = {:?}", 
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    let token_str = match entry.get_password() {
        Ok(d) => d,
        Err(keyring::Error::NoEntry) => return Ok(None),
        Err(err) => {
            log::error!(
                "FUN get_keyring_token FAILED BY entry.get_password, teck_err = {:?}, local_err = {:?}", 
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    let token = match BoxUuid::new(&token_str) {
        Ok(t) => t,
        Err(err) => {
            log::error!(
                "FUN get_keyring_token FAILED BY BoxUuid::new(&token_str), local_err = {:?}",
                err
            );
            return Err(Status::SystemLogicErr);
        }
    };

    Ok(Some(token))
} 


pub(crate) fn write_keyring_token (
    state: &ClientState,
    key_: &str,
    token: &BoxUuid
) -> Result<(), Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let token_string = token.to_string();

    let entry = match keyring::Entry::new(app_name, key_) {
        Ok(e) => e,
        Err(err) => {
            log::error!(
                "FUN write_keyring_data failed BY keyring::Entry::new, tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    match entry.set_password(&token_string) {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!(
                "FUN write_keyring_data failed BY keyring::entry.set_password, tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            Err(Status::SystemErr)
        }
    }

}


pub(crate) fn delete_keyring_token(
    state: &ClientState,
    key_: &str
) -> Result<bool, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let entry = match keyring::Entry::new(app_name, key_) {
        Ok(e) => e,
        Err(err) => {
            log::error!(
                "FUN delete_keyring_data failed BY keyring::Entry::new, tech_err = {}, local_err = {}",
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
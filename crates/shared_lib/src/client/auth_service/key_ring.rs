use crate::{Status, ProcessError};
use crate::ClientState;
use crate::primitives::frozen::text::BoxUuid;



pub fn get_keyring_token(
    state: &ClientState,
    key_: &str
) -> Result<Option<BoxUuid>, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let entry = keyring::Entry::new(app_name, key_)
        .map_err(|err| err.process_err(Status::SystemErr, ""))?; 

    let token_str = match entry.get_password() {
        Ok(d) => d,
        Err(keyring::Error::NoEntry) => return Ok(None),
        Err(err) => {
            return Err(err.process_err(Status::SystemErr, ""));
        }
    };

    let token = BoxUuid::new(&token_str)
        .map_err(|err| err.process_err(Status::SystemLogicErr, ""))?;

    Ok(Some(token))
} 


pub fn write_keyring_token (
    state: &ClientState,
    key_: &str,
    token: &BoxUuid
) -> Result<(), Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let token_string = token.to_string();

    let entry = keyring::Entry::new(app_name, key_)
        .map_err(|err| err.process_err(Status::SystemLogicErr, ""))?; 

    entry
        .set_password(&token_string)
        .map_err(|err| err.process_err(Status::SystemLogicErr, ""))?;

    Ok(())

}


pub fn delete_keyring_token(
    state: &ClientState,
    key_: &str
) -> Result<bool, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let entry = keyring::Entry::new(app_name, key_)
        .map_err(|err| err.process_err(Status::SystemLogicErr, ""))?; 

    match entry.delete_credential() {
        Ok(_) => Ok(true),
        Err(keyring::Error::NoEntry) => Ok(false),
        Err(err) => {
            Err(err.process_err(Status::SystemErr, ""))
        }
    }
}
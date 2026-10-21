use crate::{Status, ProcessError};
use crate::ClientState;
use crate::primitives::frozen::text::BoxUuid;


pub fn get_keyring_token(
    state: &ClientState,
    key_: &str,
) -> Result<Option<BoxUuid>, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

	let entry = new_entry(app_name, key_).map_err(|err| err.process_err(err, ""))?;

	let token_str = entry.get_password().map_err(|err| err.process_err(Status::KeyRingError, ""))?;

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

	let ext_info = format!("key_ = {}, app_name = {}", key_, app_name);

    let entry = new_entry(app_name, key_).map_err(|err| err.process_err(err, ""))?;

    entry
        .set_password(&token_string)
        .map_err(|err| err.process_err(Status::SystemLogicErr, &ext_info))?;

    Ok(())

}


pub fn delete_keyring_token(
    state: &ClientState,
    key_: &str
) -> Result<bool, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

    let entry = new_entry(app_name, key_).map_err(|err| err.process_err(err, ""))?;

    match entry.delete_credential() {
        Ok(_) => Ok(true),
        Err(keyring::Error::NoEntry) => Ok(false),
        Err(err) => {
            Err(err.process_err(Status::SystemErr, ""))
        }
    }
}

#[cfg(target_os = "android")]
pub fn new_entry(
	app_name: &str,
	key_: &str
) -> Result<keyring_core::Entry, Status> {
	keyring_core::Entry::new(app_name, key_).map_err(|err| err.process_err(Status::KeyRingError, ""))
}



#[cfg(not(target_os = "android"))]
pub fn new_entry(
	app_name: &str,
	key_: &str
) -> Result<keyring::Entry, Status> {
	keyring::Entry::new(app_name, key_).map_err(|err| err.process_err(Status::KeyRingError, ""))
}


#[cfg(target_os = "android")]
fn init_android_keyring() -> Result<(), Status> {
    use android_native_keyring_store::Store;
    use keyring_core::set_default_store;

	let store = Store::new().map_err(|err| err.process_err(Status::KeyRingError, ""))?;

    set_default_store(store);

    let store_registered = keyring_core::get_default_store().is_some();

	if store_registered {
		Ok(())
	} else {
		Err(Status::KeyRingError)
	}
}
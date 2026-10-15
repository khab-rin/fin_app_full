use crate::{Status, ProcessError};
use crate::ClientState;
use crate::primitives::frozen::text::BoxUuid;



pub fn get_keyring_token(
    state: &ClientState,
    key_: &str
) -> Result<Option<BoxUuid>, Status> {

    let app_name = state.app_handle.package_info().name.as_str();

	log::debug!("KEYRING READ: app_name={:?}, key_={:?}", app_name, key_);

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

	log::debug!(
		"KEYRING WRITE: entry={:?}",
		entry
	);

    Ok(Some(token))
} 


pub fn write_keyring_token (
    state: &ClientState,
    key_: &str,
    token: &BoxUuid
) -> Result<(), Status> {

    let app_name = state.app_handle.package_info().name.as_str();

	log::debug!("KEYRING WRITE: app_name={:?}, key_={:?}",app_name,key_);

    let token_string = token.to_string();

	let ext_info = format!("key_ = {}, app_name = {}", key_, app_name);

    let entry = keyring::Entry::new(app_name, key_)
        .map_err(|err| err.process_err(Status::SystemLogicErr, &ext_info))?; 

    entry
        .set_password(&token_string)
        .map_err(|err| err.process_err(Status::SystemLogicErr, &ext_info))?;

	log::debug!("KEYRING WRITE: entry={:?}",entry);

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


pub fn debug_keyring_backends() {
    println!("=========================================");
    println!("🔍 ТЕХНИЧЕСКАЯ ОТЛАДКА KEYRING:");
    

    if let Ok(entry) = keyring::Entry::new("test_app_debug", "test_key_debug") {

        if let Err(err) = entry.get_password() {
            println!("▶ Внутренний тип ошибки (Debug): {:?}", err);
            println!("▶ Описание ошибки (Display): {}", err);
        } else {
            println!("▶ Тестовое хранилище доступно и работает!");
        }
    } else {
        println!("❌ Не удалось даже инициализировать структуру Entry");
    }
    
    println!("=========================================");
}
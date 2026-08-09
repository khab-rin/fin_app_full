use std::collections::HashSet;
use std::path::PathBuf;

use chrono::Local;
use shared_lib::{Status, ProcessError};

use shared_lib::service::mchd::poa::PoaMchd;
use shared_lib::service::mchd::service::{MchdStorage};
use shared_lib::service::mchd::implements::PoaRootKind;
use shared_lib::primitives::frozen::text::{BoxUuid, Date};





pub(crate) fn get_mchd_data_path() -> Result<PathBuf, Status> {
    if cfg!(debug_assertions) {
        let base_dir = match std::env::var("CARGO_MANIFEST_DIR")
                .map(PathBuf::from) {
            Ok(p) => p,
            Err(_) => {std::env::current_dir().unwrap_or_default()}
        };

        let crate_name = "CARGO_PKG_NAME";

        let mut workspace_root = base_dir;

        if workspace_root.ends_with(crate_name) {
            workspace_root.pop();
        }

        if workspace_root.ends_with("crates") {
            workspace_root.pop();
        }

        Ok(workspace_root.join("mock_storage").join("mchd"))
    } else {

        let crate_name = env!("CARGO_PKG_NAME");

        match home::home_dir() {
            Some(home) => {
                Ok(home.join(format!(".{}", crate_name)).join("mchd"))
            }
            None => {
                Err(Status::Tech.process_err(Status::DirCreateError, ""))
            }
        }
    }
}




pub(crate) async fn get_mchd_storage() -> Result<MchdStorage, Status> {

    let path = get_mchd_data_path()
        .map_err(|err| err.process_err(err, ""))?;

    let file_path = path.join("storage.json");

    if !file_path.exists() {
        return Ok(MchdStorage { 
            storage: std::collections::HashMap::new(), 
            managers: HashSet::new()});
    }

    let json_content = std::fs::read_to_string(&file_path)
        .map_err(|err| err.process_err(Status::FileReadError, ""))?;



    serde_json::from_str(&json_content)
        .map_err(|err| err.process_err(Status::MappingError, ""))

}


pub(crate) async fn write_mchd_storage_to_file(
    storage: MchdStorage
) -> Result<(), Status> {
    let path = get_mchd_data_path()
        .map_err(|err| err.process_err(err, ""))?;
 

    if let Err(err) = std::fs::create_dir_all(&path) {
        return Err(err.process_err(Status::DirCreateError, ""));
    }

    let file_path = path.join("storage.json");

    let json_content = serde_json::to_string_pretty(&storage)
        .map_err(|err| err.process_err(Status::MappingError, ""))?;
    
    if let Err(err) = std::fs::write(&file_path, &json_content) {
        return Err(err.process_err(Status::FileWriteError, ""));
    }

    Ok(())
}


pub(crate) fn insert_poa(
    new_mchd: PoaMchd,
    storage: MchdStorage
) -> Result<MchdStorage, Status> {

    let identificator: Vec<char> = new_mchd.flie_identificator.to_string().chars().collect();

    let guide_str: String = if identificator.len() > 36 {
        identificator[identificator.len() - 36..].iter().collect()
    } else {
        return Err(Status::Tech.process_err(Status::DataCorruptionErr, ""));
    };

    let guide_uuid = uuid::Uuid::parse_str(&guide_str)
        .map_err(|err| err.process_err(Status::SystemLogicErr, ""))?;


    let guide = BoxUuid::unchecked(guide_uuid);

    let mut new_storage = MchdStorage {
        storage: std::collections::HashMap::new(),
        managers: storage.managers
    };

    new_storage.storage.insert(guide, new_mchd);

    let local_now = Local::now();
    let today = Date::unchecked(local_now.date_naive());

    for (g, poa) in storage.storage.into_iter() {
        let poa_end_date = match &poa.poa.poa_doc {
            PoaRootKind::RootPoa(boxed_root) => boxed_root.poa_metadata.life_date.clone(),
            _ => { 
                return Err(Status::Tech.process_err(Status::SystemLogicErr, "")); 
            }
        };

        if today > poa_end_date { 
            continue; 
        }

        new_storage.storage.insert(g, poa);
    }


    Ok(new_storage)
}


pub(crate) async fn add_new_manager(
    user_id: &BoxUuid,
) -> Result<(), Status> {
    let mut storage = get_mchd_storage()
        .await
        .map_err(|err| err.process_err(err, ""))?;
    
    storage.managers.insert(user_id.clone());

    write_mchd_storage_to_file(storage)
        .await
        .map_err(|err| err.process_err(err,""))?;

    Ok(())
}

pub(crate) async fn add_new_poa(
    poa: PoaMchd
) -> Result<(), Status> {

    let storage = get_mchd_storage()
        .await
        .map_err(|err| err.process_err(err, ""))?;

    let storage = insert_poa(poa, storage)
        .map_err(|err| err.process_err(err, ""))?;

    write_mchd_storage_to_file(storage).await
        .map_err(|err| err.process_err(err, ""))?;

    
    Ok(())
}




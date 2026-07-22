use std::path::PathBuf;

use chrono::Local;
use shared_lib::Status;

use shared_lib::service::mchd::implements::RootPoa;
use shared_lib::service::mchd::poa::PoaMchd;
use shared_lib::service::mchd::service::MchdStorage;
use shared_lib::service::mchd::implements::PoaRootKind;
use shared_lib::primitives::frozen::implements::{BoxUuid, Date};




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
                tracing::error!(
                    local_err = ?Status::DirCreateError,
                    "FUN get_mchd_data_path FAILED: Не удалось определить домашнюю директорию пользователя"
                );
                Err(Status::DirCreateError)
            }
        }
    }
}




pub(crate) fn get_mchd_storage() -> Result<MchdStorage, Status> {

    let path = match get_mchd_data_path() {
        Ok(p) => p,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN get_mchd_data BY FUN get_mchd_data_path"
            );
            return Err(err);
        }
    };

    let file_path = path.join("storage.json");

    if !file_path.exists() {
        return Ok(MchdStorage { storage: std::collections::HashMap::new() });
    }

    let json_content = match std::fs::read_to_string(&file_path) {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::FileReadError,
                "FUN get_mchd_data BY FUN std::fs::read_to_string"
            );
            return Err(Status::FileReadError);
        }
    };


    match serde_json::from_str(&json_content) {
        Ok(s) => Ok(s),
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN get_mchd_data BY FUN std::fs::read_to_string"
            );
            Err(Status::MappingError)
        }
    }


}


pub(crate) fn write_mchd_storage_to_file(
    storage: MchdStorage
) -> Result<(), Status> {
    let path = match get_mchd_data_path() {
        Ok(p) => p,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN write_mchd_storage_to_file BY FUN get_mchd_data_path"
            );
            return Err(err);
        }
    };

    if let Err(err) = std::fs::create_dir_all(&path) {
        tracing::error!(
            tech_err = ?err,
            local_err = ?&Status::DirCreateError,
            path = ?path,
            "FUN write_mchd_storage_to_file FAILED: Не удалось создать директорию"
        );
        return Err(Status::DirCreateError);
    }

    let file_path = path.join("storage.json");

    let json_content = match serde_json::to_string_pretty(&storage) {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN write_mchd_storage_to_file FAILED: Ошибка сериализации MchdStorage в JSON"
            );
            return Err(Status::MappingError);
        }
    };

    if let Err(err) = std::fs::write(&file_path, &json_content) {
        tracing::error!(
            tech_err = ?err,
            local_err = ?Status::FileReadError,
            path = ?file_path,
            "FUN write_mchd_storage_to_file FAILED: Не удалось записать файл на диск"
        );
        return Err(Status::FileWriteError);
    }

    Ok(())
}


pub(crate) fn add_new_mchd(
    new_mchd: PoaMchd,
    storage: MchdStorage
) -> Result<MchdStorage, Status> {

    let identificator: Vec<char> = new_mchd.flie_identificator.to_string().chars().collect();

    let guide_str: String = if identificator.len() > 36 {
        identificator[identificator.len() - 36..].iter().collect()
    } else {
        tracing::error!(
            local_err = ?Status::DataCorruptionErr,
            "FUN register_mchd FAILED BY if identificator.len() > 36"
        );
        return Err(Status::DataCorruptionErr);
    };

    let guide_uuid = match uuid::Uuid::parse_str(&guide_str) {
        Ok(g) => g,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::SystemLogicErr,
                "FUN register_mchd FAILED BY uuid::Uuid::parse_str(&guide_str)"
            );
            return Err(Status::SystemLogicErr);
        }
    };

    let guide = BoxUuid::unchecked(guide_uuid);

    let mut new_storage = MchdStorage {
        storage: std::collections::HashMap::new()
    };

    new_storage.storage.insert(guide, new_mchd);

    let local_now = Local::now();
    let today = Date::unchecked(local_now.date_naive());

    for (g, poa) in storage.storage.into_iter() {
        let poa_end_date = match &poa.poa.poa_doc {
            PoaRootKind::RootPoa(boxed_root) => boxed_root.poa_metadata.life_date.clone(),
            _ => { 
                return Err(Status::Unknown); 
            }
        };

        if today > poa_end_date { 
            continue; 
        }

        new_storage.storage.insert(g, poa);
    }


    Ok(new_storage)
}
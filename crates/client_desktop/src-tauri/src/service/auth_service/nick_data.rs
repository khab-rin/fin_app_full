use std::io::{Read, Write, Seek, SeekFrom};

use serde::{Deserialize, Serialize};
use tauri::Manager;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements_base::String1_50;
use shared_lib::service::auth_service::client_state::NickData;

use crate::state::ClientState;



pub(crate) fn add_nickname(
    state: &ClientState,
    nick_name: String1_50
) -> Result<bool, Status> {
    
    let app_handle = state.app_handle.clone();

    let app_path = match app_handle.path().app_data_dir() {
        Ok(p) => p,
        Err(err) => {
            log::error!(
                "FUN add_nickname FAILED BY app_handle.path().app_data_dir(), tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    match std::fs::create_dir_all(&app_path) {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "FUN add_nickname FAILED TO CREATE DIRECTORY: {}, tech_err = {}, local_err = {}",
                app_path.display(), err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    }

    let file_path = app_path.join("nick_names.json");

    let mut file = match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&file_path) {
        Ok(f) => f,
        Err(err) => {
            log::error!(
                "FUN add_nickname FAILED BY FILE CREATE: {}, tech_err = {}, local_err = {}",
                app_path.display(), err, Status::FileReadError
            );
            return Err(Status::SystemErr);
        }
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(err) => {
            log::error!(
                "FUN add_nickname FAILED BY FILE READ, tech_err = {}, local_err = {}",
                err, Status::FileReadError
            );
        }
    }

    let mut data = match contents.is_empty() {
        true => NickData::default(),
        false => {
            match serde_json::from_str(&contents) {
                Ok(d) => d,
                Err(err) => {
                    log::error!(
                        "FUN add_nickname FAILED BY MAPPING NickData, tech_err = {}, local_err = {}",
                        err, Status::MappingError
                    );
                    return Err(Status::MappingError);
                }
            }
        }
    };
    
    data.nick_names.push(nick_name);

    let nick_names_set: std::collections::HashSet<String1_50> = data.nick_names.into_iter().collect();

    let data = NickData {
        nick_names: nick_names_set.into_iter().collect()
    };


    let json_bytes = match file.set_len(0).is_ok() && file.seek(SeekFrom::Start(0)).is_ok() {
        true => match serde_json::to_vec_pretty(&data) {
            Ok(b) => b,
            Err(err) => {
                log::error!(
                    "FUN add_nickname FAILED BY MAPPING NickData, tech_err = {}, local_err = {}",
                    err, Status::MappingError
                );
                return Err(Status::MappingError);
            }
        },
        false => {
            return Err(Status::SystemErr);
        }
    };

    match file.write_all(&json_bytes) {
        Ok(_) => Ok(true),
        Err(err) => {
            log::error!(
                "FUN add_nickname FAILED BY MAPPING NickData, tech_err = {}, local_err = {}",
                err, Status::FileWriteError
            );
        Err(Status::FileWriteError)
        }
    }

}


pub(crate) fn get_nicknames(
    state: &ClientState,
) -> Result<NickData, Status> {
    let app_handle = state.app_handle.clone();

    let app_path = match app_handle.path().app_data_dir() {
        Ok(p) => p,
        Err(err) => {
            log::error!(
                "FUN get_nicknames FAILED BY app_handle.path().app_data_dir(), tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    match std::fs::create_dir_all(&app_path) {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "FUN get_nicknames FAILED TO CREATE DIRECTORY: {}, tech_err = {}, local_err = {}",
                app_path.display(), err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    }

    let file_path = app_path.join("nick_names.json");

    let mut file = match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&file_path) {
        Ok(f) => f,
        Err(err) => {
            log::error!(
                "FUN get_nicknames FAILED TO CREATE DIRECTORY: {}, tech_err = {}, local_err = {}",
                app_path.display(), err, Status::FileReadError
            );
            return Err(Status::SystemErr);
        }
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(err) => {
            log::error!(
                "FUN get_nicknames FAILED BY FILE READ, tech_err = {}, local_err = {}",
                err, Status::FileReadError
            );
        }
    }

    let mut data = match contents.is_empty() {
        true => NickData::default(),
        false => {
            match serde_json::from_str(&contents) {
                Ok(d) => d,
                Err(err) => {
                    log::error!(
                        "FUN get_nicknames FAILED BY MAPPING NickData, tech_err = {}, local_err = {}",
                        err, Status::MappingError
                    );
                    return Err(Status::MappingError);
                }
            }
        }
    };

    Ok(data)
}

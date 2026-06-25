use std::io::{Read};

use tauri::Manager;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements_base::String1_50;
use shared_lib::service::auth_service::client_state::NickData;

use crate::state::ClientState;



pub(crate) fn add_nick_data(
    state: &ClientState,
    nick_data: &NickData
) -> Result<NickData, Status> {
    
    let file_path = match get_nick_data_path(state) {
        Ok(f) => f,
        Err(err) => {
            log::error!(
                "FUN delete_nick_name FAILED BY FUN get_nick_data_path, local_err = {}", err
            );
            return Err(Status::SystemErr);
        }
    };

    let nick_datas = match get_nick_datas_from_file_path(&file_path) {
        Ok(n) => n,
        Err(err) => {
            log::error!(
                "FUN add_nickname FAILED BY FUN get_nick_data, local_err = {}", err
            );
            return Err(err);
        }
    };

    let mut new_nick_datas: std::collections::HashSet<NickData> = std::collections::HashSet::new();

    for prev_nick_data in nick_datas {
        if 
            prev_nick_data.pers_inn == nick_data.pers_inn &&
            prev_nick_data.comp_inn == nick_data.comp_inn &&
            prev_nick_data.kpp == nick_data.kpp {
                continue;
        } else {
            new_nick_datas.insert(prev_nick_data);
        }
    }

    new_nick_datas.insert(nick_data.clone());


    match save_nick_datas(&file_path, &new_nick_datas) {
        Ok(_) => Ok(nick_data.clone()),
        Err(err) => {
            log::error!(
                "FUN add_nickname FAILED BY FUN save_nick_data, local_err = {}", err
            );
            Err(err)
        }
    }

}


pub(crate) fn get_nick_names(
    state: &ClientState,
) -> Result<Vec<String1_50>, Status> {

    let file_path = match get_nick_data_path(state) {
        Ok(f) => f,
        Err(err) => {
            log::error!(
                "FUN delete_nick_name FAILED BY FUN get_nick_data_path, local_err = {}", err
            );
            return Err(Status::SystemErr);
        }
    };

    let nick_datas = match get_nick_datas_from_file_path(&file_path) {
        Ok(n) => n,
        Err(err) => {
            log::error!(
                "FUN get_nick_data FAILED BY FUN get_nick_data_from_file_path, err = {}", err
            );
            return Err(err);
        }
    };

    let res: Vec<String1_50> = nick_datas.into_iter().map(|x| x.nick).collect();

    Ok(res)
}


pub(crate) fn get_nick_data_by_nick(
    state: &ClientState,
    nick: &String1_50
) -> Result<Option<NickData>, Status> {

    let file_path = match get_nick_data_path(state) {
        Ok(f) => f,
        Err(err) => {
            log::error!(
                "FUN delete_nick_name FAILED BY FUN get_nick_data_path, local_err = {}", err
            );
            return Err(Status::SystemErr);
        }
    };

    let nick_datas = match get_nick_datas_from_file_path(&file_path) {
        Ok(n) => n,
        Err(err) => {
            log::error!(
                "FUN get_nick_data FAILED BY FUN get_nick_data_from_file_path, err = {}", err
            );
            return Err(err);
        }
    };

    for nick_data in nick_datas {
        if nick_data.nick == nick {
            return Ok(Some(nick_data));
        }
    }

    Ok(None)

}



pub(crate) fn get_nick_data_path(
    state: &ClientState
) -> Result<std::path::PathBuf, Status> {
    let app_handle = state.app_handle.clone();

    let app_path = match app_handle.path().app_data_dir() {
        Ok(p) => p,
        Err(err) => {
            log::error!(
                "FUN get_nick_data_path FAILED BY app_handle.path().app_data_dir(), tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    match std::fs::create_dir_all(&app_path) {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "FUN get_nick_data_path FAILED TO CREATE DIRECTORY: {}, tech_err = {}, local_err = {}",
                app_path.display(), err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    }

    let file_path = app_path.join("nick_names.json");

    Ok(file_path)
}



pub(crate) fn get_nick_datas_from_file_path(
    file_path: &std::path::PathBuf
) -> Result<std::collections::HashSet<NickData>, Status> {

    let mut file = match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(file_path) {
        Ok(f) => f,
        Err(err) => {
            log::error!(
            "FUN get_nick_data_from_file_path FAILED BY FILE OPEN, tech_err = {}, local_err = {}",
            err, Status::FileReadError
        );
        return Err(Status::SystemErr);
        }
    };

    let mut content = String::new();

    match file.read_to_string(& mut content) {
        Ok(_) => {}
        Err(err) => {
            log::error!(
                "FUN get_nick_data_from_file_path FAILED BY FILE READ, tech_err = {}, local_err = {}",
                err, Status::FileReadError
            );
        }
    }

    let nick_datas: std::collections::HashSet<NickData> = match content.is_empty() {
        true => std::collections::HashSet::new(),
        false => {
            match serde_json::from_str(&content) {
                Ok(n) => n,
                Err(err) => {
                    log::error!(
                        "FUN get_nick_data_from_file_path FAILED BY MAPPING NickData, tech_err = {}, local_err = {}",
                        err, Status::MappingError
                    );
                    return Err(Status::MappingError);
                }
            }
        }
    };

    Ok(nick_datas)
}


pub(crate) fn save_nick_datas(
    file_path: &std::path::PathBuf,
    nick_datas: &std::collections::HashSet<NickData>,
) -> Result<(), Status> {

    let content = match serde_json::to_string_pretty(nick_datas) {
        Ok(c) => c,
        Err(err) => {
            log::error!(
                "FUN save_nick_data FAILED BY SERDE SERIALIZATION, tech_err = {}, local_err = {}",
                err, Status::SerializationError
            );
            return Err(Status::SerializationError);
        }
    };

    let mut file = match std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .read(false)
            .open(file_path) {
        Ok(f) => f,
        Err(err) => {
            log::error!(
            "FUN get_nick_data FAILED BY FILE OPEN, tech_err = {}, local_err = {}",
            err, Status::FileReadError
        );
        return Err(Status::SystemErr);
        }
    };

    match std::io::Write::write_all(&mut file, content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!(
                "FUN save_nick_data FAILED BY std::io::Write::write_all, tech_err = {}, local_err = {}",
                err, Status::FileWriteError
            );
            return Err(Status::SystemErr);
        }
    }

}
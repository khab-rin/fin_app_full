use std::io::{Read};
use tauri::Manager;

use crate::{Status, ClientState, ProcessError};
use crate::service::auth_service::client_state::NickData;


pub fn add_nick_data(
    state: &ClientState,
    nick_data: &NickData
) -> Result<NickData, Status> {
    
    let file_path = get_nick_data_path(state)
        .map_err(|err| err.process_err(err, ""))?; 

    let nick_datas = get_nick_datas_from_file_path(&file_path)
        .map_err(|err| err.process_err(err, ""))?;

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


    if let Err(err) =  save_nick_datas(&file_path, &new_nick_datas) {
        return Err(err.process_err(err, ""));
    }

    Ok(nick_data.clone())

}


pub fn get_nick_names(
    state: &ClientState,
) -> Result<Vec<String>, Status> {

    let file_path = get_nick_data_path(state)
        .map_err(|err| err.process_err(err, ""))?;


    let nick_datas = get_nick_datas_from_file_path(&file_path)
        .map_err(|err| err.process_err(err, ""))?;

    let res: Vec<String> = nick_datas.into_iter().map(|x| x.nick).collect();

    Ok(res)
}


pub fn get_nick_data_by_nick(
    state: &ClientState,
    nick: &str
) -> Result<Option<NickData>, Status> {

    let file_path = get_nick_data_path(state)
        .map_err(|err| err.process_err(err, ""))?;

    let nick_datas = get_nick_datas_from_file_path(&file_path)
        .map_err(|err| err.process_err(err, ""))?;

    for nick_data in nick_datas {
        if nick_data.nick == nick {
            return Ok(Some(nick_data));
        }
    }

    Ok(None)

}



pub fn get_nick_data_path(
    state: &ClientState
) -> Result<std::path::PathBuf, Status> {
    let app_handle = state.app_handle.clone();

    let app_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|err| err.process_err(Status::SystemErr, ""))?;

    std::fs::create_dir_all(&app_path)
        .map_err(|err| err.process_err(Status::SystemErr, ""))?;


    let file_path = app_path.join("nick_names.json");

    Ok(file_path)
}



pub fn get_nick_datas_from_file_path(
    file_path: &std::path::PathBuf
) -> Result<std::collections::HashSet<NickData>, Status> {

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(file_path)
        .map_err(|err| err.process_err(Status::SystemErr, ""))?;

    let mut content = String::new();

    file
        .read_to_string(& mut content)
        .map_err(|err| err.process_err(Status::FileReadError, ""))?;


    let nick_datas: std::collections::HashSet<NickData> = match content.is_empty() {
        true => std::collections::HashSet::new(),
        false => {
            serde_json::from_str(&content)
                .map_err(|err| err.process_err(Status::MappingError, ""))? 
        }
    };

    Ok(nick_datas)
}


pub fn save_nick_datas(
    file_path: &std::path::PathBuf,
    nick_datas: &std::collections::HashSet<NickData>,
) -> Result<(), Status> {

    let content = serde_json::to_string_pretty(nick_datas)
        .map_err(|err| err.process_err(Status::SerializationError, ""))?; 


    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .read(false)
        .open(file_path)
        .map_err(|err| err.process_err(Status::FileReadError, ""))?;
  

    std::io::Write::write_all(&mut file, content.as_bytes())
        .map_err(|err| err.process_err(Status::FileWriteError, "")) 

}
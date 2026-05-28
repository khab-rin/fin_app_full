use serde::{Deserialize, Serialize};
use tauri::Manager;

use shared_lib::Status;
use shared_lib::primitives::frozen::implplemets_base::String1_50;

use crate::state::ClientState;

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct NikData{
    nick_names: Vec<String1_50>
}

pub(crate) fn add_nikname(
    state: &ClientState,
    nik_name: String1_50
) -> Result<bool, Status> {
    
    let app_handle = state.app_handle.clone();

    let app_path = match app_handle.path().app_data_dir() {
        Ok(p) => p,
        Err(err) => {
            log::error!(
                "FUN add_nikname FAILED BY app_handle.path().app_data_dir(), tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            return Ok(false);
        }
    };

    Err(Status::Unknown)
}
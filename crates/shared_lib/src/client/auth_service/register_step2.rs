use std::str::FromStr;

use tauri_plugin_fs::FsExt;

use crate::service::auth_service::implements::{
    AuthStep, AuthInfo, RegFilesPathData
};
use crate::service::cryp_serv::implements::CheckSignDocData;
use crate::service::api_routes::implements::ApiRoutes;
use crate::{Status, ClientState, ProcessError};
use crate::service::auth_service::client_state::init_session;
use crate::client::back_api::post_query::post_query_back_api;
use crate::client::auth_service::helper::write_new_user_info_to_device;


pub async fn register_step2(
    state: &ClientState,
    data: &RegFilesPathData
) -> Result<AuthStep, Status> {

    let failed_result = AuthStep::TryLater { text: AuthInfo::ClientApiSystemError };

    let RegFilesPathData { json_path, sign_path } = data;

	let tauri_json_path = tauri_plugin_fs::FilePath::from_str(json_path)
		.map_err(|err| {
			let ext_info = format!("wrong svelte json_path = {:?}", json_path);
			err.process_err(Status::FilePathError, &ext_info)
		})?;

	let init_file = state.app_handle.fs().read(tauri_json_path.to_owned())
		.map_err(|err| {
			let ext_info = format!("wrong tauri_json_path = {:?}", tauri_json_path);
			err.process_err(Status::FileReadError, &ext_info)
		})?;


	let tauri_sign_path = tauri_plugin_fs::FilePath::from_str(sign_path)
		.map_err(|err| {
			let ext_info = format!("wrong svelte sign_path = {:?}", sign_path);
			err.process_err(Status::FilePathError, &ext_info)
		})?;

	let sign_file = state.app_handle.fs().read(tauri_sign_path.to_owned())
		.map_err(|err| {
			let ext_info = format!("wrong tauri_sign_path = {:?}", tauri_sign_path);
			err.process_err(Status::FileReadError, &ext_info)
		})?;

	
    let reg_files_data = CheckSignDocData {
        init_file, sign_file
    };


    let response = match post_query_back_api(
            state,
            state.config.get_std_client(),
            ApiRoutes::AuthRegisterStep2,
            &reg_files_data
    ).await {
        Ok(r) => r,
        Err(err) => {
            err.process_err(err, "");
            return Ok(failed_result);
        }
    };

    let auth_step: AuthStep = match response.json().await {
        Ok(s) => s,
        Err(err) => {
            err.process_err(Status::MappingError, "");
            return Ok(failed_result);
        }
    };

    let session_token = match auth_step {
        AuthStep::SuccessFull { session_user_token } => session_user_token,
        _ => return Ok(auth_step)
    };

    match write_new_user_info_to_device(state, &session_token) {
        Ok(_) => {},
        Err(err) => {
            err.process_err(Status::MappingError, "");
            return Ok(failed_result);
        }
    }

    match init_session(state, session_token.as_ref()).await {
        Ok(_) => Ok(AuthStep::SuccessShort {  }),
        Err(err) => {
            err.process_err(err, "");
            Ok(failed_result)
        }
    }


}
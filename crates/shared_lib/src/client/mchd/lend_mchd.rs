use std::str::FromStr;
use tauri_plugin_fs::FsExt;

use crate::{Status, ClientState, ProcessError};
use crate::service::mchd::service::{MchdInfo, MchdStep, RegisterMchdData};
use crate::service::api_routes::implements::ApiRoutes;

use crate::client::back_api::post_query::post_query_back_api;


pub async fn lend_mchd_to_back_api_for_register(
    state: &ClientState,
    xml_file_path: &String,
    sig_file_path: &String
) -> Result<MchdStep, Status> {

    let failed_result = Ok(MchdStep::TryLater { text: MchdInfo::ClientServiceError });

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

	let tauri_xml_path = tauri_plugin_fs::FilePath::from_str(xml_file_path)
		.map_err(|err| {
			let ext_info = format!("wrong svelte xml_file_path = {:?}", xml_file_path);
			err.process_err(Status::FilePathError, &ext_info)
		})?;

	let xml_file = state.app_handle.fs().read(tauri_xml_path.to_owned())
		.map_err(|err| {
			let ext_info = format!("wrong tauri_xml_path = {:?}", tauri_xml_path);
			err.process_err(Status::FileReadError, &ext_info)
		})?;

	
	let tauri_sig_path = tauri_plugin_fs::FilePath::from_str(sig_file_path)
		.map_err(|err| {
			let ext_info = format!("wrong svelte sig_file_path = {:?}", sig_file_path);
			err.process_err(Status::FilePathError, &ext_info)
		})?;

	let sig_file = state.app_handle.fs().read(tauri_sig_path.to_owned())
		.map_err(|err| {
			let ext_info = format!("wrong tauri_sig_path ={:?}", tauri_sig_path);
			err.process_err(Status::FileReadError, &ext_info)
		})?;

    let user_id = session.session_user.user.user_id.clone();

    let data = RegisterMchdData {
        xml_file,
        sig_file,
        user_id
    };

    let response = match post_query_back_api(
            state, 
            state.config.get_std_client(),
            ApiRoutes::MchdLend,
            &data).await {
        Ok(r) => r, 
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    let mchd_step = match response.json().await {
        Ok(d) => d,
        Err(err) => {
            err.process_err(Status::MappingError, "");
            return failed_result;
        }
    };

    Ok(mchd_step)
}